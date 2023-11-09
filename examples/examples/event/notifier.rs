use elkodon::prelude::*;
use elkodon_bb_posix::signal::SignalHandler;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_name = ServiceName::new(b"MyEventName")?;

    let event = zero_copy::Service::new(&event_name)
        .event()
        .open_or_create()?;

    let notifier = event.notifier().create()?;

    let mut counter: u64 = 0;
    while !SignalHandler::termination_requested() {
        counter += 1;
        notifier.notify_with_custom_event_id(EventId::new(counter))?;

        println!("Trigger event with id {} ...", counter);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    println!("exit ... ");

    Ok(())
}
