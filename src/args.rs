use std::ffi::OsString;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct  Args {
    #[arg(short, long, help = "Keeps the program open and adds the data to a list.")]
    pub automatic: bool,
    #[arg(short, long, help = "Filepath for the output file.")]
    pub output_path: OsString,
    #[arg(short, long, help = "Interval in which the program gets run.", default_value_t=60)]
    pub interval: u64,
}