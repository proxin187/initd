mod stages;
mod error;
mod args;

use std::os::unix::process::CommandExt;
use std::process::{self, Command};
use std::panic;


fn main() {
    panic::set_hook(Box::new(|panic_info| {
        if let Some(payload) = panic_info.payload_as_str() {
            println!("error: initd panic: {}", payload);
        }

        println!("critical: failed to exec emergency shell: {}", Command::new("/bin/sh").exec());
    }));

    if process::id() == 1 {
        if let Err(err) = stages::init() {
            panic!("{}", err);
        }

        unreachable!();
    } else {
        println!("error: initd failed: not PID 1");
    }
}


