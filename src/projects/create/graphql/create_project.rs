#![allow(clippy::all, warnings)]
pub struct CreateProject;
pub mod create_project {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "CreateProject";
    pub const QUERY : & str = "mutation CreateProject($icon: String!, $name: String!, $token: TokenObject, $slug: String!) {\n  createProject(icon: $icon, name: $name, token: $token, slug: $slug) {\n    id\n    tokenValue\n    slug\n    projectTeamId\n  }\n}\n" ;
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
    pub struct TokenObject {
        #[serde(rename = "expiresAt")]
        pub expires_at: Option<String>,
        pub name: String,
    }
    #[derive(Serialize)]
    pub struct Variables {
        pub icon: String,
        pub name: String,
        pub token: Option<TokenObject>,
        pub slug: String,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        #[serde(rename = "createProject")]
        pub create_project: CreateProjectCreateProject,
    }
    #[derive(Deserialize)]
    pub struct CreateProjectCreateProject {
        pub id: Option<ID>,
        #[serde(rename = "tokenValue")]
        pub token_value: Option<String>,
        pub slug: Option<String>,
        #[serde(rename = "projectTeamId")]
        pub project_team_id: Option<String>,
    }
}
impl graphql_client::GraphQLQuery for CreateProject {
    type Variables = create_project::Variables;
    type ResponseData = create_project::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: create_project::QUERY,
            operation_name: create_project::OPERATION_NAME,
        }
    }
}
