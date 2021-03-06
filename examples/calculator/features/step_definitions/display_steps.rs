use cucumber::CucumberRegistrar;
use support::env::CalculatorWorld;

#[allow(dead_code)]
pub fn register_steps(c: &mut CucumberRegistrar<CalculatorWorld>) {
  Then!(c,
        "^the (?:next )?result is (-)?(\\d+)$",
        |_, world: &mut CalculatorWorld, (negate, mut val): (bool, i32)| {
          if negate {
            val = -val
          }
          assert_eq!(world.calculator.evaluate(), val)
        });

  Then!(c,
        "^the last message includes \"(.*)\"$",
        |_, world: &mut CalculatorWorld, (message,): (String,)| {
          match world.last_response {
            None => panic!("No last message"),
            Some(ref msg) => assert!(msg.to_string().contains(&message)),
          }
        });
}
