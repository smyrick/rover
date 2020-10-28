mod check;
mod fetch;

use anyhow::Result;
use serde::Serialize;
use structopt::StructOpt;

#[derive(Debug, Serialize, StructOpt)]
pub struct Schema {
    #[structopt(subcommand)]
    command: Command,
}

#[derive(Debug, Serialize, StructOpt)]
pub enum Command {
    /// 🐶 Get a schema given an identifier
    Fetch(fetch::Fetch),
    /// ✅ Check a schema for breaking changes
    Check(check::Check),
}

impl Schema {
    pub fn run(&self) -> Result<()> {
        match &self.command {
            Command::Fetch(fetch) => fetch.run(),
            Command::Check(check) => check.run(),
        }
    }
}
