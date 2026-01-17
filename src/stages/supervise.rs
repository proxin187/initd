use crate::error::Error;

use std::path::PathBuf;
use std::process::Command;
use std::os::unix::process::CommandExt;
use std::fs;
use std::sync::atomic::{Ordering, AtomicUsize};
use std::sync::Arc;

use nix::sys::reboot::RebootMode;
use nix::unistd::{self, Pid};
use nix::sys::{wait, signal};

use signal_hook::{flag, consts};


const SIGNALS: [(i32, usize); 3] = [
    (consts::SIGINT, Signal::Shutdown as usize),
    (consts::SIGUSR1, Signal::Reboot as usize),
    (consts::SIGUSR2, Signal::Update as usize)
];

#[repr(usize)]
pub enum Signal {
    Shutdown = 1,
    Reboot = 2,
    Update = 3,
    Other,
}

impl Signal {
    pub fn new(value: usize) -> Signal {
        match value {
            1 => Signal::Shutdown,
            2 => Signal::Reboot,
            3 => Signal::Update,
            _ => Signal::Other,
        }
    }

    pub fn reboot_mode(&self) -> RebootMode {
        match self {
            Signal::Shutdown => RebootMode::RB_POWER_OFF,
            Signal::Reboot => RebootMode::RB_AUTOBOOT,
            _ => unreachable!(),
        }
    }
}

pub struct Service {
    pid: Pid,
    path: PathBuf,
}

impl Service {
    pub fn new(pid: Pid, path: PathBuf) -> Service {
        Service {
            pid,
            path,
        }
    }

    pub fn kill(&self) {
        println!("info: kill: {}", self.path.display());

        if let Err(_) = signal::kill(self.pid, signal::SIGTERM) {
            println!("error: unable to kill: {}", self.path.display());
        }
    }

    pub fn wait(&self) {
        println!("info: wait: {}", self.path.display());

        if let Err(_) = wait::waitpid(self.pid, None) {
            println!("error: unable to wait: {}", self.path.display());
        }
    }
}

pub struct Supervisor<'a> {
    services: Vec<Service>,
    path: &'a PathBuf,
}

impl<'a> Supervisor<'a> {
    pub fn new(path: &'a PathBuf) -> Supervisor<'a> {
        Supervisor {
            services: Vec::new(),
            path,
        }
    }

    pub fn update(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("info: update: {}", self.path.display());

        let dir = fs::read_dir(self.path.join("services"))
            .map_err(|_| Error::InvalidServiceDirectory)?
            .filter_map(|entry| entry.ok().map(|entry| entry.path()))
            .collect::<Vec<PathBuf>>();

        self.services.retain(|service| {
            dir.contains(&service.path) || {
                service.kill();
                false
            }
        });

        for path in dir {
            if !self.services.iter().any(|service| service.path == path) {
                self.spawn(&path);
            }
        }

        Ok(())
    }

    pub fn spawn(&mut self, path: &PathBuf) {
        println!("info: start: {}", path.display());

        match unsafe { Command::new(path).pre_exec(|| { let _ = unistd::setsid(); Ok(()) }).spawn() } {
            Ok(child) => self.services.push(Service::new(Pid::from_raw(child.id() as i32), path.clone())),
            Err(err) => println!("error: {}: {}", path.display(), err),
        }
    }

    pub fn shutdown(&self) {
        for service in self.services.iter() {
            service.kill();
        }

        for service in self.services.iter() {
            service.wait();
        }
    }

    pub fn supervise(&mut self) -> Result<RebootMode, Box<dyn std::error::Error>> {
        let signal = Arc::new(AtomicUsize::new(0));

        for (hook, value) in SIGNALS {
            flag::register_usize(hook, Arc::clone(&signal), value).map_err(|_| Error::SignalFlagFailed)?;
        }

        loop {
            if let Some(pid) = wait::waitpid(Pid::from_raw(-1), None).ok().and_then(|status| status.pid()) {
                if let Some(index) = self.services.iter().position(|service| service.pid == pid) {
                    let service = self.services.remove(index);

                    println!("warn: {}: exited unexpectedly", service.path.display());

                    self.spawn(&service.path);
                }
            } else {
                let signal = Signal::new(signal.load(Ordering::Relaxed));

                match signal {
                    Signal::Shutdown | Signal::Reboot => {
                        self.shutdown();

                        return Ok(signal.reboot_mode());
                    },
                    Signal::Update => self.update()?,
                    Signal::Other => println!("warn: ignoring invalid signal"),
                }
            }
        }
    }
}


