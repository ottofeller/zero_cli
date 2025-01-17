mod graphql;
use crate::common::{
    authorization_headers::authorization_headers,
    colorful_theme::theme,
    config::Config,
    execute_graphql_request::execute_graphql_request,
    keyring::keyring,
    print_formatted_error::print_formatted_error,
    query_full_id::{query_full_id, QueryType},
    slugify::slugify_prompt,
};
use clap::Args;
use dialoguer::Input;
use graphql::teams_info::{teams_info, TeamsInfo};
use graphql::update_team_description::{update_team_description, UpdateTeamDescription};
use graphql::update_team_name::{update_team_name, UpdateTeamName};
use graphql_client::GraphQLQuery;
use reqwest::Client;
use termimad::crossterm::style::{style, Color, Stylize};

#[derive(Args, Debug)]
pub struct TeamsEditArgs {
    #[clap(short, long, help = "Team ID (First 4 characters or more are allowed)")]
    id: String,
    #[clap(short, long, help = "The name of the team, not required")]
    name: Option<String>,
    #[clap(short, long, help = "The description of the team, not required")]
    description: Option<String>,
    #[clap(short, long, help = "The slug of the team, not required")]
    slug: Option<String>,
    #[clap(
        short,
        long,
        help = "Access token, if not specified, the token will be taken from the keychain"
    )]
    access_token: Option<String>,
}

pub fn edit(args: &TeamsEditArgs) -> () {
    let access_token = match &args.access_token {
        Some(token) => token.clone(),
        None => keyring::get("access_token"),
    };

    let team_id = query_full_id(QueryType::Teams, args.id.clone(), &access_token);
    let client = Client::new();
    let headers = authorization_headers(&access_token);
    let mut name = args.name.clone();
    let mut description = args.description.clone();
    let user_teams_error_message = "Editing error. Failed to retrieve user teams.";

    let user_teams = execute_graphql_request::<teams_info::Variables, teams_info::ResponseData>(
        headers.clone(),
        TeamsInfo::build_query,
        &client,
        user_teams_error_message,
        teams_info::Variables {},
    )
    .team;

    if user_teams.len() == 0 {
        print_formatted_error("Editing error. You don't have any teams.");
        std::process::exit(1);
    }

    let edited_team = match user_teams.iter().find(|team| team.id == team_id) {
        Some(team_info) => team_info,

        None => {
            print_formatted_error(user_teams_error_message);
            std::process::exit(1);
        }
    };

    // Protect change name for `Personal projects`
    if edited_team.name == "Personal projects" {
        print_formatted_error(
            "Editing error. You can't change the name of the team `Personal projects`.",
        );

        std::process::exit(1);
    }

    let already_used_team_names: Vec<String> = user_teams
        .iter()
        .map(|team| team.name.to_string())
        .collect();

    if name.is_none() && description.is_none() {
        name = match Input::with_theme(&theme())
            .with_prompt("Type a new team name:")
            .validate_with(|name: &String| -> Result<(), &str> {
                if already_used_team_names.contains(&name) && name != &edited_team.name {
                    return Err("The team name must be unique.");
                } else {
                    Ok(())
                }
            })
            .default(edited_team.name.clone())
            .interact()
        {
            Ok(new_name) => Some(new_name),

            Err(_) => {
                print_formatted_error("Editing error. Failed to read the team name.");
                std::process::exit(1);
            }
        };

        description = match Input::with_theme(&theme())
            .with_prompt("Type a new team description:")
            .default(match &edited_team.description {
                Some(description) => description.clone(),
                None => "".to_string(),
            })
            .interact()
        {
            Ok(new_description) => Some(new_description),

            Err(_) => {
                print_formatted_error("Editing error. Failed to read the team description.");
                std::process::exit(1);
            }
        };
    }

    if let Some(name) = name {
        let update_team_name_error_message = "Editing error. Failed to update the team name.";

        let updated_team_id =
            execute_graphql_request::<update_team_name::Variables, update_team_name::ResponseData>(
                headers.clone(),
                UpdateTeamName::build_query,
                &client,
                update_team_name_error_message,
                update_team_name::Variables {
                    id: team_id,
                    name: name.to_owned(),
                    slug: slugify_prompt(&name, "Type a slug for the team:"),
                },
            )
            .update_team_by_pk;

        if updated_team_id.is_none() {
            print_formatted_error(update_team_name_error_message);
            std::process::exit(1);
        }
    }

    if let Some(description) = description {
        let update_team_description_error_message =
            "Editing error. Failed to update the team description.";

        let updated_team_id = execute_graphql_request::<
            update_team_description::Variables,
            update_team_description::ResponseData,
        >(
            headers.clone(),
            UpdateTeamDescription::build_query,
            &client,
            update_team_description_error_message,
            update_team_description::Variables {
                id: team_id,
                description,
            },
        )
        .update_team_by_pk;

        if updated_team_id.is_none() {
            print_formatted_error(update_team_description_error_message);
            std::process::exit(1);
        }
    }

    println!(
        "{} {}\n{}",
        "✔".green(),
        "The team has been successfully updated.",
        style(format!(
            "{}/teams/{}?tab=settings",
            Config::new().webapp_url,
            team_id
        ))
        .with(Color::Rgb {
            r: 0,
            g: 135,
            b: 255,
        })
        .to_string()
    );

    std::process::exit(0);
}
