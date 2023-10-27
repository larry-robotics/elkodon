#[generic_tests::define]
mod service_publish_subscribe {
    use elkodon::global_config::{Config, Entries};
    use elkodon::port::publisher::{LoanError, PublisherCreateError};
    use elkodon::port::subscriber::SubscriberCreateError;
    use elkodon::service::builder::publish_subscribe::PublishSubscribeCreateError;
    use elkodon::service::builder::publish_subscribe::PublishSubscribeOpenError;
    use elkodon::service::port_factory::publisher::UnableToDeliverStrategy;
    use elkodon::service::{service_name::ServiceName, Service};
    use elkodon_bb_container::semantic_string::*;
    use elkodon_bb_posix::unique_system_id::UniqueSystemId;
    use elkodon_bb_testing::assert_that;

    fn generate_name() -> ServiceName {
        let mut service = ServiceName::new(b"service_tests_").unwrap();
        service
            .push_bytes(
                UniqueSystemId::new()
                    .unwrap()
                    .value()
                    .to_string()
                    .as_bytes(),
            )
            .unwrap();
        service
    }

    #[test]
    fn creating_non_existing_service_works<Sut: Service>() {
        let service_name = generate_name();
        let sut = Sut::new(&service_name).publish_subscribe().create::<u64>();

        assert_that!(sut, is_ok);
        let sut = sut.unwrap();
        assert_that!(*sut.name(), eq service_name);
    }

    #[test]
    fn creating_same_service_twice_fails<Sut: Service>() {
        let service_name = generate_name();
        let sut = Sut::new(&service_name).publish_subscribe().create::<u64>();
        assert_that!(sut, is_ok);

        let sut2 = Sut::new(&service_name).publish_subscribe().create::<u64>();
        assert_that!(sut2, is_err);
        assert_that!(
            sut2.err().unwrap(), eq
            PublishSubscribeCreateError::AlreadyExists
        );
    }

    #[test]
    fn recreate_after_drop_works<Sut: Service>() {
        let service_name = generate_name();
        let sut = Sut::new(&service_name).publish_subscribe().create::<u64>();
        assert_that!(sut, is_ok);

        drop(sut);

        let sut2 = Sut::new(&service_name).publish_subscribe().create::<u64>();
        assert_that!(sut2, is_ok);
    }

    #[test]
    fn open_fails_when_service_does_not_exist<Sut: Service>() {
        let service_name = generate_name();
        let sut = Sut::new(&service_name).publish_subscribe().open::<u64>();
        assert_that!(sut, is_err);
        assert_that!(sut.err().unwrap(), eq PublishSubscribeOpenError::DoesNotExist);
    }

    #[test]
    fn open_succeeds_when_service_does_exist<Sut: Service>() {
        let service_name = generate_name();
        let sut = Sut::new(&service_name).publish_subscribe().create::<u64>();
        assert_that!(sut, is_ok);

        let sut2 = Sut::new(&service_name).publish_subscribe().open::<u64>();
        assert_that!(sut2, is_ok);
    }

    #[test]
    fn open_fails_when_service_has_wrong_type<Sut: Service>() {
        let service_name = generate_name();
        let sut = Sut::new(&service_name).publish_subscribe().create::<u64>();
        assert_that!(sut, is_ok);

        let sut2 = Sut::new(&service_name).publish_subscribe().open::<i64>();
        assert_that!(sut2, is_err);
        assert_that!(sut2.err().unwrap(), eq PublishSubscribeOpenError::IncompatibleTypes);
    }

    #[test]
    fn open_fails_when_service_does_not_fulfill_publisher_requirements<Sut: Service>() {
        let service_name = generate_name();
        let sut = Sut::new(&service_name)
            .publish_subscribe()
            .max_publishers(1)
            .create::<u64>();
        assert_that!(sut, is_ok);

        let sut2 = Sut::new(&service_name)
            .publish_subscribe()
            .max_publishers(2)
            .open::<u64>();

        assert_that!(sut2, is_err);
        assert_that!(
            sut2.err().unwrap(), eq
            PublishSubscribeOpenError::DoesNotSupportRequestedAmountOfPublishers
        );
    }

