use std::fs;


pub fn boot() -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir("/etc/pid1/boot/")? {
    }

    Ok(())
}


