#[macro_use]
extern crate cucumber;

extern crate tempdir;
extern crate itertools;

mod step_definitions;
mod support;

use support::env::CucumberWorld;

use step_definitions::project_steps;

fn cuke() {
  cucumber::create_config(CucumberWorld::new()).registrar_fn(&project_steps::register_steps).start();
}

fn main() {
  cuke()
}
