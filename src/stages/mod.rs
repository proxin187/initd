pub mod supervise;
pub mod script;

use crate::args::Args;

use supervise::Supervisor;

use nix::sys::reboot;


pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::new()?;
    let services = args.services_path()?;

    println!("info: initd.services={}", services.display());

    println!("info: enter boot stage");

    script::run_dir(services.join("boot"), "start")?;

    println!("info: enter supervise stage");

    let mut supervisor = Supervisor::new(&services);

    supervisor.update()?;

    let mode = supervisor.supervise()?;

    println!("info: enter shutdown stage");

    script::run_dir(services.join("shutdown"), "shutdown")?;

    let _ = reboot::reboot(mode);

    Ok(())
}


