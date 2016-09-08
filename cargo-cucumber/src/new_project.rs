extern crate rustyline;
extern crate env_logger;

use std::fs;
use std::env;
use std::path;

pub fn new_default(overwrite: bool) {
    println!("Creating a new Cucumber configuration with default values")
}

pub fn new(overwrite: bool) {
    info!("Creating a new Cucumber configuration");

    let cuc = check_for_preexisting_projects(overwrite);
    let proj = check_if_cargo_project();

    if let Ok(_) = cuc.and(proj) {
        let mut rl = rustyline::Editor::<()>::new();

        error!("HELP, Im kidnapped");
        while let Ok(line) = rl.readline(">> ") {}
    }
    else {
        unimplemented!();
    }
}

fn delete_preexisting_projects(p: &path::Path) -> Result<(), ()> {
    fs::remove_dir_all(p).map_err(|_| ())
}


fn check_for_preexisting_projects(overwrite: bool) -> Result<(), ()> {
    /* Check if a project exists */
    let mut features = env::current_dir().expect("The current dir should be available!");
    features.push(get_features_dir_name());
    if fs::metadata(features.as_path()).is_err() {
        debug!("No project found");
        Ok(())
    } else if overwrite {
        try!(delete_preexisting_projects(features.as_path()));
        info!("Cucumber project exists already! Overwriting!");
        Ok(())
    } else {
        error!("Cucumber project exists already! Not overwriting!");
        Err(())
    }
}

fn check_if_cargo_project() -> Result<(), ()> {
    let mut features = env::current_dir().expect("The current dir should be available!");
    features.push(get_cargo_name());
    if let Err(_) = fs::metadata(features.as_path()) {
        error!("This is not a cargo project!");
        Err(())
    } else {
        Ok(())
    }
}

#[cfg(not(test))]
fn get_cargo_name() -> &'static str {
    "Cargo.toml"
}

#[cfg(test)]
fn get_cargo_name() -> &'static str {
    "Cargo_test.toml"
}

#[cfg(not(test))]
fn get_features_dir_name() -> &'static str {
    "features"
}

#[cfg(test)]
fn get_features_dir_name() -> &'static str {
    "features_test"
}

struct WorldConfig {
    name: String,
    address: String,
    port: usize,
    registrar_fns: Vec<String>,
    arguments: Vec<String>,
}

impl WorldConfig {
    fn new(name: String) -> WorldConfig {
        WorldConfig {
            name: name,
            address: "127.0.0.1".to_string(),
            port: 7878,
            registrar_fns: Vec::new(),
            arguments: Vec::new(),
        }
    }
}