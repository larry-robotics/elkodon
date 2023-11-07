use elkodon::prelude::*;
use elkodon_bb_posix::signal::SignalHandler;
use transmission_data::TransmissionData;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service_name = ServiceName::new(b"My/Funk/ServiceName")?;

    let service = zero_copy::Service::new(&service_name)
        .publish_subscribe()
        .open_or_create::<TransmissionData>()?;

    let subscriber = service.subscriber().create()?;

    while !SignalHandler::termination_requested() {
        while let Some(sample) = subscriber.receive()? {
            println!("received: {:?}", *sample);
        }

        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    println!("exit ...");

    Ok(())
}