    #[test]
    fn open_does_not_fail_when_service_owner_is_dropped<Sut: Service>() {
        let service_name = generate_name();
        let sut = Sut::new(&service_name).publish_subscribe().create::<u64>();
        assert_that!(sut, is_ok);

        let sut2 = Sut::new(&service_name).publish_subscribe().open::<u64>();
        assert_that!(sut2, is_ok);

        drop(sut);

        let sut3 = Sut::new(&service_name).publish_subscribe().open::<u64>();
        assert_that!(sut3, is_ok);
    }

    #[test]
    fn open_fails_when_all_previous_owners_have_been_dropped<Sut: Service>() {
        let service_name = generate_name();
        let sut = Sut::new(&service_name).publish_subscribe().create::<u64>();
        assert_that!(sut, is_ok);

        let sut2 = Sut::new(&service_name).publish_subscribe().open::<u64>();
        assert_that!(sut2, is_ok);

        drop(sut);
        drop(sut2);

        let sut3 = Sut::new(&service_name).publish_subscribe().open::<u64>();
        assert_that!(sut3, is_err);
        assert_that!(sut3.err().unwrap(), eq PublishSubscribeOpenError::DoesNotExist);
    }

    #[test]
    fn open_or_create_creates_service_if_it_does_not_exist<Sut: Service>() {
        let service_name = generate_name();
        let sut = Sut::new(&service_name)
            .publish_subscribe()
            .open_or_create::<u64>();

        assert_that!(sut, is_ok);
    }

    #[test]
    fn open_or_create_opens_service_if_it_does_exist<Sut: Service>() {
        let service_name = generate_name();
        let _sut = Sut::new(&service_name)
            .publish_subscribe()
            .create::<u64>()
            .unwrap();

        let sut = Sut::new(&service_name)
            .publish_subscribe()
            .open_or_create::<u64>();

        assert_that!(sut, is_ok);
    }

    #[test]
    fn max_publishers_and_subscribers_is_set_to_config_default<Sut: Service>() {
        let service_name = generate_name();
        let sut = Sut::new(&service_name)
            .publish_subscribe()
            .create::<u64>()
            .unwrap();

        let defaults = &Config::get_global_config().get().defaults;

        assert_that!(
            sut.max_supported_publishers(), eq
            defaults.publish_subscribe.max_publishers
        );
        assert_that!(
            sut.max_supported_subscribers(), eq
            defaults.publish_subscribe.max_subscribers
        );
    }

    #[test]
    fn max_publishers_and_subscribers_can_be_modified<Sut: Service>() {
        let service_name = generate_name();
        let sut = Sut::new(&service_name)
            .publish_subscribe()
            .max_publishers(42)
            .max_subscribers(73)
            .create::<u64>()
            .unwrap();

        assert_that!(sut.max_supported_publishers(), eq 42);
        assert_that!(sut.max_supported_subscribers(), eq 73);

        let sut2 = Sut::new(&service_name)
            .publish_subscribe()
            .open::<u64>()
            .unwrap();

        assert_that!(sut2.max_supported_publishers(), eq 42);
        assert_that!(sut2.max_supported_subscribers(), eq 73);
    }

    #[test]
    fn max_publishers_and_subscribers_can_be_modified_via_custom_config<Sut: Service>() {
        let service_name = generate_name();
        let mut entries = Entries::default();
        entries.defaults.publish_subscribe.max_subscribers = 1;
        entries.defaults.publish_subscribe.max_publishers = 1;

        let custom_config = Config::from_entries(&entries);
        let sut = Sut::new(&service_name)
            .publish_subscribe_with_custom_config(&custom_config)
            .create::<u64>()
            .unwrap();

        assert_that!(sut.max_supported_subscribers(), eq 1);
        assert_that!(sut.max_supported_publishers(), eq 1);

        let sut2 = Sut::new(&service_name)
            .publish_subscribe()
            .open::<u64>()
            .unwrap();

        assert_that!(sut2.max_supported_publishers(), eq 1);
        assert_that!(sut2.max_supported_subscribers(), eq 1);
    }

