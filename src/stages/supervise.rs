use crate::error::Error;

use std::path::PathBuf;
use std::process::{Command, Child};
use std::os::unix::process::CommandExt;
use std::collections::HashMap;
use std::fs;
use std::sync::atomic::{Ordering, AtomicUsize};
use std::sync::Arc;

use nix::unistd::{self, Pid};
use nix::sys::wait;

use signal_hook::{flag, consts};


const SIGNALS: [(i32, usize); 3] = [
    (consts::SIGINT, Signal::Shutdown as usize),
    (consts::SIGUSR1, Signal::Reboot as usize),
    (consts::SIGUSR2, Signal::Update as usize)
];

#[repr(usize)]
enum Signal {
    Shutdown = 1,
    Reboot = 2,
    Update = 3,
}

impl Signal {
    pub fn new(value: usize) -> Option<Signal> {
        match value {
            1 => Some(Signal::Shutdown),
            2 => Some(Signal::Reboot),
            3 => Some(Signal::Update),
            _ => None,
        }
    }
}

pub struct Service {
    path: PathBuf,
    child: Child,
}

impl Service {
    pub fn new(path: PathBuf, child: Child) -> Service {
        Service {
            path,
            child,
        }
    }
}

#[derive(Default)]
pub struct Superviser {
    services: HashMap<Pid, Service>,
}

impl Superviser {
    pub fn probe(path: &PathBuf) -> Result<Superviser, Box<dyn std::error::Error>> {
        let mut superviser = Superviser::default();

        for entry in fs::read_dir(path.join("services")).map_err(|_| Error::InvalidServiceDirectory)?.filter_map(|entry| entry.ok()) {
            superviser.spawn(entry.path());
        }

        Ok(superviser)
    }

    pub fn spawn(&mut self, path: PathBuf) {
        println!("info: start {}", path.display());

        if let Ok(child) = unsafe { Command::new(path.join("supervise")).pre_exec(|| { let _ = unistd::setsid(); Ok(()) }).spawn() } {
            self.services.insert(Pid::from_raw(child.id() as i32), Service::new(path, child));
        }
    }

    pub fn supervise(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let signal = Arc::new(AtomicUsize::new(0));

        for (hook, value) in SIGNALS {
            flag::register_usize(hook, Arc::clone(&signal), value).map_err(|_| Error::SignalFlagFailed)?;
        }

        loop {
            if let Some(pid) = wait::waitpid(Pid::from_raw(-1), None).ok().and_then(|status| status.pid()) {
                if let Some(service) = self.services.remove(&pid) {
                    println!("warn: {}: exited unexpectedly", service.path.display());

                    self.spawn(service.path);
                }
            } else {
                // TODO: use other enum instead
                match Signal::new(signal.load(Ordering::Relaxed)) {
                    Some(Signal::Shutdown) => {
                    },
                    Some(Signal::Reboot) => {
                    },
                    Some(Signal::Update) => {
                    },
                    None => {
                    },
                }
            }
        }
    }
}


