mod fetch;
mod introspect;
mod push;

use anyhow::Result;
use serde::Serialize;
use structopt::StructOpt;

use crate::command::RoverStdout;

#[derive(Debug, Serialize, StructOpt)]
pub struct Schema {
    #[structopt(subcommand)]
    command: Command,
}

#[derive(Debug, Serialize, StructOpt)]
pub enum Command {
    /// 🐶 Get a schema given an identifier
    Fetch(fetch::Fetch),

    /// Push a schema from a file
    Push(push::Push),

    /// Introspect a schema from a running service
    Introspect(introspect::Introspect),
}

impl<'a> Schema {
    pub fn run(&self) -> Result<RoverStdout> {
        match &self.command {
            Command::Fetch(command) => command.run(),
            Command::Push(command) => command.run(),
            Command::Introspect(command) => command.run(),
        }
    }
}
