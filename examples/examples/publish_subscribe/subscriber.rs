use core::time::Duration;
use elkodon::prelude::*;
use transmission_data::TransmissionData;

const CYCLE_TIME: Duration = Duration::from_secs(1);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service_name = ServiceName::new(b"My/Funk/ServiceName")?;

    let service = zero_copy::Service::new(&service_name)
        .publish_subscribe()
        .open_or_create::<TransmissionData>()?;

    let subscriber = service.subscriber().create()?;

    while let ElkEvent::Tick = Elk::wait(CYCLE_TIME) {
        while let Some(sample) = subscriber.receive()? {
            println!("received: {:?}", *sample);
        }
    }

    println!("exit ...");

    Ok(())
}
