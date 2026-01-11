mod stages;
mod error;

use error::Error;

use std::os::unix::process::CommandExt;
use std::process::{self, Command};
use std::fs;


struct Cmdline {
    args: String,
}

impl Cmdline {
    pub fn new() -> Result<Cmdline, Box<dyn std::error::Error>> {
        let args = fs::read_to_string("/proc/cmdline")?;

        Ok(Cmdline {
            args,
        })
    }

    pub fn services_path(&self) -> Result<String, Box<dyn std::error::Error>> {
        self.args.find("initd.services=")
            .and_then(|index| self.args[index..].strip_prefix("initd.services="))
            .and_then(|path| path.contains(' ').then_some(path.split_once(" ")).unwrap_or(Some((path, ""))))
            .map(|(path, _)| path.to_string())
            .ok_or(Box::new(Error::NoServicePath))
    }
}

fn init() -> Result<(), Box<dyn std::error::Error>> {
    let cmdline = Cmdline::new()?;

    println!("info: detected services: {}", cmdline.services_path()?);

    loop {
    }
}

fn main() {
    if process::id() == 1 {
        match init() {
            Err(error) => println!("error: initd failed: {}", error),
            Ok(_) => println!("error: initd unexpectedly returned"),
        }

        println!("critical: failed to exec emergency shell: {}", Command::new("/bin/dash").exec());

        loop {}
    } else {
        println!("error: initd failed: {}", Error::NotPid1);
    }
}


