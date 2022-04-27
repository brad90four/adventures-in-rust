#![allow(unused)]

use clap::Parser;
use anyhow::{Context, Result};
use log::{debug, info, warn};



#[derive(Debug)] // add an attribute to the struct in order to use the debug print
#[derive(Parser)] // Use clap to to parse the file path
struct Cli {
    pattern: String, // the pattern to search for
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf, // the path to the file to read
}



/*
#[derive(Debug)]
struct CustomError(String);
*/


fn main() -> Result<()> { // some sorta error handling
    // Logging!
    env_logger::init();
    debug!("Logging started");
    info!("Just FYI");
    warn!("Ruh-roh");
    /* Section 1
    let pattern = std::env::args() // use the built in library to collect arguments
        .nth(1) // the "nth" index of 1 will be the pattern
        .expect("no pattern given"); // on error, there was no pattern given
    let path = std::env::args()
        .nth(2) // the "nth" index of 2 will be the path
        .expect("no path given"); // on error, there was no path given
    
    /*
    println!("Pattern is {}", pattern); // check that the pattern is captured
    println!("Path is {}", path); // check that the path is captured
    */
    // create a struct to hold data in fields
    let args = Cli {
        pattern: pattern,
        path: std::path::PathBuf::from(path),
    };
    println!("Args are: {:?}", args); // print the args with a debug print to format the struct
    Section 1 */

    /* Section 2
    let args = Cli::parse(); // fill the Cli struct from parse
    println!("Args are: {:?}", args); 

    let content = std::fs::read_to_string(&args.path) // use std::fs::read_to_string to read a file
        .expect("could not read file");

    for line in content.lines() {
        // println!("Line: {}", &line);
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
    Section 2 */


    /* Section 3 - unknown territory */
    let args = Cli::parse();

    /* Lets change this up
    let result = std::fs::read_to_string(&args.path);
    let content = match result {
        Ok(content) => { content },
        Err(error) => { return Err(error.into());}
    };
    */
    /* We can do better than that
    let content = std::fs::read_to_string(&args.path)
        .map_err(|err| CustomError(format!("Error reading '{:?}: {}", &args.path, err)))?;
    */
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
