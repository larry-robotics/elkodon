#[generic_tests::define]
mod subscriber {
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
    fn number_of_publishers_works<Sut: Service>() {
        let service_name = generate_name();
        let service = Sut::new(&service_name)
            .publish_subscribe()
            .create::<u64>()
            .unwrap();

        let sut = service.subscriber().create().unwrap();
        assert_that!(sut.number_of_publishers(), eq 0);

        let _publisher = service.publisher().create();
        assert_that!(sut.number_of_publishers(), eq 0);
        assert_that!(sut.update_connections(), is_ok);
        assert_that!(sut.number_of_publishers(), eq 1);
    }

    #[instantiate_tests(<elkodon::service::zero_copy::Service>)]
    mod zero_copy {}

    #[instantiate_tests(<elkodon::service::process_local::Service>)]
    mod process_local {}
}
