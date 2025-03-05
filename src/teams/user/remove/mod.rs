mod graphql;
use crate::common::{
    authorization_headers::authorization_headers,
    colorful_theme::theme,
    execute_graphql_request::execute_graphql_request,
    keyring::keyring,
    print_formatted_error::print_formatted_error,
    query_full_id::{query_full_id, QueryType},
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
pub struct UserRemoveArgs {
    #[clap(short, long, help = "Team slug")]
    slug: String,
    #[clap(short, long, help = "User ID (First 4 characters or more are allowed)")]
    user_id: String,
    #[clap(
        short,
        long,
        help = "Access token, if not specified, the token will be taken from the keychain"
    )]
    access_token: Option<String>,
}

pub fn remove(args: &UserRemoveArgs) {
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

    let user_id_to_be_deleted = query_full_id(QueryType::User, args.user_id.clone(), &access_token);
    let team_info = team_info(&access_token, args.slug.clone());

    // Check if the user is the owner of the team
    if team_info.owner_user_id != user_id {
        print_formatted_error(
            "You are not the owner of the team. Only the owner can remove users.",
        );

        std::process::exit(1);
    }

    // Check if the selected user is the member of the team
    if team_info
        .members
        .iter()
        .find(|member| member.member.id == user_id_to_be_deleted)
        .is_none()
    {
        print_formatted_error("The selected user is not a member of the team.");
        std::process::exit(1);
    }

    // Confirm deletion
    let input: String = Input::with_theme(&theme())
        .with_prompt(format!("Type {} to confirm deletion:", &args.slug))
        .interact_text()
        .expect("Failed to read the user's reply.");

    if input != args.slug {
        println!(
            "{}",
            format!("X Sorry your reply was invalid: You entered {}.", input).red()
        );

        std::process::exit(0);
    }

    let remove_team_error_message = "Failed to remove the user from the team.";

    let response_team_id = execute_graphql_request::<
        remove_user_from_team::Variables,
        remove_user_from_team::ResponseData,
    >(
        authorization_headers(&access_token),
        RemoveUserFromTeam::build_query,
        &Client::new(),
        remove_team_error_message,
        remove_user_from_team::Variables {
            team_id: team_info.id.to_string(),
            user_id: user_id_to_be_deleted.to_string(),
        },
    )
    .remove_user_from_team
    .team_id;

    if response_team_id.is_empty() {
        println!(
            "{} User '{}' has not been removed from the '{}' team.",
            "❌".red(),
            user_id_to_be_deleted.to_string(),
            team_info.id.to_string()
        );

        std::process::exit(0);
    } else {
        println!(
            "{} {}",
            "✔".green(),
            "User successfully removed from the team"
        );

        std::process::exit(0);
    }
}
