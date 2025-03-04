#![allow(clippy::all, warnings)]
pub struct SendInvite;
pub mod send_invite {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "SendInvite";
    pub const QUERY : & str = "mutation SendInvite($teamId: ID!, $email: String!) {\n  sendInviteUserTeam(teamId: $teamId, email: $email) {\n    teamId\n  }\n}\n" ;
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
        #[serde(rename = "teamId")]
        pub team_id: ID,
        pub email: String,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        #[serde(rename = "sendInviteUserTeam")]
        pub send_invite_user_team: SendInviteSendInviteUserTeam,
    }
    #[derive(Deserialize)]
    pub struct SendInviteSendInviteUserTeam {
        #[serde(rename = "teamId")]
        pub team_id: ID,
    }
}
impl graphql_client::GraphQLQuery for SendInvite {
    type Variables = send_invite::Variables;
    type ResponseData = send_invite::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: send_invite::QUERY,
            operation_name: send_invite::OPERATION_NAME,
        }
    }
}
