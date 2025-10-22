use std::fs::OpenOptions;
use std::process::exit;

use anyhow::Result;
use log::{LevelFilter, error};
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode, WriteLogger};

#[rustfmt::skip]
use firefoxpwa::{connector::Connection, directories::ProjectDirs};

fn main() -> Result<()> {
    let dirs = ProjectDirs::new()?;

    let debugmode = dirs.userdata.join("DEBUG").exists();
    let loglevel = if debugmode { LevelFilter::Debug } else { LevelFilter::Warn };

    let logfile = dirs.userdata.join("firefoxpwa.log");
    let logfile = OpenOptions::new().create(true).append(true).open(logfile)?;

    CombinedLogger::init(vec![
        TermLogger::new(loglevel, Config::default(), TerminalMode::Stderr, ColorChoice::Auto),
        WriteLogger::new(loglevel, Config::default(), logfile),
    ])?;

    if let Err(error) = Connection::start(&dirs, debugmode) {
        error!("{error:?}");
        exit(1);
    }

    Ok(())
}