    #[test]
    fn simple_communication_works_subscriber_created_first<Sut: Service>() {
        let service_name = generate_name();

        let sut = Sut::new(&service_name)
            .publish_subscribe()
            .create::<u64>()
            .unwrap();

        let sut2 = Sut::new(&service_name)
            .publish_subscribe()
            .open::<u64>()
            .unwrap();

        let subscriber = sut.subscriber().create().unwrap();
        let publisher = sut2.publisher().create().unwrap();
        assert_that!(subscriber.update_connections(), is_ok);

        assert_that!(publisher.send_copy(1234), is_ok);
        assert_that!(publisher.send_copy(4567), is_ok);

        let result = subscriber.receive().unwrap();
        assert_that!(result, is_some);
        assert_that!(*result.unwrap(), eq 1234);

        let result = subscriber.receive().unwrap();
        assert_that!(result, is_some);
        assert_that!(*result.unwrap(), eq 4567);
    }

    #[test]
    fn simple_communication_works_publisher_created_first<Sut: Service>() {
        let service_name = generate_name();

        let sut = Sut::new(&service_name)
            .publish_subscribe()
            .create::<u64>()
            .unwrap();

        let sut2 = Sut::new(&service_name)
            .publish_subscribe()
            .open::<u64>()
            .unwrap();

        let publisher = sut.publisher().create().unwrap();
        let subscriber = sut2.subscriber().create().unwrap();
        assert_that!(publisher.update_connections(), is_ok);

        assert_that!(publisher.send_copy(1234), is_ok);
        assert_that!(publisher.send_copy(4567), is_ok);

        let result = subscriber.receive().unwrap();
        assert_that!(result, is_some);
        assert_that!(*result.unwrap(), eq 1234);

        let result = subscriber.receive().unwrap();
        assert_that!(result, is_some);
        assert_that!(*result.unwrap(), eq 4567);
    }

    #[test]
    fn communication_with_max_subscribers_and_publishers<Sut: Service>() {
        const MAX_PUB: usize = 4;
        const MAX_SUB: usize = 6;
        const NUMBER_OF_ITERATIONS: u64 = 128;
        let service_name = generate_name();

        let sut = Sut::new(&service_name)
            .publish_subscribe()
            .max_publishers(MAX_PUB)
            .max_subscribers(MAX_SUB)
            .create::<u64>()
            .unwrap();

        let mut publishers = vec![];
        publishers.reserve(MAX_PUB);

        for _ in 0..MAX_PUB {
            publishers.push(sut.publisher().create().unwrap());
        }

        let mut subscribers = vec![];
        subscribers.reserve(MAX_SUB);

        for _ in 0..MAX_SUB {
            subscribers.push(sut.subscriber().create().unwrap());
        }

        let mut counter = 0;
        for _ in 0..NUMBER_OF_ITERATIONS {
            for publisher in &mut publishers {
                assert_that!(publisher.send_copy(counter), is_ok);

                for subscriber in &subscribers {
                    let sample = subscriber.receive().unwrap();
                    assert_that!(sample, is_some);
                    assert_that!(*sample.unwrap(), eq counter);
                }
                counter += 1;
            }
        }
    }

