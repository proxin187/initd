pub mod boot;
pub mod supervise;
pub mod shutdown;

use crate::args::Args;

use supervise::Supervisor;


pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::new()?;
    let services = args.services_path()?;

    println!("info: initd.services={}", services.display());

    println!("info: enter boot stage");

    boot::boot(&services)?;

    println!("info: enter supervise stage");

    let mut supervisor = Supervisor::new(&services);

    supervisor.update()?;

    let mode = supervisor.supervise()?;

    println!("info: enter shutdown stage");

    Ok(())
}


