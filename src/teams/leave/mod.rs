mod graphql;
use crate::common::{
    authorization_headers::authorization_headers,
    colorful_theme::theme,
    execute_graphql_request::execute_graphql_request,
    keyring::keyring,
    print_formatted_error::print_formatted_error,
    take_user_id_from_token::take_user_id_from_token,
};
use crate::teams::common::team_info::team_info;
use clap::Args;
use dialoguer::Input;
use graphql::remove_user_from_team::{remove_user_from_team, RemoveUserFromTeam};
use graphql_client::GraphQLQuery;
use reqwest::Client;
use termimad::crossterm::style::Stylize;
use uuid::Uuid;

#[derive(Args, Debug)]
pub struct TeamLeaveArgs {
    #[clap(short, long, help = "Team slug")]
    slug: String,
    #[clap(
        short,
        long,
        help = "Access token, if not specified, the token will be taken from the keychain"
    )]
    access_token: Option<String>,
}

pub fn leave(args: &TeamLeaveArgs) {
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

    let input: String = Input::with_theme(&theme())
        .with_prompt(format!(
            "Type {} to confirm leaving the team:",
            &args.slug
        ))
        .interact_text()
        .expect("Failed to read the user's reply.");

    if input != args.slug {
        println!(
            "{}",
            format!("X Sorry your reply was invalid: You entered {}.", input).red()
        );

        std::process::exit(0);
    }

    let team_info = team_info(&access_token, args.slug.clone());
    let leave_team_error_message = "Failed to leave the team.";

    let leave_team_id = execute_graphql_request::<
        remove_user_from_team::Variables,
        remove_user_from_team::ResponseData,
    >(
        authorization_headers(&access_token),
        RemoveUserFromTeam::build_query,
        &Client::new(),
        leave_team_error_message,
        remove_user_from_team::Variables {
            team_id: team_info.id.to_string(),
            user_id: user_id.to_string(),
        },
    )
    .remove_user_from_team
    .team_id;

    if leave_team_id.is_empty() {
        print_formatted_error(&leave_team_error_message);
        std::process::exit(1);
    } else {
        println!(
            "{} You have successfully left the '{}' team.",
            "✔".green(),
            team_info.name
        );
    }
}
