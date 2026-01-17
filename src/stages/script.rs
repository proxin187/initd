use crate::error::Error;

use std::process::Command;
use std::fs::{self, DirEntry};
use std::path::PathBuf;


pub fn run_dir(dir: PathBuf, message: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut entries = fs::read_dir(dir)
        .map_err(|_| Error::InvalidServiceDirectory)?
        .filter_map(|entry| entry.ok())
        .collect::<Vec<DirEntry>>();

    entries.sort_by_key(|entry| entry.file_name());

    for entry in entries {
        println!("info: {}: {}", message, entry.path().display());

        match Command::new(entry.path()).status() {
            Ok(status) if !status.success() => println!("warn: {}: exited with a non-zero exit code", entry.path().display()),
            Err(err) => println!("error: {}: {}", entry.path().display(), err),
            _ => {},
        }
    }

    Ok(())
}


