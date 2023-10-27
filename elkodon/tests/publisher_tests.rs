#[generic_tests::define]
mod publisher {
    use std::time::{Duration, Instant};

    use elkodon::port::publisher::LoanError;
    use elkodon::service::port_factory::publisher::UnableToDeliverStrategy;
    use elkodon::service::{service_name::ServiceName, Service};
    use elkodon_bb_container::semantic_string::*;
    use elkodon_bb_posix::barrier::{BarrierBuilder, BarrierHandle};
    use elkodon_bb_posix::unique_system_id::UniqueSystemId;
    use elkodon_bb_testing::assert_that;

    const TIMEOUT: Duration = Duration::from_millis(25);

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
    fn number_of_subscriber_works<Sut: Service>() {
        let service_name = generate_name();
        let service = Sut::new(&service_name)
            .publish_subscribe()
            .create::<u64>()
            .unwrap();

        let sut = service.publisher().create().unwrap();
        assert_that!(sut.number_of_subscribers(), eq 0);

        let _subscriber = service.subscriber().create();
        assert_that!(sut.number_of_subscribers(), eq 0);
        assert_that!(sut.update_connections(), is_ok);
        assert_that!(sut.number_of_subscribers(), eq 1);
    }

    #[test]
    fn publisher_can_borrow_multiple_sample_at_once<Sut: Service>() {
        let service_name = generate_name();
        let service = Sut::new(&service_name)
            .publish_subscribe()
            .create::<u64>()
            .unwrap();

        let sut = service.publisher().max_loaned_samples(4).create().unwrap();

        let mut sample1 = sut.loan().unwrap();
        let mut sample2 = sut.loan().unwrap();
        let mut sample3 = sut.loan().unwrap();

        unsafe { *sample1.as_mut_ptr() = 1 };
        unsafe { *sample2.as_mut_ptr() = 2 };
        unsafe { *sample3.as_mut_ptr() = 3 };

        let subscriber = service.subscriber().create().unwrap();

        assert_that!(sut.send_copy(4), is_ok);
        assert_that!(sut.send(sample3), is_ok);
        drop(sample2);
        drop(sample1);

        let r = subscriber.receive().unwrap();
        assert_that!(r, is_some);
        assert_that!(unsafe { *r.unwrap().as_ptr() }, eq 4);
        let r = subscriber.receive().unwrap();
        assert_that!(r, is_some);
        assert_that!(unsafe { *r.unwrap().as_ptr() }, eq 3);
    }

    #[test]
    fn publisher_max_loaned_samples_works<Sut: Service>() {
        let service_name = generate_name();
        let service = Sut::new(&service_name)
            .publish_subscribe()
            .create::<u64>()
            .unwrap();

        let sut = service.publisher().max_loaned_samples(2).create().unwrap();

        let _sample1 = sut.loan().unwrap();
        let _sample2 = sut.loan().unwrap();

        let sample3 = sut.loan();
        assert_that!(sample3, is_err);
        assert_that!(sample3.err().unwrap(), eq LoanError::ExceedsMaxLoanedChunks);
    }

    #[test]
    fn publisher_sending_sample_reduces_loan_counter<Sut: Service>() {
        let service_name = generate_name();
        let service = Sut::new(&service_name)
            .publish_subscribe()
            .create::<u64>()
            .unwrap();

        let sut = service.publisher().max_loaned_samples(2).create().unwrap();

        let _sample1 = sut.loan().unwrap();
        let sample2 = sut.loan().unwrap();

        assert_that!(sut.send(sample2), is_ok);

        let _sample3 = sut.loan();
        let sample4 = sut.loan();
        assert_that!(sample4, is_err);
        assert_that!(sample4.err().unwrap(), eq LoanError::ExceedsMaxLoanedChunks);
    }

    #[test]
    fn publisher_dropping_sample_reduces_loan_counter<Sut: Service>() {
        let service_name = generate_name();
        let service = Sut::new(&service_name)
            .publish_subscribe()
            .create::<u64>()
            .unwrap();

        let sut = service.publisher().max_loaned_samples(2).create().unwrap();

        let _sample1 = sut.loan().unwrap();
        let sample2 = sut.loan().unwrap();

        drop(sample2);

        let _sample3 = sut.loan();
        let sample4 = sut.loan();
        assert_that!(sample4, is_err);
        assert_that!(sample4.err().unwrap(), eq LoanError::ExceedsMaxLoanedChunks);
    }

    #[test]
    fn publisher_block_when_unable_to_deliver_blocks<Sut: Service>() {
        let service_name = generate_name();
        let service = Sut::new(&service_name)
            .publish_subscribe()
            .subscriber_buffer_size(1)
            .enable_safe_overflow(false)
            .create::<u64>()
            .unwrap();

        let sut = service
            .publisher()
            .unable_to_deliver_strategy(UnableToDeliverStrategy::Block)
            .create()
            .unwrap();

        let handle = BarrierHandle::new();
        let barrier = BarrierBuilder::new(2).create(&handle).unwrap();

        std::thread::scope(|s| {
            s.spawn(|| {
                let service = Sut::new(&service_name)
                    .publish_subscribe()
                    .subscriber_buffer_size(1)
                    .open::<u64>()
                    .unwrap();

                let subscriber = service.subscriber().create().unwrap();
                barrier.wait();
                std::thread::sleep(TIMEOUT);
                let sample = subscriber.receive().unwrap().unwrap();
                assert_that!(*sample, eq 8192);
                std::thread::sleep(TIMEOUT);
                let sample = subscriber.receive().unwrap().unwrap();
                assert_that!(*sample, eq 2);
            });

            barrier.wait();
            let now = Instant::now();
            sut.send_copy(8192).unwrap();
            sut.send_copy(2).unwrap();
            assert_that!(now.elapsed(), ge TIMEOUT);
        });
    }

    #[instantiate_tests(<elkodon::service::zero_copy::Service>)]
    mod zero_copy {}

    #[instantiate_tests(<elkodon::service::process_local::Service>)]
    mod process_local {}
}
