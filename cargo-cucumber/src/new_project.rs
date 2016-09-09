extern crate rustyline;
extern crate env_logger;


use errors::*;
use std::env;
use std::fs;
use std::path;

pub fn new_default(overwrite: bool) -> Result<()> {
  println!("Creating a new Cucumber configuration with default values");
  Ok(())
}

pub fn new(overwrite: bool) -> Result<()> {

  info!("Creating a new Cucumber configuration");

  try!(check_if_cargo_project().and(check_for_preexisting_projects(overwrite)));
  let mut rl = rustyline::Editor::<()>::new();

  get_cucumber_world(rl);
  println!("Please enter the name of the Cucumber world that is to be created");
//  while let Ok(line) = rl.readline(">> ") {
//    match Some(line) {
//      Some(_) => do_something();
//
//    }
//  }
  Ok(())
}

fn get_cucumber_world(rl: rustyline::Editor<()>) -> Result<()>{
  println!("Please enter the name of the Cucumber world that is to be created");
//  if let Ok(line) = rrl
Ok(())
}

fn delete_preexisting_projects(p: &path::Path) -> Result<()> {
  fs::remove_dir_all(p).chain_err(|| "Cannot remove the features directory!")
}


fn check_for_preexisting_projects(overwrite: bool) -> Result<()> {
  // Check if a project exists
  let mut features = try!(env::current_dir().chain_err(|| "The current dir should be available!"));
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
    Err(ErrorKind::ProjectAlreadyExistsError.into())
  }
}

fn check_if_cargo_project() -> Result<()> {
  let mut features = env::current_dir().expect("The current dir should be available!");
  features.push(get_cargo_name());
  if let Err(_) = fs::metadata(features.as_path()) {
    error!("This is not a cargo project!");
    Err(ErrorKind::NotACargoProjectError.into())
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
