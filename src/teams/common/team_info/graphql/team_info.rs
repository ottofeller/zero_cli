#![allow(clippy::all, warnings)]
pub struct TeamInfo;
pub mod team_info {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "TeamInfo";
    pub const QUERY : & str = "query TeamInfo($slug: String!) {\n  team(where: {slug: {_eq: $slug}}) {\n    id\n    name\n    description\n    ownerUserId\n    slug\n\n    members {\n      id\n\n      member {\n        id\n      }\n    }\n  }\n}\n" ;
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
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub team: Vec<TeamInfoTeam>,
    }
    #[derive(Deserialize, Clone)]
    pub struct TeamInfoTeam {
        pub id: uuid,
        pub name: String,
        pub description: String,
        #[serde(rename = "ownerUserId")]
        pub owner_user_id: uuid,
        pub slug: String,
        pub members: Vec<TeamInfoTeamMembers>,
    }
    #[derive(Deserialize, Clone)]
    pub struct TeamInfoTeamMembers {
        pub id: uuid,
        pub member: TeamInfoTeamMembersMember,
    }
    #[derive(Deserialize, Clone)]
    pub struct TeamInfoTeamMembersMember {
        pub id: uuid,
    }
}
impl graphql_client::GraphQLQuery for TeamInfo {
    type Variables = team_info::Variables;
    type ResponseData = team_info::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: team_info::QUERY,
            operation_name: team_info::OPERATION_NAME,
        }
    }
}
