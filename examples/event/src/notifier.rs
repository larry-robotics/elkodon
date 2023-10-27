use elkodon::service::{service_name::ServiceName, zero_copy, Service};
use elkodon_bb_container::semantic_string::SemanticString;
use elkodon_bb_posix::signal::SignalHandler;

fn main() {
    let event_name = ServiceName::new(b"MyEventName").unwrap();

    let event = zero_copy::Service::new(&event_name)
        .event()
        .open_or_create()
        .expect("failed to create/open event");

    let notifier = event
        .notifier()
        .create()
        .expect("failed to create notifier");

    let mut counter: u64 = 0;
    while !SignalHandler::was_ctrl_c_pressed() {
        counter += 1;
        notifier
            .notify_with_custom_trigger_id(counter)
            .expect("failed to trigger event");

        println!("Trigger event with id {} ...", counter);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    println!("exit ... ");
}
