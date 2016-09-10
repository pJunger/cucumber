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

  let cuke = get_cucumber_world(&mut rl);
  println!("Please enter the name of the Cucumber world that is to be created");
  Ok(())
}

fn get_cucumber_world(rl: &mut rustyline::Editor<()>) -> Result<String> {
  println!("Please enter the name of the Cucumber world that is to be created");

  fn check_identifier(l: &str) -> Result<String> {
    Err("Add real logic to check_identifier!".into()) // TODO: Real logic
  }

  fn get_default_identifier() -> Result<String> {
    Ok("CucumberConfig".to_string())
  }

  if let Ok(line) = rl.readline(">>") {
    match line.as_ref() {
      ":help" => {
        println!("The Cucumber world is a struct which can hold information needed for the test \
                  execution.");
        get_cucumber_world(rl)
      },
      ":default" => get_default_identifier(),
      ":quit" => Err(ErrorKind::UserAbort.into()),
      ident => check_identifier(ident),
    }
  } else {
    Err("something".into())
  }
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
  fn new() -> WorldConfig {
    WorldConfig {
      name: WorldConfig::default_name(),
      address: "127.0.0.1".to_string(),
      port: 7878,
      registrar_fns: Vec::new(),
      arguments: Vec::new(),
    }
  }

  fn default() -> WorldConfig {
    WorldConfig {
      name: WorldConfig::default_name(),
      address: WorldConfig::default_address(),
      port: WorldConfig::default_port(),
      registrar_fns: vec![WorldConfig::default_registrar_fn()],
      arguments: Vec::new(),
    }
  }

  fn default_name() -> String {
    "CucumberConfig".to_string()
  }

  fn default_address() -> String {
    "127.0.0.1".to_string()
  }

  fn default_port() -> usize {
    7878
  }
  fn default_registrar_fn() -> String {
    "CucumberSteps".to_string()
  }
}
