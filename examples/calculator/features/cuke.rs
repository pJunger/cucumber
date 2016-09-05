extern crate calculator;

#[macro_use]
extern crate cucumber;

mod step_definitions;
mod support;

use step_definitions::{calculator_steps, display_steps};
use support::env::CalculatorWorld;

#[test]
fn main() {
  cucumber::create_config(CalculatorWorld::new())
    .registrar_fn(&calculator_steps::register_steps)
    .registrar_fn(&display_steps::register_steps)
    .start();
}
