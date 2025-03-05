#![allow(clippy::all, warnings)]
pub struct SecretInfo;
pub mod secret_info {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "SecretInfo";
    pub const QUERY : & str = "query SecretInfo($slug: String!) {\n  userSecret(where:{slug: {_eq: $slug}}) {\n    id\n    name\n    slug\n\n    project {\n      id\n      name\n      slug\n\n      team {\n        id\n        slug\n        name\n      }\n    }\n  }\n}\n" ;
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
    type uuid = ::uuid::Uuid;
    #[derive(Serialize)]
    pub struct Variables {
        pub slug: String,
    }
    impl Variables {}
    #[derive(Deserialize, Clone)]
    pub struct ResponseData {
        #[serde(rename = "userSecret")]
        pub user_secret: Vec<SecretInfoUserSecret>,
    }
    #[derive(Deserialize, Clone)]
    pub struct SecretInfoUserSecret {
        pub id: uuid,
        pub name: String,
        pub slug: String,
        pub project: SecretInfoUserSecretProject,
    }
    #[derive(Deserialize, Clone)]
    pub struct SecretInfoUserSecretProject {
        pub id: uuid,
        pub name: String,
        pub slug: String,
        pub team: Option<SecretInfoUserSecretProjectTeam>,
    }
    #[derive(Deserialize, Clone)]
    pub struct SecretInfoUserSecretProjectTeam {
        pub id: uuid,
        pub slug: String,
        pub name: String,
    }
}
impl graphql_client::GraphQLQuery for SecretInfo {
    type Variables = secret_info::Variables;
    type ResponseData = secret_info::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: secret_info::QUERY,
            operation_name: secret_info::OPERATION_NAME,
        }
    }
}
