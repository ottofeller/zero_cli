#![allow(clippy::all, warnings)]
pub struct InsertSecretSharingRecord;
pub mod insert_secret_sharing_record {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "InsertSecretSharingRecord";
    pub const QUERY : & str = "mutation InsertSecretSharingRecord(\n  $expiresAt: String!\n  $passphrase: String!\n  $secretsFieldIds: [ID!]!\n) {\n  insertSecretSharingRecord(\n    expiresAt: $expiresAt\n    passphrase: $passphrase\n    secretsFieldIds: $secretsFieldIds\n  ) {\n    id\n  }\n}\n" ;
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
        #[serde(rename = "expiresAt")]
        pub expires_at: String,
        pub passphrase: String,
        #[serde(rename = "secretsFieldIds")]
        pub secrets_field_ids: Vec<ID>,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        #[serde(rename = "insertSecretSharingRecord")]
        pub insert_secret_sharing_record: InsertSecretSharingRecordInsertSecretSharingRecord,
    }
    #[derive(Deserialize)]
    pub struct InsertSecretSharingRecordInsertSecretSharingRecord {
        pub id: ID,
    }
}
impl graphql_client::GraphQLQuery for InsertSecretSharingRecord {
    type Variables = insert_secret_sharing_record::Variables;
    type ResponseData = insert_secret_sharing_record::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: insert_secret_sharing_record::QUERY,
            operation_name: insert_secret_sharing_record::OPERATION_NAME,
        }
    }
}
