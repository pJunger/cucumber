extern crate rustyline;
extern crate env_logger;

use std::env;
use std::fs;
use std::path;
use std::io;

use errors::*;
use regex::Regex;


/// Creates a new Cucumber project for the current crate
/// using defaults.
/// # Arguments
/// * `overwrite` - If set, will overwrite the preexisting cucumber project, else it will fail.
/// # Example
///
/// ```
/// if new_project::new_default(true) {
///   println!("Done! New project is ready");
/// }
/// ```
pub fn new_default(overwrite: bool) -> Result<()> {
  println!("Creating a new Cucumber configuration with default values");

  try!(is_cargo_project().and(assert_no_project(overwrite)));

  let world = WorldConfig::default();
  
  Ok(())
}

/// Creates a new Cucumber project for the current crate
/// interactively - will ask for names etc.
/// # Arguments
/// * `overwrite` - If set, will overwrite the preexisting cucumber project, else it will fail.
pub fn new(overwrite: bool) -> Result<()> {

  info!("Creating a new Cucumber configuration");

  try!(is_cargo_project().and(assert_no_project(overwrite)));
  let mut reader = InputReader::new();

  let mut world = get_cucumber_world(&mut reader);

  println!("Do you want to change the address? y | n");
  try!(&reader.next(|line|
    match line.as_ref() {
        "y" => Ok(true),
        "n" => Ok(false),
        x => Err(ErrorKind::NoValidInput.into()),
    }
  ));

  Ok(())
}

fn get_cucumber_world(rl: &mut InputReader) -> Result<WorldConfig> {
  let world_name = try!(get_cucumber_world_name(rl));
  Ok(WorldConfig::new().set_name(world_name))  
}

fn get_cucumber_world_name(rl: &mut InputReader) -> Result<String> {
  println!("Please enter the name of the Cucumber world that is to be created");

  fn check_identifier(candidate: &str) -> Result<String> {
    lazy_static! {
        static ref IS_IDENTIFIER: Regex = Regex::new(r"(?P<ident>\w+)").unwrap();
    }
    IS_IDENTIFIER.captures(candidate).and_then(|m| m.name("ident")).map(|s| s.to_string()).ok_or(ErrorKind::NoValidIdentifier.into())    
  }

  fn get_default_identifier() -> Result<String> {
    Ok("CucumberConfig".to_string())
  }
  
  rl.try_next(|line| 
  match line.as_ref() {
      ":help" => {
        println!("The Cucumber world is a struct which can hold information needed for the test execution.");
        Err(ErrorKind::UserAbort.into())
      },
      ":default" => get_default_identifier(),
      ":quit" => Err(ErrorKind::UserAbort.into()),
      ident => check_identifier(ident),
  })
}



fn delete_preexisting_projects(p: &path::Path) -> Result<()> {
  fs::remove_dir_all(p).chain_err(|| "Cannot remove the features directory!")
}

/// Checks that no cucumber project exists yet
/// If it exists and overwrite is true, then it will return Ok and delete the old project.
fn assert_no_project(overwrite: bool) -> Result<()> {
  // Check if a project exists
  let mut features = try!(env::current_dir().chain_err(|| "The current dir should be available!"));
  features.push("features");
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

fn is_cargo_project() -> Result<()> {
  let mut proj_dir = env::current_dir().expect("The current dir should be available!");
  proj_dir.push("Cargo.toml");

  try!(fs::metadata(proj_dir.as_path()).chain_err(|| {
    error!("This is not a cargo project!");
    ErrorKind::NotACargoProjectError
  }));
  Ok(())
}

struct WorldConfig {
  name: String,
  address: String,
  port: usize,
  registrar_fns: Vec<String>,
  arguments: Vec<String>,
}

impl WorldConfig {
  pub fn new() -> WorldConfig {
    WorldConfig {
      name: WorldConfig::default_name(),
      address: WorldConfig::default_address(),
      port: WorldConfig::default_port(),
      registrar_fns: Vec::new(),
      arguments: Vec::new(),
    }
  }

  pub fn default() -> WorldConfig {
    WorldConfig::new().add_registrar_fn(WorldConfig::default_registrar_fn())
  }

  pub fn add_registrar_fn<S: Into<String>>(mut self, s: S) -> WorldConfig {
    self.registrar_fns.push(s.into());
    self
  }

  pub fn add_arg<S: Into<String>>(mut self, s: S) -> WorldConfig {
    self.arguments.push(s.into());
    self
  }

  pub fn set_name<S: Into<String>>(mut self, s: S) -> WorldConfig {
    self.name = s.into();
    self
  }

  pub fn set_address<S: Into<String>>(mut self, addr: S, port: usize) -> WorldConfig {
    self.address = addr.into();
    self.port = port;
    self
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

struct InputReader(rustyline::Editor<()>);

impl InputReader {
  fn new() -> InputReader {
    InputReader(rustyline::Editor::<()>::new())
  }

  fn next<F, R>(&mut self, fun: F) -> Result<R>
      where F: FnOnce(String) -> R {
    let line = try!(self.0.readline(">>").chain_err(|| "No input?"));
    Ok(fun(line))
  }

  fn try_next<F, R>(&mut self, fun: F) -> Result<R>
      where F: FnOnce(String) -> Result<R> {
    let line = try!(self.0.readline(">>").chain_err(|| "No input?"));
    fun(line)
  }
}