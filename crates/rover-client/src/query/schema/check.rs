use crate::blocking::Client;
use crate::RoverClientError;
use graphql_client::*;

/// GraphQLDocument scalar is a String in the schema
type GraphQLDocument = String;

#[derive(GraphQLQuery)]
// The paths are relative to the directory where your `Cargo.toml` is located.
// Both json and the GraphQL schema language are supported as sources for the schema
#[graphql(
    query_path = "src/query/schema/check.graphql",
    schema_path = "schema.graphql",
    response_derives = "PartialEq, Debug, Serialize, Deserialize",
    deprecated = "warn"
)]
/// This struct is used to generate the module containing `Variables` and
/// `ResponseData` structs.
/// Snake case of this name is the mod name. i.e. check_schema_query
pub struct CheckSchemaQuery;

/// The main function to be used from this module. This function fetches a
/// schema from apollo studio and returns it in either sdl (default) or json format
pub fn run(
    variables: check_schema_query::Variables,
    client: Client,
) -> Result<String, RoverClientError> {
    let response_data = execute_query(client, variables)?;
    tracing::debug!(?response_data);
    Ok("schema check complete".to_string())
}

fn execute_query(
    client: Client,
    variables: check_schema_query::Variables,
) -> Result<check_schema_query::ResponseData, RoverClientError> {
    let res = client.post::<CheckSchemaQuery>(variables)?;
    if let Some(data) = res {
        Ok(data)
    } else {
        Err(RoverClientError::HandleResponse {
            msg: "Error running schema checks. No data in response".to_string(),
        })
    }
}

// fn get_schema_from_response_data(
//     response_data: get_schema_query::ResponseData,
// ) -> Result<String, RoverClientError> {
//     let service_data = match response_data.service {
//         Some(data) => Ok(data),
//         None => Err(RoverClientError::HandleResponse {
//             msg: "No service found".to_string(),
//         }),
//     }?;

//     if let Some(schema) = service_data.schema {
//         Ok(schema.document)
//     } else {
//         Err(RoverClientError::HandleResponse {
//             msg: "No schema found for this variant".to_string(),
//         })
//     }
// }
