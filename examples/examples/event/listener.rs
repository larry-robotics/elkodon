use elkodon::prelude::*;
use elkodon_bb_posix::signal::SignalHandler;

fn main() {
    let event_name = ServiceName::new(b"MyEventName").unwrap();

    let event = zero_copy::Service::new(&event_name)
        .event()
        .open_or_create()
        .expect("failed to create/open event");

    let mut listener = event
        .listener()
        .create()
        .expect("failed to create listener");

    while !SignalHandler::was_ctrl_c_pressed() {
        for event_id in listener
            .timed_wait(std::time::Duration::from_secs(1))
            .expect("failed to wait on listener")
        {
            println!("event was triggered with id: {:?}", event_id);
        }
    }

    println!("exit ...");
}
