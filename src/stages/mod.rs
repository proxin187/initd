pub mod boot;
pub mod supervise;
pub mod shutdown;

use crate::args::Args;


pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::new()?;
    let services = args.services_path()?;

    println!("info: initd.services={}", services.display());

    boot::boot(&services)?;

    loop {
    }
}