    #[test]
    fn multi_channel_communication_with_max_subscribers_and_publishers<Sut: Service>() {
        const MAX_PUB: usize = 5;
        const MAX_SUB: usize = 7;
        const NUMBER_OF_ITERATIONS: u64 = 128;
        let service_name = generate_name();

        let _sut = Sut::new(&service_name)
            .publish_subscribe()
            .max_publishers(MAX_PUB)
            .max_subscribers(MAX_SUB)
            .create::<u64>()
            .unwrap();

        let mut channels = vec![];
        channels.reserve(MAX_PUB + MAX_SUB);

        for _ in 0..MAX_PUB + MAX_SUB {
            channels.push(
                Sut::new(&service_name)
                    .publish_subscribe()
                    .open::<u64>()
                    .unwrap(),
            );
        }

        let mut publishers = vec![];
        publishers.reserve(MAX_PUB);

        for i in 0..MAX_PUB {
            publishers.push(channels[i].publisher().create().unwrap());
        }

        let mut subscribers = vec![];
        subscribers.reserve(MAX_SUB);

        for i in 0..MAX_SUB {
            subscribers.push(channels[i + MAX_PUB].subscriber().create().unwrap());
        }

        let mut counter = 0;
        for _ in 0..NUMBER_OF_ITERATIONS {
            for publisher in &mut publishers {
                assert_that!(publisher.send_copy(counter), is_ok);

                for subscriber in &subscribers {
                    let sample = subscriber.receive().unwrap();
                    assert_that!(sample, is_some);
                    assert_that!(*sample.unwrap(), eq counter);
                }
                counter += 1;
            }
        }
    }

    #[test]
    fn publish_safely_overflows_when_enabled<Sut: Service>() {
        let service_name = generate_name();
        const BUFFER_SIZE: usize = 2;

        let sut = Sut::new(&service_name)
            .publish_subscribe()
            .enable_safe_overflow(true)
            .subscriber_buffer_size(BUFFER_SIZE)
            .create::<usize>()
            .unwrap();

        let publisher = sut.publisher().create().unwrap();
        let subscriber = sut.subscriber().create().unwrap();

        for i in 0..BUFFER_SIZE {
            assert_that!(publisher.send_copy(i), is_ok);
        }

        for i in 0..BUFFER_SIZE {
            assert_that!(publisher.send_copy(2 * i + 25), is_ok);
        }

        for i in 0..BUFFER_SIZE {
            let sample = subscriber.receive().unwrap().unwrap();
            assert_that!(*sample, eq 2 * i + 25);
        }
    }

    #[test]
    fn publish_does_not_overflow_when_deactivated<Sut: Service>() {
        let service_name = generate_name();
        const BUFFER_SIZE: usize = 5;

        let sut = Sut::new(&service_name)
            .publish_subscribe()
            .enable_safe_overflow(false)
            .subscriber_buffer_size(BUFFER_SIZE)
            .create::<usize>()
            .unwrap();

        let publisher = sut
            .publisher()
            .unable_to_deliver_strategy(UnableToDeliverStrategy::DiscardSample)
            .create()
            .unwrap();
        let subscriber = sut.subscriber().create().unwrap();

        for i in 0..BUFFER_SIZE {
            assert_that!(publisher.send_copy(i), is_ok);
        }

        for i in 0..BUFFER_SIZE {
            assert_that!(publisher.send_copy(2 * i + 25), is_ok);
        }

        for i in 0..BUFFER_SIZE {
            let sample = subscriber.receive().unwrap().unwrap();
            assert_that!(*sample, eq i);
        }
    }

    #[test]
    fn publish_non_overflow_with_greater_history_than_buffer_fails<Sut: Service>() {
        let service_name = generate_name();

        let sut = Sut::new(&service_name)
            .publish_subscribe()
            .enable_safe_overflow(false)
            .history_size(12)
            .subscriber_buffer_size(11)
            .create::<usize>();

        assert_that!(sut, is_err);
        assert_that!(
            sut.err().unwrap(), eq
            PublishSubscribeCreateError::SubscriberBufferMustBeLargerThanHistorySize
        );
    }

