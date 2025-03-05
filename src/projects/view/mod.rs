use clap::Args;
mod graphql;
use crate::common::{
    authorization_headers::authorization_headers,
    config::Config,
    execute_graphql_request::execute_graphql_request,
    format_relative_time::format_relative_time,
    keyring::keyring,
    print_formatted_error::print_formatted_error,
};
use crate::projects::common::project_info_by_slug::project_info_by_slug;
use crate::projects::view::graphql::project_details::{project_details, ProjectDetails};
use graphql_client::GraphQLQuery;
use reqwest::Client;
use termimad::{
    crossterm::style::{style, Color, Stylize},
    MadSkin,
};

#[derive(Args, Debug)]
pub struct ProjectsViewArgs {
    #[clap(short, long, help = "Project slug")]
    slug: String,
    #[clap(
        short,
        long,
        help = "Access token, if not specified, the token will be taken from the keychain"
    )]
    access_token: Option<String>,
}

pub fn view(args: &ProjectsViewArgs) {
    let access_token = match &args.access_token {
        Some(token) => token.clone(),
        None => keyring::get("access_token"),
    };

    let project_info = project_info_by_slug(&args.slug, &access_token);

    let project_details_error_message = format!(
        "Failed to retrieve detailed information for the project with slug '{}'.",
        &args.slug
    );

    let project_details =
        execute_graphql_request::<project_details::Variables, project_details::ResponseData>(
            authorization_headers(&access_token),
            ProjectDetails::build_query,
            &Client::new(),
            &project_details_error_message,
            project_details::Variables {
                id: project_info.id,
            },
        )
        .project;

    let current_project_details = match project_details.first() {
        Some(data) => data,

        None => {
            print_formatted_error(&project_details_error_message);
            std::process::exit(1);
        }
    };

    let last_usage = if let Some(usage_history) = &current_project_details.usage_histories.first() {
        match format_relative_time(&usage_history.created_at.to_string()) {
            Ok(relative_time) => relative_time,

            Err(_) => {
                print_formatted_error("Failed to parse the last usage time. Please try again.");
                std::process::exit(1);
            }
        }
    } else {
        "never used".to_string().grey()
    };

    let skin = MadSkin {
        ..Default::default()
    };

    let markdown_text = format!(
        "**Name**: {}\n**Slug**: {}\n**Secrets**: {}\n**Integrations**: {}\n**Team**: {}\n**URL**: {}\n**Last used**: {}",
        &project_info.name,
        &project_info.slug,
        if let Some(secret_count) = &current_project_details.user_secrets_aggregate.aggregate {
            secret_count.count
        } else {
            0
        },
        if let Some(integrations_count) = &current_project_details.integration_installations_aggregate.aggregate {
            integrations_count.count
        } else {
            0
        },
        match &project_info.team {
            Some(team) => team.name.clone(),
            None => {
                print_formatted_error("Project must belong to a team");
                std::process::exit(1);
            }
        },
        style(format!("{}/{}/{}", Config::new().webapp_url,
        match &project_info.team {
            Some(team) => team.slug.clone(),
            None => {
                print_formatted_error("Project must belong to a team");
                std::process::exit(1);
            }
        },
        &project_info.slug)).with(Color::Rgb {
            r: 0,
            g: 135,
            b: 255,
        }),
        last_usage,
    );

    skin.print_text(&markdown_text);
}
