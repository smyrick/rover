use anyhow::Result;
use houston as config;
use rover_client::blocking::Client;
use rover_client::query::schema::{check, get};
use serde::Serialize;
use structopt::StructOpt;

#[derive(Debug, Serialize, StructOpt)]
pub struct Check {
    /// ID of the graph to fetch from Apollo Studio
    #[structopt(name = "GRAPH_NAME")]
    #[serde(skip_serializing)]
    graph_name: String,

    /// The variant of the request graph from Apollo Studio
    #[structopt(long, default_value = "current")]
    #[serde(skip_serializing)]
    variant: String,

    #[structopt(long = "profile", default_value = "default")]
    #[serde(skip_serializing)]
    profile_name: String,
}

impl Check {
    pub fn run(&self) -> Result<()> {
        match config::Profile::get_api_key(&self.profile_name) {
            Ok(api_key) => {
                tracing::info!(
                    "Checking local schema against {}@{}",
                    &self.graph_name,
                    &self.variant
                );

                let client = Client::new(
                    api_key,
                    "https://graphql.api.apollographql.com/api/graphql".to_string(),
                );

                let result = check::run(
                    get::get_schema_query::Variables {
                        graph_id: self.graph_name.clone(),
                        hash: None,
                        variant: Some(self.variant.clone()),
                    },
                    client,
                )?;

                tracing::info!(%result);
                Ok(())
            }
            Err(e) => Err(e.into()),
        }
    }
}
