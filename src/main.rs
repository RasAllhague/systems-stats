mod args;
mod systeminfo;

use std::{path::Path, thread};

use args::Args;
use clap::Parser;
use systeminfo::{SystemInfo, SystemInfoContainer};
use systemstat::{Platform, System};


#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Serde(serde_json::Error),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IO(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::Serde(value)
    }
}

fn main() -> Result<(), Error> {
    let args = Args::parse();
    let system = System::new();

    if !args.automatic {
        let info = SystemInfo::collect(&system);
        return info.save(&args.output_path);
    }

    loop {
        let mut container = SystemInfoContainer::new();

        if Path::new(&args.output_path).exists() {
            container = SystemInfoContainer::load(&args.output_path)?;
        }

        let info = SystemInfo::collect(&system);
        container.push(&info);
        container.save(&args.output_path)?;

        thread::sleep(std::time::Duration::from_secs(args.interval));
    }
}
