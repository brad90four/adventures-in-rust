#![allow(unused)]

use clap::Parser;
use anyhow::{Context, Result};
use log::{debug, info, warn};


#[derive(Debug)]
#[derive(Parser)]
struct Cli {
    pattern: String,
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}


fn main() -> Result<()> {
    // Logging!
    env_logger::init();
    debug!("Logging started");

    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("Could not read file'{:?}'", &args.path))?;
    let mut lineno = 0;
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}, {}", &lineno, &line);
            debug!("{}, {}", &lineno, &line)
        }
        lineno += 1;
    };
    Ok(())
}
