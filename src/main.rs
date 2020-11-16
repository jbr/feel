#![forbid(unsafe_code, future_incompatible)]
#![deny(
    missing_debug_implementations,
    nonstandard_style,
    missing_copy_implementations,
    unused_qualifications
)]

use filetime::FileTime;
use structopt::StructOpt;

use std::fs::{DirBuilder, OpenOptions};
use std::path::PathBuf;
use std::time::SystemTime;

#[derive(StructOpt, Debug)]
#[structopt(name = "feel", about = "it goes beyond touch")]
struct FeelOpts {
    #[structopt(parse(from_os_str))]
    path: PathBuf,
}

fn main() -> Result<(), String> {
    let FeelOpts { path } = FeelOpts::from_args();
    let dir = path.parent().expect("unable to find the path base");

    DirBuilder::new()
        .recursive(true)
        .create(dir)
        .map_err(|_| format!("could not create {}", dir.to_string_lossy()))?;

    OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(&path)
        .map_err(|_| format!("could not open {}", path.to_string_lossy()))?;

    let file_time = FileTime::from_system_time(SystemTime::now());

    filetime::set_file_times(path, file_time, file_time)
        .map_err(|_| String::from("could not update file times"))?;

    Ok(())
}
