use crate::error::Error;

use std::path::PathBuf;
use std::env;


pub struct Args {
    args: Vec<String>,
}

impl Args {
    pub fn new() -> Result<Args, Box<dyn std::error::Error>> {
        Ok(Args {
            args: env::args().collect::<Vec<String>>(),
        })
    }

    pub fn services_path(&self) -> Result<PathBuf, Box<dyn std::error::Error>> {
        self.args.iter()
            .find(|arg| arg.starts_with("initd.services="))
            .map(|arg| PathBuf::from(arg.trim_start_matches("initd.services=")))
            .ok_or(Box::new(Error::NoServicePath))
    }
}


