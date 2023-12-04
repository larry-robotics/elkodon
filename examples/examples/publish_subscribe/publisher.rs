use core::time::Duration;
use elkodon::prelude::*;
use transmission_data::TransmissionData;

const CYCLE_TIME: Duration = Duration::from_secs(1);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service_name = ServiceName::new("My/Funk/ServiceName")?;

    let service = zero_copy::Service::new(&service_name)
        .publish_subscribe()
        .open_or_create::<TransmissionData>()?;

    let publisher = service.publisher().create()?;

    let mut counter: u64 = 0;

    while let ElkEvent::Tick = Elk::wait(CYCLE_TIME) {
        counter += 1;
        let sample = publisher.loan_uninit()?;

        let sample = sample.write_payload(TransmissionData {
            x: counter as i32,
            y: counter as i32 * 3,
            funky: counter as f64 * 812.12,
        });

        publisher.send(sample)?;

        println!("Send sample {} ...", counter);
    }

    println!("exit ...");

    Ok(())
}
