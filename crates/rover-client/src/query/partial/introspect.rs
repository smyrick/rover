use crate::blocking::Client;
use crate::RoverClientError;
use graphql_client::*;
use std::collections::HashMap;

#[derive(GraphQLQuery)]
// The paths are relative to the directory where your `Cargo.toml` is located.
// Both json and the GraphQL schema language are supported as sources for the schema
#[graphql(
    query_path = "src/query/partial/introspect.graphql",
    schema_path = "federation-schema.graphql",
    response_derives = "PartialEq, Debug, Serialize, Deserialize",
    deprecated = "warn"
)]
/// This struct is used to generate the module containing `Variables` and
/// `ResponseData` structs.
/// Snake case of this name is the mod name. i.e. federated_introspection_query
pub struct FederatedIntrospectionQuery;

/// The main function to be used from this module. This function
/// TODO: DOCS
pub fn run(
    headers: HashMap<String, String>,
    client: Client,
) -> Result<String, RoverClientError> {
    let response_data = execute_query(client, headers)?;
    let sdl = get_sdl_from_response_data(response_data)?;
    Ok(sdl)
}

fn execute_query(
    client: Client,
    headers: HashMap<String, String>,
) -> Result<federated_introspection_query::ResponseData, RoverClientError> {
    let res = client.post::<FederatedIntrospectionQuery>(federated_introspection_query::Variables {}, &headers)?;
    if let Some(data) = res {
        // dbg!(&data);
        Ok(data)
    } else {
        Err(RoverClientError::HandleResponse {
            msg: "Error fetching schema. No data in response".to_string(),
        })
    }
}

fn get_sdl_from_response_data(
    response_data: federated_introspection_query::ResponseData,
) -> Result<String, RoverClientError> {
    if let Some(sdl) = response_data.service.sdl {
        Ok(sdl)
    } else {
        Err(RoverClientError::HandleResponse {
            msg: "SDL not returned from the service. Check your authentication settings. This service may not support federation".to_string(),
        })
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use serde_json::json;
//     #[test]
//     fn get_schema_from_response_data_works() {
//         let json_response = json!({
//             "service": {
//                 "schema": {
//                     "document": "type Query { hello: String }"
//                 }
//             }
//         });
//         let data: federated_introspection_query::ResponseData = serde_json::from_value(json_response).unwrap();
//         let output = get_schema_from_response_data(data);

//         assert!(output.is_ok());
//         assert_eq!(output.unwrap(), "type Query { hello: String }".to_string());
//     }

//     #[test]
//     fn get_schema_from_response_data_errs_on_no_service() {
//         let json_response = json!({ "service": null });
//         let data: federated_introspection_query::ResponseData = serde_json::from_value(json_response).unwrap();
//         let output = get_schema_from_response_data(data);

//         assert!(output.is_err());
//     }

//     #[test]
//     fn get_schema_from_response_data_errs_on_no_schema() {
//         let json_response = json!({
//             "service": {
//                 "schema": null
//             }
//         });
//         let data: federated_introspection_query::ResponseData = serde_json::from_value(json_response).unwrap();
//         let output = get_schema_from_response_data(data);

//         assert!(output.is_err());
//     }
// }