    #[test]
    fn publish_history_is_delivered_on_subscription<Sut: Service>() {
        const BUFFER_SIZE: usize = 2;
        let service_name = generate_name();

        let sut = Sut::new(&service_name)
            .publish_subscribe()
            .history_size(3)
            .subscriber_buffer_size(BUFFER_SIZE)
            .create::<usize>()
            .unwrap();

        let sut_publisher = sut.publisher().create().unwrap();
        assert_that!(sut_publisher.send_copy(29), is_ok);
        assert_that!(sut_publisher.send_copy(32), is_ok);
        assert_that!(sut_publisher.send_copy(35), is_ok);

        let sut_subscriber = sut.subscriber().create().unwrap();
        assert_that!(sut_publisher.update_connections(), is_ok);

        for i in 0..BUFFER_SIZE {
            let data = sut_subscriber.receive().unwrap();
            assert_that!(data, is_some);
            assert_that!(*data.unwrap(), eq 29 + (i + 1) * 3 )
        }
    }

    #[test]
    fn publish_history_of_zero_works<Sut: Service>() {
        const BUFFER_SIZE: usize = 2;
        let service_name = generate_name();

        let sut = Sut::new(&service_name)
            .publish_subscribe()
            .history_size(0)
            .subscriber_buffer_size(BUFFER_SIZE)
            .create::<usize>()
            .unwrap();

        let sut_publisher = sut.publisher().create().unwrap();
        assert_that!(sut_publisher.send_copy(29), is_ok);

        let sut_subscriber = sut.subscriber().create().unwrap();
        assert_that!(sut_publisher.update_connections(), is_ok);

        let data = sut_subscriber.receive().unwrap();
        assert_that!(data, is_none);
    }

    #[test]
    fn publish_send_copy_with_huge_overflow_works<Sut: Service>() {
        let service_name = generate_name();
        const BUFFER_SIZE: usize = 5;

        let sut = Sut::new(&service_name)
            .publish_subscribe()
            .max_publishers(1)
            .max_subscribers(2)
            .history_size(0)
            .subscriber_buffer_size(BUFFER_SIZE)
            .subscriber_max_borrowed_samples(1)
            .create::<usize>()
            .unwrap();

        let sut_publisher = sut.publisher().max_loaned_samples(1).create().unwrap();

        let mut subscribers = vec![];
        for _ in 0..2 {
            let sut_subscriber = sut.subscriber().create();
            subscribers.push(sut_subscriber);
        }

        for _ in 0..BUFFER_SIZE * 100 {
            assert_that!(sut_publisher.send_copy(889), is_ok);
        }
    }

    fn publisher_never_goes_out_of_memory_impl<Sut: Service>(
        buffer_size: usize,
        history_size: usize,
        max_borrow: usize,
        max_subscribers: usize,
        max_loan: usize,
    ) {
        const ITERATIONS: usize = 16;
        let service_name = generate_name();

        let sut = Sut::new(&service_name)
            .publish_subscribe()
            .max_publishers(1)
            .max_subscribers(max_subscribers)
            .enable_safe_overflow(true)
            .history_size(history_size)
            .subscriber_buffer_size(buffer_size)
            .subscriber_max_borrowed_samples(max_borrow)
            .create::<usize>()
            .unwrap();

        let sut_publisher = sut
            .publisher()
            .max_loaned_samples(max_loan)
            .create()
            .unwrap();

        let mut subscribers = vec![];
        for _ in 0..max_subscribers {
            let sut_subscriber = sut.subscriber().create().unwrap();
            subscribers.push(sut_subscriber);
        }

        for _ in 0..ITERATIONS {
            // max out borrow
            let mut borrowed_samples = vec![];
            while borrowed_samples.len() < max_borrow * max_subscribers {
                for _ in 0..buffer_size {
                    assert_that!(sut_publisher.send_copy(889), is_ok);
                }

                for i in 0..max_subscribers {
                    while let Ok(Some(sample)) = subscribers[i].receive() {
                        borrowed_samples.push((i, sample));
                    }
                }
            }

            // max out buffer
            for _ in 0..buffer_size {
                assert_that!(sut_publisher.send_copy(8127), is_ok);
            }

            // max out history
            for _ in 0..history_size {
                assert_that!(sut_publisher.send_copy(17283), is_ok);
            }

            // max out loan
            let mut loaned_samples = vec![];
            for _ in 0..max_loan {
                let sample = sut_publisher.loan();
                assert_that!(sample, is_ok);
                loaned_samples.push(sample);
            }

            let sample = sut_publisher.loan();
            assert_that!(sample, is_err);
            assert_that!(sample.err().unwrap(), eq LoanError::ExceedsMaxLoanedChunks);

            // cleanup
            borrowed_samples.clear();
            loaned_samples.clear();
            for i in 0..max_subscribers {
                while let Ok(Some(_)) = subscribers[i].receive() {}
            }
        }
    }

