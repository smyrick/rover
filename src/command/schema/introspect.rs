use anyhow::Result;
use serde::Serialize;
use std::collections::HashMap;
use structopt::StructOpt;

use rover_client::query::schema::introspect;

use crate::client::get_client;
use crate::command::RoverStdout;

#[derive(Debug, Serialize, StructOpt)]
pub struct Introspect {
    /// Uri to introspect
    #[structopt(name = "URI")]
    #[serde(skip_serializing)]
    uri: String,

    /// "key: value" pairs of headers to pass for introspection
    #[structopt(long = "header")]
    #[serde(skip_serializing)]
    header: Option<Vec<String>>,
}

impl Introspect {
    pub fn run(&self) -> Result<RoverStdout> {
        let client = get_client(&self.uri)?;

        tracing::info!("Introspecting schema from {}", &self.uri);

        // TODO: build header map
        let headers = build_header_map(&self.header);

        let sdl = introspect::run(headers, client)?;

        Ok(RoverStdout::SDL(sdl))
    }
}

// TODO: how to handle headers with colons in them
fn build_header_map(raw_headers: &Option<Vec<String>>) -> HashMap<String, String> {
    let mut header_map = HashMap::new();
    if let Some(headers) = raw_headers {
        for header in headers {
            let pair: Vec<&str> = header.split(":").collect();
            if pair.len() < 2 {
                panic!("oh noe")
            }
            header_map.insert(pair[0].to_string(), pair[1].to_string());
        }
    }
    header_map
}
