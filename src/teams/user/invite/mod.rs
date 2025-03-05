mod graphql;
use crate::common::{
    authorization_headers::authorization_headers,
    execute_graphql_request::execute_graphql_request,
    keyring::keyring,
    print_formatted_error::print_formatted_error,
    take_user_id_from_token::take_user_id_from_token,
};
use crate::teams::common::team_info::team_info;
use clap::Args;
use graphql::send_invite::{send_invite, SendInvite};
use graphql_client::GraphQLQuery;
use reqwest::Client;
use termimad::crossterm::style::Stylize;
use uuid::Uuid;

#[derive(Args, Debug)]
pub struct UserInviteArgs {
    #[clap(short, long, help = "Team slug")]
    slug: String,
    #[clap(short, long, help = "The email of the user to invite")]
    email: String,
    #[clap(
        short,
        long,
        help = "Access token, if not specified, the token will be taken from the keychain"
    )]
    access_token: Option<String>,
}

pub fn invite(args: &UserInviteArgs) {
    let access_token = match &args.access_token {
        Some(token) => token.clone(),
        None => keyring::get("access_token"),
    };

    let user_id = match Uuid::parse_str(&take_user_id_from_token(&access_token)) {
        Ok(uuid) => uuid,

        Err(err) => {
            print_formatted_error(&format!("Invalid user id: {}", err));
            std::process::exit(1);
        }
    };

    let team_info = team_info(&access_token, args.slug.clone());

    if team_info.owner_user_id != user_id {
        print_formatted_error(
            "You are not the owner of the team. Only the owner can invite users.",
        );

        std::process::exit(1);
    };

    let error_message = "Failed to send the invitation.";

    let invited_team_id =
        execute_graphql_request::<send_invite::Variables, send_invite::ResponseData>(
            authorization_headers(&access_token),
            SendInvite::build_query,
            &Client::new(),
            &error_message,
            send_invite::Variables {
                team_id: team_info.id.to_string(),
                email: args.email.to_string(),
            },
        )
        .send_invite_user_team
        .team_id;

    if invited_team_id.is_empty() {
        print_formatted_error(error_message);
        std::process::exit(1);
    } else {
        println!(
            "{} {} {}",
            "✔".green(),
            "The invitation was successfully sent to the user.",
            &args.email.to_string()
        );
    }
}
