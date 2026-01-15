pub mod boot;
pub mod supervise;
pub mod shutdown;

use crate::args::Args;

use supervise::Superviser;


pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::new()?;
    let services = args.services_path()?;

    println!("info: initd.services={}", services.display());

    println!("info: enter boot stage");

    boot::boot(&services)?;

    println!("info: enter supervise stage");

    let mut superviser = Superviser::probe(&services)?;

    superviser.supervise();

    println!("info: enter shutdown stage");

    Ok(())
}


