/// ```compile_fail
/// use elkodon::prelude::*;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # let service_name = ServiceName::new(b"My/Funk/ServiceName").unwrap();
/// #
/// # let service = zero_copy::Service::new(&service_name)
/// #     .publish_subscribe()
/// #     .open_or_create::<u64>()?;
/// #
/// # let publisher = service.publisher().create()?;
///
/// let mut sample = publisher.loan_uninit()?;
/// sample.payload_mut().write(1234);
///
/// publisher.send(sample)?; // should fail to compile
///
/// # Ok(())
/// # }
/// ```
#[cfg(doctest)]
fn sending_uninitialized_sample_fails_to_compile() {}