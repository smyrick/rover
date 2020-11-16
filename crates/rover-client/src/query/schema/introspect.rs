use crate::blocking::Client;
use crate::RoverClientError;
use graphql_client::*;
use std::collections::HashMap;

#[derive(GraphQLQuery)]
// The paths are relative to the directory where your `Cargo.toml` is located.
// Both json and the GraphQL schema language are supported as sources for the schema
#[graphql(
    query_path = "src/query/schema/introspect.graphql",
    schema_path = "introspection-schema.graphql",
    response_derives = "PartialEq, Debug, Serialize, Deserialize",
    deprecated = "warn"
)]
/// This struct is used to generate the module containing `Variables` and
/// `ResponseData` structs.
/// Snake case of this name is the mod name. i.e. introspection_query
pub struct IntrospectionQuery;

/// The main function to be used from this module. This function
/// TODO: DOCS
pub fn run(headers: HashMap<String, String>, client: Client) -> Result<String, RoverClientError> {
    let response_data = execute_query(client, headers)?;
    let sdl = convert_response_to_sdl(response_data)?;
    Ok(sdl)
}

fn execute_query(
    client: Client,
    headers: HashMap<String, String>,
) -> Result<introspection_query::ResponseData, RoverClientError> {
    let res = client.post::<IntrospectionQuery>(introspection_query::Variables {}, &headers)?;
    if let Some(data) = res {
        Ok(data)
    } else {
        Err(RoverClientError::HandleResponse {
            msg: "Error fetching schema. No data in response".to_string(),
        })
    }
}

fn convert_response_to_sdl(
    response_data: introspection_query::ResponseData,
) -> Result<String, RoverClientError> {
    // first we stringify the results to raw json
    let json = serde_json::to_string(&response_data);

    // if the stringify is successful, we try to parse that as a `Schema`
    if let Ok(_json) = json {
        // let parsed = graphql_parser::schema::parse_schema::<String>(&json);
        // dbg!(parsed);
        Ok("type Query {\n  hello: String!\n}".to_string())
    } else {
        Err(RoverClientError::HandleResponse {
            msg: "Could not parse introspection response to json".to_string(),
        })
    }
    // let json: graphql_introspection_query::introspection_response::IntrospectionResponse = serde_json::from_str(response_data).unwrap();
}

// fn get_schema_from_response_data(
//     response_data: introspection_query::ResponseData,
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
//         let data: introspection_query::ResponseData = serde_json::from_value(json_response).unwrap();
//         let output = get_schema_from_response_data(data);

//         assert!(output.is_ok());
//         assert_eq!(output.unwrap(), "type Query { hello: String }".to_string());
//     }

//     #[test]
//     fn get_schema_from_response_data_errs_on_no_service() {
//         let json_response = json!({ "service": null });
//         let data: introspection_query::ResponseData = serde_json::from_value(json_response).unwrap();
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
//         let data: introspection_query::ResponseData = serde_json::from_value(json_response).unwrap();
//         let output = get_schema_from_response_data(data);

//         assert!(output.is_err());
//     }
// }
