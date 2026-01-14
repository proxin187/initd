use crate::error::Error;

use std::path::PathBuf;
use std::fs;


pub fn boot(services: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    println!("info: entering boot stage");

    for result in fs::read_dir(services.join("boot")).map_err(|_| Error::InvalidServiceDirectory)? {
        if let Ok(entry) = result {
            println!("info: starting {}", entry.path().display());
        }
    }

    Ok(())
}


