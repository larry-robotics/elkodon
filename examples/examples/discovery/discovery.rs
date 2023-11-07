use elkodon::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let services = zero_copy::Service::list()?;

    for service in services {
        println!("\n{:#?}", &service);
    }

    Ok(())
}
