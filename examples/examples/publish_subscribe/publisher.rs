use elkodon::prelude::*;
use elkodon_bb_posix::signal::SignalHandler;
use transmission_data::TransmissionData;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service_name = ServiceName::new(b"My/Funk/ServiceName")?;

    let service = zero_copy::Service::new(&service_name)
        .publish_subscribe()
        .open_or_create::<TransmissionData>()?;

    let publisher = service.publisher().create()?;

    let mut counter: u64 = 0;

    while !SignalHandler::termination_requested() {
        counter += 1;

        let sample = publisher.loan_uninit()?;

        let sample = sample.write_payload(TransmissionData {
            x: counter as i32,
            y: counter as i32 * 3,
            funky: counter as f64 * 812.12,
        });

        publisher.send(sample)?;

        println!("Send sample {} ...", counter);

        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    println!("exit ...");

    Ok(())
}
