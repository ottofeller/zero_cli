#![allow(clippy::all, warnings)]
pub struct SharedTeams;
pub mod shared_teams {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "SharedTeams";
    pub const QUERY : & str = "query SharedTeams($userId: uuid!) {\n  team(\n    where: {\n      _and: [\n        { members: { member: { id: { _eq: $userId } } } }\n        { ownerUserId: { _neq: $userId } }\n      ]\n    }\n    order_by: { updatedAt: desc }\n    limit: 1000\n  ) {\n    id\n    name\n    slug\n\n    owner {\n      id\n      name\n    }\n\n    members_aggregate {\n      aggregate {\n        count\n      }\n    }\n  }\n}\n" ;
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
        #[serde(rename = "userId")]
        pub user_id: uuid,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub team: Vec<SharedTeamsTeam>,
    }
    #[derive(Deserialize)]
    pub struct SharedTeamsTeam {
        pub id: uuid,
        pub name: String,
        pub slug: String,
        pub owner: SharedTeamsTeamOwner,
        pub members_aggregate: SharedTeamsTeamMembersAggregate,
    }
    #[derive(Deserialize)]
    pub struct SharedTeamsTeamOwner {
        pub id: uuid,
        pub name: String,
    }
    #[derive(Deserialize)]
    pub struct SharedTeamsTeamMembersAggregate {
        pub aggregate: Option<SharedTeamsTeamMembersAggregateAggregate>,
    }
    #[derive(Deserialize)]
    pub struct SharedTeamsTeamMembersAggregateAggregate {
        pub count: Int,
    }
}
impl graphql_client::GraphQLQuery for SharedTeams {
    type Variables = shared_teams::Variables;
    type ResponseData = shared_teams::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: shared_teams::QUERY,
            operation_name: shared_teams::OPERATION_NAME,
        }
    }
}