    #[test]
    fn publisher_never_goes_out_of_memory_with_huge_max_loan<Sut: Service>() {
        const BUFFER_SIZE: usize = 1;
        const HISTORY_SIZE: usize = 0;
        const MAX_BORROW: usize = 1;
        const MAX_SUBSCRIBERS: usize = 1;
        const MAX_LOAN: usize = 100;

        publisher_never_goes_out_of_memory_impl::<Sut>(
            BUFFER_SIZE,
            HISTORY_SIZE,
            MAX_BORROW,
            MAX_SUBSCRIBERS,
            MAX_LOAN,
        );
    }

    #[test]
    fn publisher_never_goes_out_of_memory_with_huge_max_subscribers<Sut: Service>() {
        const BUFFER_SIZE: usize = 1;
        const HISTORY_SIZE: usize = 0;
        const MAX_BORROW: usize = 1;
        const MAX_SUBSCRIBERS: usize = 100;
        const MAX_LOAN: usize = 1;

        publisher_never_goes_out_of_memory_impl::<Sut>(
            BUFFER_SIZE,
            HISTORY_SIZE,
            MAX_BORROW,
            MAX_SUBSCRIBERS,
            MAX_LOAN,
        );
    }

    #[test]
    fn publisher_never_goes_out_of_memory_with_huge_max_borrow<Sut: Service>() {
        const BUFFER_SIZE: usize = 1;
        const HISTORY_SIZE: usize = 0;
        const MAX_BORROW: usize = 100;
        const MAX_SUBSCRIBERS: usize = 1;
        const MAX_LOAN: usize = 1;

        publisher_never_goes_out_of_memory_impl::<Sut>(
            BUFFER_SIZE,
            HISTORY_SIZE,
            MAX_BORROW,
            MAX_SUBSCRIBERS,
            MAX_LOAN,
        );
    }

    #[test]
    fn publisher_never_goes_out_of_memory_with_huge_history_size<Sut: Service>() {
        const BUFFER_SIZE: usize = 1;
        const HISTORY_SIZE: usize = 100;
        const MAX_BORROW: usize = 1;
        const MAX_SUBSCRIBERS: usize = 1;
        const MAX_LOAN: usize = 1;

        publisher_never_goes_out_of_memory_impl::<Sut>(
            BUFFER_SIZE,
            HISTORY_SIZE,
            MAX_BORROW,
            MAX_SUBSCRIBERS,
            MAX_LOAN,
        );
    }

    #[test]
    fn publisher_never_goes_out_of_memory_with_huge_buffer_size<Sut: Service>() {
        const BUFFER_SIZE: usize = 3;
        const HISTORY_SIZE: usize = 0;
        const MAX_BORROW: usize = 1;
        const MAX_SUBSCRIBERS: usize = 1;
        const MAX_LOAN: usize = 1;

        publisher_never_goes_out_of_memory_impl::<Sut>(
            BUFFER_SIZE,
            HISTORY_SIZE,
            MAX_BORROW,
            MAX_SUBSCRIBERS,
            MAX_LOAN,
        );
    }

