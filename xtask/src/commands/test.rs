use anyhow::Result;
use structopt::StructOpt;

use crate::commands::{IntegrationTest, UnitTest};
use crate::target::{Target, POSSIBLE_TARGETS};

#[derive(Debug, StructOpt)]
pub struct Test {
    // The target to build Rover for
    #[structopt(long = "target", env = "XTASK_TARGET", default_value, possible_values = &POSSIBLE_TARGETS)]
    target: Target,
}

impl Test {
    pub fn run(&self, verbose: bool) -> Result<()> {
        let unit_test_runner = UnitTest {
            target: self.target.clone(),
        };
        unit_test_runner.run(verbose)?;
        let integration_test_runner = IntegrationTest {
            target: self.target.clone(),
            branch: Default::default(),
            org: Default::default(),
        };
        integration_test_runner.run(verbose)?;
        Ok(())
    }
}
