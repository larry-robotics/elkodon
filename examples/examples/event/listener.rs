use elkodon::prelude::*;
use elkodon_bb_posix::signal::SignalHandler;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_name = ServiceName::new(b"MyEventName")?;

    let event = zero_copy::Service::new(&event_name)
        .event()
        .open_or_create()?;

    let mut listener = event.listener().create()?;

    while !SignalHandler::termination_requested() {
        for event_id in listener.timed_wait(std::time::Duration::from_secs(1))? {
            println!("event was triggered with id: {:?}", event_id);
        }
    }

    println!("exit ...");

    Ok(())
}
