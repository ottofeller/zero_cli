#![allow(clippy::all, warnings)]
pub struct Team;
pub mod team {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "Team";
    pub const QUERY : & str = "query Team($id: uuid!) {\n  team_by_pk(id: $id) {\n    id\n    name\n    description\n    slug\n\n    owner {\n      id\n      name\n    }\n\n    members {\n      id\n\n      member {\n        id\n        name\n      }\n    }\n  }\n}\n" ;
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
        pub team_by_pk: Option<TeamTeamByPk>,
    }
    #[derive(Deserialize)]
    pub struct TeamTeamByPk {
        pub id: uuid,
        pub name: String,
        pub description: String,
        pub slug: String,
        pub owner: TeamTeamByPkOwner,
        pub members: Vec<TeamTeamByPkMembers>,
    }
    #[derive(Deserialize)]
    pub struct TeamTeamByPkOwner {
        pub id: uuid,
        pub name: String,
    }
    #[derive(Deserialize)]
    pub struct TeamTeamByPkMembers {
        pub id: uuid,
        pub member: TeamTeamByPkMembersMember,
    }
    #[derive(Deserialize)]
    pub struct TeamTeamByPkMembersMember {
        pub id: uuid,
        pub name: String,
    }
}
impl graphql_client::GraphQLQuery for Team {
    type Variables = team::Variables;
    type ResponseData = team::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: team::QUERY,
            operation_name: team::OPERATION_NAME,
        }
    }
}
