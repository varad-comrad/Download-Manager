use std::process::Command;
use std::io::Result;
extern crate clap;
use clap::{Arg, App, ArgMatches};

extern crate serde;
extern crate serde_json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Manager{
    watchlist: String,
    patterns: Vec<(String, String)>
}

fn parse_args() -> ArgMatches<'static>{
    App::new("rust-manager")
        .version("0.1.0")
        .author("Siddharth Dushantha <")
        .arg(
            Arg::with_name("create-pattern")
                .short("c")
                .long("create-pattern")
                .value_name("PATTERN")
                .help("Create a pattern")
                .takes_value(true)
                .multiple(true)
                .number_of_values(2)
                .required(false)
        )
        .arg(
            Arg::with_name("delete-pattern")
                .short("d")
                .long("delete-pattern")
                .value_name("PATTERN")
                .help("Delete a pattern")
                .takes_value(true)
                .multiple(true)
                .number_of_values(2)
                .required(false)
        )
        .arg(
            Arg::with_name("add-directory")
                .short("a")
                .long("add-directory")
                .value_name("DIRECTORY")
                .help("Add a directory to watchlist")
                .takes_value(true)
                .multiple(true)
                .number_of_values(1)
                .required(false)
        )
        .arg(
            Arg::with_name("delete-directory")
                .short("r")
                .long("delete-directory")
                .value_name("DIRECTORY")
                .help("Delete a directory from watchlist")
                .takes_value(true)
                .multiple(true)
                .number_of_values(1)
                .required(false)
        )
        .arg(
            Arg::with_name("loop")
                .short("l")
                .long("loop")
                .help("Run the manager in a loop")
                .takes_value(false)
                .required(false)
        )
        .get_matches()
}

fn create_pattern() -> Vec<(String, String)>{
    let mut patterns: Vec<(String, String)> = Vec::new();
    patterns.push(("".to_string(), "".to_string()));
    patterns
}

fn delete_pattern() -> Vec<(String, String)>{
    let mut patterns: Vec<(String, String)> = Vec::new();
    patterns.push(("".to_string(), "".to_string()));
    patterns
}

fn add_directory() -> String{
    "".to_string()
}

fn delete_directory() -> String{
    "".to_string()
}

fn watchdog() -> String{
    "".to_string()
}

fn watchdog_loop() -> String{
    loop{
        // sleep(1000);
        watchdog();

    }
}

fn main() -> Result<()> {
    // Create a Command to run an external command (e.g., "ls" with arguments)
    let matches = parse_args();
    // match matches{
    //     "create-pattern" => create_pattern(),
    //     "delete-pattern" => delete_pattern(),
    //     "add-directory" => add_directory(),
    //     "delete-directory" => delete_directory(),
    //     "loop" => watchdog_loop(),
    //     _ => println!("Invalid command")
    // }
    let output = Command::new("ls")
        .arg("-l")
        .output()?; // Execute the command and capture its output

    // Check if the command was successful
    if output.status.success() {
        // Convert the output bytes to a string
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("Output:\n{}", stdout);
    } else {
        eprintln!("Error: Command failed with {:?}", output.status);
    }

    Ok(())
}
