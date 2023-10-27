#[generic_tests::define]
mod serialize {
    use elkodon_bb_testing::assert_that;
    use elkodon_cal::serialize::Serialize;

    #[derive(Debug, serde::Serialize, serde::Deserialize, Eq, PartialEq)]
    struct TestStruct {
        value1: String,
        value2: u64,
        value3: bool,
    }

    #[test]
    fn serialize_deserialize_works<Sut: Serialize>() {
        let test_object = TestStruct {
            value1: "hello world".to_string(),
            value2: 192381,
            value3: false,
        };

        let serialized = Sut::serialize(&test_object);
        assert_that!(serialized, is_ok);

        let deserialized = Sut::deserialize::<TestStruct>(&serialized.unwrap());
        assert_that!(deserialized, is_ok);
        assert_that!(deserialized.unwrap(), eq test_object);
    }

    #[instantiate_tests(<elkodon_cal::serialize::toml::Toml>)]
    mod toml {}

    #[instantiate_tests(<elkodon_cal::serialize::cdr::Cdr>)]
    mod cdr {}
}
