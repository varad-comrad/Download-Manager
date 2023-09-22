use std::{io::{Result, Read, Write}, fs::{File, self}, str::FromStr, result, collections::HashMap, thread::sleep, time::Duration, process::Command, env};

extern crate clap;
use clap::{Arg, App, ArgMatches};

extern crate serde;
extern crate serde_json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Manager{
    watchlist: Vec<String>,
    patterns: HashMap<String, String>
}

impl FromStr for Manager{
    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        serde_json::from_str(s).map_err(serde_json::Error::from)
    }

    type Err = serde_json::Error;
}

fn parse_args() -> ArgMatches<'static>{
    App::new("rust-manager")
        .version("0.1.0")
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

fn read_config() -> Result<Manager>{
    let mut path = env::current_dir()?;
    if !path.ends_with("src"){
        path = path.join("src");
    }
    let mut file = File::open(path.join("downloadmanager.json").to_string_lossy().to_string())?;
    let mut str_json = String::new();
    let _ = file.read_to_string(&mut str_json);
    let config: Manager = str_json.parse().unwrap();
    Ok(config)
}

fn write_config(config: Manager) -> Result<()>{
    let mut path = env::current_dir()?;
    if !path.ends_with("src"){
        path = path.join("src");
    }
    let mut file = File::create(path.join("downloadmanager.json").to_string_lossy().to_string())?;
    let str_json = serde_json::to_string(&config).unwrap();
    let _ = file.write_all(str_json.as_bytes());
    Ok(())
}

fn create_pattern(pattern: String, dir: String) -> Result<()>{
    let mut config = read_config()?;
    config.patterns.insert(pattern, dir);
    write_config(config)
}

fn delete_pattern(mut pattern:Vec<&str> ) -> Result<()>{
    let mut config = read_config()?;
    while let Some(last_element) = pattern.pop() {
        config.patterns.remove(last_element);
    }
    write_config(config)
}

fn add_directory(mut dir: Vec<&str>) -> Result<()>{
    let mut config = read_config()?;

    while let Some(last_element) = dir.pop() {
        config.watchlist.append(&mut vec![last_element.to_owned()]);
    }
    write_config(config)
}

fn delete_directory(mut dir: Vec<&str>) -> Result<()>{
    let mut config = read_config()?;
    while let Some(last_element) = dir.pop() {
        config.watchlist.retain(|x| *x != last_element);
    }

    write_config(config)
}

fn whoami() -> Result<String>{
    let user_command = Command::new("whoami").output()?;
    let user = String::from_utf8_lossy(&user_command.stdout);
    Ok(user.to_string().replace('\n', ""))
}

fn watchdog() -> Result<()>{
    let Ok(manager) = read_config() else{
        panic!("Error reading config file")
    };
    let user = whoami()?;
    let home = "/home/".to_owned() + &user;
    for watched_dir in manager.watchlist{
        let directory = home.clone() + "/" + &watched_dir; 
        let dir = fs::read_dir(directory)?;
        for element in dir{
            let path = element?.path();
            let file_name = path.file_name().unwrap().to_str().unwrap().to_owned();
            for (pattern, dir) in &manager.patterns{
                if file_name.contains(pattern){
                    let _ = fs::copy(&path, home.clone() + "/" + dir.clone().as_str() + "/" + &file_name);
                    let _ = fs::remove_file(&path);
                }
            }
        }
    }
    Ok(())
}

fn watchdog_loop() -> Result<()>{
    loop{
        sleep(Duration::from_secs(5));
        let _ = watchdog();
    }
}

fn main() -> Result<()> {
    let matches = parse_args();
    let aux = matches.is_present("loop") as i32
     + matches.is_present("create-pattern") as i32
     + matches.is_present("delete-pattern") as i32
     + matches.is_present("add-directory") as i32
     + matches.is_present("delete-directory") as i32;
     
     if aux > 1{
         println!("Invalid command. Please use only one command at a time.");
         return Ok(());
        }
        if matches.is_present("create-pattern"){
            let values = matches.values_of("create-pattern").unwrap().to_owned();
            let vec_vals = values.collect::<Vec<&str>>();
            let _ = create_pattern(vec_vals[0].to_owned(), vec_vals[1].to_owned());
        } else if matches.is_present("delete-pattern"){
            let values = matches.values_of("delete-pattern").unwrap().to_owned();
            let vec_vals = values.collect::<Vec<&str>>();
            let _ = delete_pattern(vec_vals);
        } else if matches.is_present("add-directory"){
            let values = matches.values_of("add-directory").unwrap().to_owned();
            let vec_vals = values.collect::<Vec<&str>>();
            let _ = add_directory(vec_vals);
    } else if matches.is_present("delete-directory"){
            let values = matches.values_of("delete-directory").unwrap().to_owned();
            let vec_vals = values.collect::<Vec<&str>>();
            let _ = delete_directory(vec_vals);
    } else if matches.is_present("loop"){
        let _ = watchdog_loop();
    } else {
        let _ = watchdog();
    }
    
    Ok(())
}
