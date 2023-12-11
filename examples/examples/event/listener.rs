use core::time::Duration;
use elkodon::prelude::*;

const CYCLE_TIME: Duration = Duration::from_secs(1);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_name = ServiceName::new("MyEventName")?;

    let event = zero_copy::Service::new(&event_name)
        .event()
        .open_or_create()?;

    let mut listener = event.listener().create()?;

    while let ElkEvent::Tick = Elk::wait(Duration::ZERO) {
        if let Ok(events) = listener.timed_wait(CYCLE_TIME) {
            for event_id in events {
                println!("event was triggered with id: {:?}", event_id);
            }
        }
    }

    println!("exit ...");

    Ok(())
}
