use std::cell::UnsafeCell;
use std::fmt::Debug;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::{alloc::Layout, marker::PhantomData, mem::MaybeUninit, ptr::NonNull};

use super::port_identifiers::{UniquePublisherId, UniqueSubscriberId};
use crate::message::Message;
use crate::port::details::subscriber_connections::*;
use crate::port::{DegrationAction, DegrationCallback};
use crate::service;
use crate::service::header::publish_subscribe::Header;
use crate::service::port_factory::publisher::{LocalPublisherConfig, UnableToDeliverStrategy};
use crate::service::static_config::publish_subscribe;
use crate::{global_config, sample_mut::SampleMut};
use elkodon_bb_container::queue::Queue;
use elkodon_bb_container::semantic_string::SemanticString;
use elkodon_bb_elementary::allocator::AllocationError;
use elkodon_bb_elementary::enum_gen;
use elkodon_bb_lock_free::mpmc::container::ContainerState;
use elkodon_bb_lock_free::mpmc::unique_index_set::UniqueIndex;
use elkodon_bb_log::{fail, fatal_panic, warn};
use elkodon_bb_system_types::file_name::FileName;
use elkodon_cal::dynamic_storage::DynamicStorage;
use elkodon_cal::named_concept::{
    NamedConceptBuilder, NamedConceptConfiguration, NamedConceptMgmt,
};
use elkodon_cal::shared_memory::{SharedMemory, SharedMemoryBuilder, SharedMemoryCreateError};
use elkodon_cal::shm_allocator::pool_allocator::PoolAllocator;
use elkodon_cal::shm_allocator::{self, PointerOffset, ShmAllocationError};
use elkodon_cal::zero_copy_connection::{
    ZeroCopyConnection, ZeroCopyCreationError, ZeroCopySendError, ZeroCopySender,
};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum PublisherCreateError {
    ExceedsMaxSupportedPublishers,
    UnableToCreateDataSegment,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum LoanError {
    OutOfMemory,
    ExceedsMaxLoanedChunks,
    InternalFailure,
}

enum_gen! { SendCopyError
  mapping:
    LoanError to LoanError,
    ZeroCopyCreationError to ConnectionError
}

pub(crate) fn data_segment_name(publisher_id: UniquePublisherId) -> FileName {
    let msg = "The system does not support the required file name length for the publishers data segment.";
    let origin = "data_segment_name()";

    let mut file = fatal_panic!(from origin, when FileName::new(publisher_id.0.pid().to_string().as_bytes()), "{}", msg);
    fatal_panic!(from origin, when file.push(b'_'), "{}", msg);
    fatal_panic!(from origin, when file.push_bytes(publisher_id.0.value().to_string().as_bytes()), "{}", msg);
    file
}

pub(crate) fn data_segment_config<'global_config, Service: service::Details<'global_config>>(
    global_config: &global_config::Entries,
) -> <Service::SharedMemory as NamedConceptMgmt>::Configuration {
    let origin = "data_segment_config()";

    let f = match FileName::new(
        global_config
            .global
            .service
            .publisher_data_segment_suffix
            .as_bytes(),
    ) {
        Err(_) => {
            fatal_panic!(from origin, "The publisher_data_segment_suffix \"{}\" provided by the config contains either invalid file name characters or is too long.",
                                       global_config.global.service.publisher_data_segment_suffix);
        }
        Ok(v) => v,
    };

    <Service::SharedMemory as NamedConceptMgmt>::Configuration::default().suffix(f)
}

#[derive(Debug)]
pub struct Publisher<
    'a,
    'global_config: 'a,
    Service: service::Details<'global_config>,
    MessageType: Debug,
> {
    port_id: UniquePublisherId,
    pub(crate) sample_reference_counter: Vec<AtomicU64>,
    pub(crate) data_segment: Service::SharedMemory,
    config: LocalPublisherConfig,

    subscriber_connections: SubscriberConnections<'global_config, Service>,
    subscriber_list_state: UnsafeCell<ContainerState<'a, UniqueSubscriberId>>,
    history: Option<UnsafeCell<Queue<usize>>>,
    service: &'a Service,
    degration_callback: Option<DegrationCallback<'a>>,
    pub(crate) loan_counter: AtomicUsize,
    _dynamic_config_guard: UniqueIndex<'a>,
    _phantom_message_type: PhantomData<MessageType>,
}

impl<'a, 'global_config: 'a, Service: service::Details<'global_config>, MessageType: Debug>
    Publisher<'a, 'global_config, Service, MessageType>
{
    pub(crate) fn new(
        service: &'a Service,
        static_config: &publish_subscribe::StaticConfig,
        config: &LocalPublisherConfig,
    ) -> Result<Self, PublisherCreateError> {
        let msg = "Unable to create Publisher port";
        let origin = "Publisher::new()";
        let port_id = UniquePublisherId::new();
        let subscriber_list = &service
            .state()
            .dynamic_storage
            .get()
            .publish_subscribe()
            .subscribers;

        let number_of_samples = service
            .state()
            .static_config
            .messaging_pattern
            .required_amount_of_samples_per_data_segment(config.max_loaned_samples);

        let data_segment = fail!(from origin, when Self::create_data_segment(port_id, service.state().global_config, number_of_samples),
                with PublisherCreateError::UnableToCreateDataSegment,
                "{} since the data segment could not be acquired.", msg);

        // !MUST! be the last task otherwise a publisher is added to the dynamic config without the
        // creation of all required resources
        let _dynamic_config_guard = match service
            .state()
            .dynamic_storage
            .get()
            .publish_subscribe()
            .add_publisher_id(port_id)
        {
            Some(unique_index) => unique_index,
            None => {
                fail!(from origin, with PublisherCreateError::ExceedsMaxSupportedPublishers,
                            "{} since it would exceed the maximum supported amount of publishers of {}.",
                            msg, service.state().static_config.publish_subscribe().max_publishers);
            }
        };

        let new_self = Self {
            port_id,
            subscriber_connections: SubscriberConnections::new(
                subscriber_list.capacity(),
                service.state().global_config,
                port_id,
                static_config,
            ),
            data_segment,
            config: *config,
            sample_reference_counter: {
                let mut v = Vec::with_capacity(number_of_samples);
                for _ in 0..number_of_samples {
                    v.push(AtomicU64::new(0));
                }
                v
            },
            subscriber_list_state: unsafe { UnsafeCell::new(subscriber_list.get_state()) },
            history: match static_config.history_size == 0 {
                true => None,
                false => Some(UnsafeCell::new(Queue::new(static_config.history_size))),
            },
            service,
            degration_callback: None,
            loan_counter: AtomicUsize::new(0),
            _dynamic_config_guard,
            _phantom_message_type: PhantomData,
        };

        if let Err(e) = new_self.populate_subscriber_channels() {
            warn!(from new_self, "The new Publisher port is unable to connect to every Subscriber port, caused by {:?}.", e);
        }

        Ok(new_self)
    }

    fn populate_subscriber_channels(&self) -> Result<(), ZeroCopyCreationError> {
        let mut visited_indices = vec![];
        visited_indices.resize(self.subscriber_connections.capacity(), None);

        unsafe {
            (*self.subscriber_list_state.get()).for_each(|index, subscriber_id| {
                visited_indices[index as usize] = Some(*subscriber_id);
            })
        };

        // retrieve samples before destroying channel
        self.retrieve_returned_samples();

        for (i, index) in visited_indices.iter().enumerate() {
            match index {
                Some(subscriber_id) => {
                    match self.subscriber_connections.create(i, *subscriber_id) {
                        Ok(false) => (),
                        Ok(true) => match &self.subscriber_connections.get(i) {
                            Some(connection) => self.deliver_history(connection),
                            None => {
                                fatal_panic!(from self, "This should never happen! Unable to acquire previously created subscriber connection.")
                            }
                        },
                        Err(e) => match &self.degration_callback {
                            Some(c) => match c.call(
                                self.service.state().static_config.clone(),
                                self.port_id,
                                *subscriber_id,
                            ) {
                                DegrationAction::Ignore => (),
                                DegrationAction::Warn => {
                                    warn!(from self, "Unable to establish connection to new subscriber {:?}.", subscriber_id )
                                }
                                DegrationAction::Fail => {
                                    fail!(from self, with e,
                                           "Unable to establish connection to new subscriber {:?}.", subscriber_id );
                                }
                            },
                            None => {
                                warn!(from self, "Unable to establish connection to new subscriber {:?}.", subscriber_id )
                            }
                        },
                    }
                }
                None => self.subscriber_connections.remove(i),
            }
        }

        Ok(())
    }

    fn deliver_history(&self, connection: &Connection<'global_config, Service>) {
        match &self.history {
            None => (),
            Some(history) => {
                let history = unsafe { &mut *history.get() };
                for i in 0..history.len() {
                    let ptr_distance = unsafe { history.get_unchecked(i) };

                    match connection.sender.try_send(PointerOffset::new(ptr_distance)) {
                        Ok(_) => {
                            self.sample_reference_counter[Self::sample_index(ptr_distance)]
                                .fetch_add(1, Ordering::Relaxed);
                        }
                        Err(e) => {
                            warn!(from self, "Failed to deliver history to new subscriber via {:?} due to {:?}", connection, e);
                        }
                    }
                }
            }
        }
    }

    fn sample_index(distance_to_chunk: usize) -> usize {
        distance_to_chunk / std::mem::size_of::<Message<Header, MessageType>>()
    }

    fn create_data_segment(
        port_id: UniquePublisherId,
        global_config: &'global_config global_config::Entries,
        number_of_samples: usize,
    ) -> Result<Service::SharedMemory, SharedMemoryCreateError> {
        let allocator_config = shm_allocator::pool_allocator::Config {
            bucket_layout: Layout::new::<Message<Header, MessageType>>(),
        };
        let chunk_size = allocator_config.bucket_layout.size();
        let chunk_align = allocator_config.bucket_layout.align();

        Ok(fail!(from "Publisher::create_data_segment()",
            when <<Service::SharedMemory as SharedMemory<PoolAllocator>>::Builder as NamedConceptBuilder<
            Service::SharedMemory,
                >>::new(&data_segment_name(port_id))
                .config(&data_segment_config::<Service>(global_config))
                .size(chunk_size * number_of_samples + chunk_align - 1)
                .create(&allocator_config),
            "Unable to create the data segment."))
    }

    fn send_impl(&self, address_to_chunk: usize) -> Result<usize, ZeroCopyCreationError> {
        fail!(from self, when self.update_connections(),
            "Unable to send sample since the connections could not be updated.");

        self.add_to_history(address_to_chunk);
        Ok(self.deliver_sample(address_to_chunk))
    }

    fn add_to_history(&self, address_to_chunk: usize) {
        match &self.history {
            None => (),
            Some(history) => {
                let history = unsafe { &mut *history.get() };
                self.sample_reference_counter[Self::sample_index(address_to_chunk)]
                    .fetch_add(1, Ordering::Relaxed);
                match unsafe { history.push_with_overflow(address_to_chunk) } {
                    None => (),
                    Some(old) => self.release_sample(PointerOffset::new(old)),
                }
            }
        }
    }

    fn deliver_sample(&self, address_to_chunk: usize) -> usize {
        let deliver_call = match self.config.unable_to_deliver_strategy {
            UnableToDeliverStrategy::Block => <<Service as service::Details<'global_config>>::Connection as ZeroCopyConnection>::Sender::blocking_send,
            UnableToDeliverStrategy::DiscardSample => <<Service as service::Details<'global_config>>::Connection as ZeroCopyConnection>::Sender::try_send,
        };

        let mut number_of_recipients = 0;
        for i in 0..self.subscriber_connections.len() {
            match self.subscriber_connections.get(i) {
                Some(ref connection) => {
                    match deliver_call(&connection.sender, PointerOffset::new(address_to_chunk)) {
                        Err(ZeroCopySendError::ReceiveBufferFull) => {
                            /* causes no problem
                             *   blocking_send => can never happen
                             *   try_send => we tried and expect that the buffer is full
                             * */
                        }
                        Err(ZeroCopySendError::ClearRetrieveChannelBeforeSend) => {
                            warn!(from self, "Unable to send sample via connection {:?} since the retrieve buffer is full. This can be caused by a corrupted retrieve channel.", connection);
                        }
                        Ok(overflow) => {
                            self.sample_reference_counter[Self::sample_index(address_to_chunk)]
                                .fetch_add(1, Ordering::Relaxed);
                            number_of_recipients += 1;

                            if let Some(old) = overflow {
                                self.release_sample(old)
                            }
                        }
                    }
                }
                None => (),
            }
        }
        number_of_recipients
    }

    pub(crate) fn release_sample(&self, distance_to_chunk: PointerOffset) {
        if self.sample_reference_counter[Self::sample_index(distance_to_chunk.value())]
            .fetch_sub(1, Ordering::Relaxed)
            == 1
        {
            unsafe {
                fatal_panic!(from self, when self.data_segment
                .deallocate(
                    distance_to_chunk,
                    Layout::new::<MessageType>(),
                ), "Internal logic error. The sample should always contain a valid memory chunk from the provided allocator.");
            };
        }
    }

    fn retrieve_returned_samples(&self) {
        for i in 0..self.subscriber_connections.len() {
            match self.subscriber_connections.get(i) {
                Some(ref connection) => loop {
                    match connection.sender.reclaim() {
                        Ok(Some(ptr_dist)) => {
                            let sample_index = Self::sample_index(ptr_dist.value());

                            if self.sample_reference_counter[sample_index]
                                .fetch_sub(1, Ordering::Relaxed)
                                == 1
                            {
                                unsafe {
                                    fatal_panic!(from self, when self.data_segment
                                    .deallocate(
                                        ptr_dist,
                                        Layout::new::<Message<Header, MessageType>>(),
                                    ), "This should never happen! Failed to deallocate the reclaimed ptr. Either the data was corrupted or an invalid ptr was returned.")
                                };
                            }
                        }
                        Ok(None) => break,
                        Err(e) => {
                            warn!(from self, "Unable to reclaim samples from connection {:?} due to {:?}. This may lead to a situation where no more samples will be delivered to this connection.", connection, e)
                        }
                    }
                },
                None => (),
            }
        }
    }

    pub fn set_degration_callback<
        F: Fn(
                service::static_config::StaticConfig,
                UniquePublisherId,
                UniqueSubscriberId,
            ) -> DegrationAction
            + 'a,
    >(
        &mut self,
        callback: Option<F>,
    ) {
        match callback {
            Some(c) => self.degration_callback = Some(DegrationCallback::new(c)),
            None => self.degration_callback = None,
        }
    }

    pub fn update_connections(&self) -> Result<(), ZeroCopyCreationError> {
        if unsafe { (*self.subscriber_list_state.get()).update() } {
            fail!(from self, when self.populate_subscriber_channels(),
                "Connections were updated only partially since at least one connection to a Subscriber port failed.");
        }

        Ok(())
    }

    pub fn number_of_subscribers(&self) -> usize {
        self.subscriber_connections.number_of_subscribers()
    }

    pub fn send<'publisher>(
        &'publisher self,
        sample: SampleMut<'a, 'publisher, 'global_config, Service, Header, MessageType>,
    ) -> Result<usize, ZeroCopyCreationError> {
        Ok(
            fail!(from self, when self.send_impl(sample.offset_to_chunk().value()),
            "Unable to send sample since the underlying send failed."),
        )
    }

    pub fn send_copy(&self, value: MessageType) -> Result<usize, SendCopyError> {
        let msg = "Unable to send copy of message";
        let mut sample = fail!(from self, when self.loan(),
                                    "{} since the loan of a sample failed.", msg);

        unsafe { sample.as_mut_ptr().write(value) };
        Ok(
            fail!(from self, when self.send_impl(sample.offset_to_chunk().value()),
            "{} since the underlying send operation failed.", msg),
        )
    }

    pub fn loan<'publisher>(
        &'publisher self,
    ) -> Result<SampleMut<'a, 'publisher, 'global_config, Service, Header, MessageType>, LoanError>
    {
        self.retrieve_returned_samples();
        let msg = "Unable to loan Sample";

        if self.loan_counter.load(Ordering::Relaxed) >= self.config.max_loaned_samples {
            fail!(from self, with LoanError::ExceedsMaxLoanedChunks,
                "{} since already {} samples were loaned and it would exceed the maximum of parallel loans of {}. Release or send a loaned sample to loan another sample.",
                msg, self.loan_counter.load(Ordering::Relaxed), self.config.max_loaned_samples);
        }

        match self
            .data_segment
            .allocate(Layout::new::<Message<Header, MessageType>>())
        {
            Ok(chunk) => {
                if self.sample_reference_counter[Self::sample_index(chunk.offset.value())]
                    .fetch_add(1, Ordering::Relaxed)
                    != 0
                {
                    fatal_panic!(from self,
                                "{} since the allocated sample is already in use! This should never happen!", msg);
                }

                let mut chunk_ptr;
                unsafe {
                    chunk_ptr = NonNull::new_unchecked(
                        chunk.data_ptr as *mut MaybeUninit<Message<Header, MessageType>>,
                    );
                    let header_ptr =
                        std::ptr::addr_of_mut!((*chunk_ptr.as_mut().as_mut_ptr()).header);
                    header_ptr.write(Header::new(self.port_id))
                }

                Ok(SampleMut::new(self, chunk_ptr, chunk.offset))
            }
            Err(ShmAllocationError::AllocationError(AllocationError::OutOfMemory)) => {
                fail!(from self, with LoanError::OutOfMemory,
                    "{} since the underlying shared memory is out of memory.", msg);
            }
            Err(ShmAllocationError::AllocationError(AllocationError::SizeTooLarge))
            | Err(ShmAllocationError::AllocationError(AllocationError::AlignmentFailure)) => {
                fatal_panic!(from self, "{} since the system seems to be corrupted.", msg);
            }
            Err(v) => {
                fail!(from self, with LoanError::InternalFailure,
                    "{} since an internal failure occurred ({:?}).", msg, v);
            }
        }
    }
}
