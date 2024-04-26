#![allow(clippy::all, warnings)]
pub struct GenerateToken;
pub mod generate_token {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "GenerateToken";
    pub const QUERY : & str = "mutation GenerateToken($id: String!, $name: String!, $expiresAt: String) {\n  createProjectToken(\n    object: { projectId: $id, name: $name, expiresAt: $expiresAt }\n  ) {\n    tokenId\n    tokenValue\n  }\n}\n" ;
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    #[derive(Serialize)]
    pub struct Variables {
        pub id: String,
        pub name: String,
        #[serde(rename = "expiresAt")]
        pub expires_at: Option<String>,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        #[serde(rename = "createProjectToken")]
        pub create_project_token: GenerateTokenCreateProjectToken,
    }
    #[derive(Deserialize)]
    pub struct GenerateTokenCreateProjectToken {
        #[serde(rename = "tokenId")]
        pub token_id: ID,
        #[serde(rename = "tokenValue")]
        pub token_value: String,
    }
}
impl graphql_client::GraphQLQuery for GenerateToken {
    type Variables = generate_token::Variables;
    type ResponseData = generate_token::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: generate_token::QUERY,
            operation_name: generate_token::OPERATION_NAME,
        }
    }
}