    #[test]
    fn publisher_never_goes_out_of_memory_with_huge_values<Sut: Service>() {
        const BUFFER_SIZE: usize = 129;
        const HISTORY_SIZE: usize = 131;
        const MAX_BORROW: usize = 112;
        const MAX_SUBSCRIBERS: usize = 123;
        const MAX_LOAN: usize = 135;

        publisher_never_goes_out_of_memory_impl::<Sut>(
            BUFFER_SIZE,
            HISTORY_SIZE,
            MAX_BORROW,
            MAX_SUBSCRIBERS,
            MAX_LOAN,
        );
    }

    #[test]
    fn creating_max_supported_amount_of_ports_work<Sut: Service>() {
        const MAX_PUBLISHERS: usize = 4;
        const MAX_SUBSCRIBERS: usize = 8;

        let service_name = generate_name();
        let sut = Sut::new(&service_name)
            .publish_subscribe()
            .max_publishers(MAX_PUBLISHERS)
            .max_subscribers(MAX_SUBSCRIBERS)
            .create::<u64>()
            .unwrap();

        let mut publishers = vec![];
        let mut subscribers = vec![];

        // acquire all possible ports
        for _ in 0..MAX_PUBLISHERS {
            let publisher = sut.publisher().create();
            assert_that!(publisher, is_ok);
            publishers.push(publisher);
        }

        for _ in 0..MAX_SUBSCRIBERS {
            let subscriber = sut.subscriber().create();
            assert_that!(subscriber, is_ok);
            subscribers.push(subscriber);
        }

        // create additional ports and fail
        let publisher = sut.publisher().create();
        assert_that!(publisher, is_err);
        assert_that!(
            publisher.err().unwrap(), eq
            PublisherCreateError::ExceedsMaxSupportedPublishers
        );

        let subscriber = sut.subscriber().create();
        assert_that!(subscriber, is_err);
        assert_that!(
            subscriber.err().unwrap(), eq
            SubscriberCreateError::ExceedsMaxSupportedSubscribers
        );

        // remove a publisher and subscriber
        assert_that!(publishers.remove(0), is_ok);
        assert_that!(subscribers.remove(0), is_ok);

        // create additional ports shall work again
        let publisher = sut.publisher().create();
        assert_that!(publisher, is_ok);

        let subscriber = sut.subscriber().create();
        assert_that!(subscriber, is_ok);
    }

    #[test]
    fn set_max_publishers_to_zero_adjusts_it_to_one<Sut: Service>() {
        let service_name = generate_name();
        let sut = Sut::new(&service_name)
            .publish_subscribe()
            .max_publishers(0)
            .create::<u64>()
            .unwrap();

        assert_that!(sut.max_supported_publishers(), eq 1);
    }

    #[test]
    fn set_max_subscribers_to_zero_adjusts_it_to_one<Sut: Service>() {
        let service_name = generate_name();
        let sut = Sut::new(&service_name)
            .publish_subscribe()
            .max_subscribers(0)
            .create::<u64>()
            .unwrap();

        assert_that!(sut.max_supported_subscribers(), eq 1);
    }

    #[test]
    fn set_subscriber_max_borrowed_samples_to_zero_adjusts_it_to_one<Sut: Service>() {
        let service_name = generate_name();
        let sut = Sut::new(&service_name)
            .publish_subscribe()
            .subscriber_max_borrowed_samples(0)
            .create::<u64>()
            .unwrap();

        assert_that!(sut.subscriber_max_borrowed_samples(), eq 1);
    }

    #[test]
    fn set_buffer_size_to_zero_adjusts_it_to_one<Sut: Service>() {
        let service_name = generate_name();
        let sut = Sut::new(&service_name)
            .publish_subscribe()
            .subscriber_buffer_size(0)
            .create::<u64>()
            .unwrap();

        assert_that!(sut.subscriber_buffer_size(), eq 1);
    }

    #[instantiate_tests(<elkodon::service::zero_copy::Service>)]
    mod zero_copy {}

    #[instantiate_tests(<elkodon::service::process_local::Service>)]
    mod process_local {}
}
