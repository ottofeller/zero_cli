#![allow(clippy::all, warnings)]
pub struct UpdateSecretFields;
pub mod update_secret_fields {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "UpdateSecretFields";
    pub const QUERY : & str = "mutation UpdateSecretFields(\n  $id: ID!\n  $name: String!\n  $slug: String!\n  $userSecretFields: [UpdateUserSecretFieldsInput!]!\n) {\n  updateSecret(fields: $userSecretFields, id: $id, name: $name, slug: $slug) {\n    id\n  }\n}\n" ;
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
    pub struct UpdateUserSecretFieldsInput {
        #[serde(rename = "decryptedValue")]
        pub decrypted_value: String,
        pub id: Option<ID>,
        pub name: String,
    }
    #[derive(Serialize)]
    pub struct Variables {
        pub id: ID,
        pub name: String,
        pub slug: String,
        #[serde(rename = "userSecretFields")]
        pub user_secret_fields: Vec<UpdateUserSecretFieldsInput>,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        #[serde(rename = "updateSecret")]
        pub update_secret: UpdateSecretFieldsUpdateSecret,
    }
    #[derive(Deserialize)]
    pub struct UpdateSecretFieldsUpdateSecret {
        pub id: ID,
    }
}
impl graphql_client::GraphQLQuery for UpdateSecretFields {
    type Variables = update_secret_fields::Variables;
    type ResponseData = update_secret_fields::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: update_secret_fields::QUERY,
            operation_name: update_secret_fields::OPERATION_NAME,
        }
    }
}
