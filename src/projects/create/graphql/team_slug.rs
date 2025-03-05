#![allow(clippy::all, warnings)]
pub struct TeamSlug;
pub mod team_slug {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "TeamSlug";
    pub const QUERY: &str =
        "query TeamSlug($id: uuid!) {\n  team_by_pk(id: $id) {\n    id\n    slug\n  }\n}\n";
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
        pub id: uuid,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub team_by_pk: Option<TeamSlugTeamByPk>,
    }
    #[derive(Deserialize)]
    pub struct TeamSlugTeamByPk {
        pub id: uuid,
        pub slug: String,
    }
}
impl graphql_client::GraphQLQuery for TeamSlug {
    type Variables = team_slug::Variables;
    type ResponseData = team_slug::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: team_slug::QUERY,
            operation_name: team_slug::OPERATION_NAME,
        }
    }
}
