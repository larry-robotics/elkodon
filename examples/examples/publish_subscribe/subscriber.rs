use elkodon::prelude::*;
use elkodon_bb_posix::signal::SignalHandler;
use transmission_data::TransmissionData;

fn main() {
    let service_name = ServiceName::new(b"My/Funk/ServiceName").unwrap();

    let service = zero_copy::Service::new(&service_name)
        .publish_subscribe()
        .open_or_create::<TransmissionData>()
        .expect("failed to create/open service");

    let subscriber = service
        .subscriber()
        .create()
        .expect("Failed to create subscriber");

    while !SignalHandler::was_ctrl_c_pressed() {
        while let Some(sample) = subscriber.receive().unwrap() {
            println!("received: {:?}", *sample);
        }

        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    println!("exit ...");
}
