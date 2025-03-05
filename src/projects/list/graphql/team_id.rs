#![allow(clippy::all, warnings)]
pub struct TeamId;
pub mod team_id {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "TeamId";
    pub const QUERY : & str = "query TeamId($slug: String!, $userId: uuid!) {\n  team(\n    where: {\n      _and: [\n        { slug: { _eq: $slug } }\n        { members: { member: { id: { _eq: $userId } } } }\n      ]\n    }\n  ) {\n    id\n    name\n    slug\n  }\n}\n" ;
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
        #[serde(rename = "userId")]
        pub user_id: uuid,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub team: Vec<TeamIdTeam>,
    }
    #[derive(Deserialize)]
    pub struct TeamIdTeam {
        pub id: uuid,
        pub name: String,
        pub slug: String,
    }
}
impl graphql_client::GraphQLQuery for TeamId {
    type Variables = team_id::Variables;
    type ResponseData = team_id::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: team_id::QUERY,
            operation_name: team_id::OPERATION_NAME,
        }
    }
}
