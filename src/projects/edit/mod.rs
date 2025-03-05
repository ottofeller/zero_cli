mod graphql;
use crate::common::{
    authorization_headers::authorization_headers, colorful_theme::theme, config::Config,
    execute_graphql_request::execute_graphql_request, keyring::keyring,
    print_formatted_error::print_formatted_error, slugify::slugify_prompt,
    validate_name::validate_name,
};
use crate::projects::common::project_info_by_slug::project_info_by_slug;
use crate::projects::edit::graphql::update_project_description::{
    update_project_description, UpdateProjectDescription,
};
use crate::projects::edit::graphql::update_project_name::{update_project_name, UpdateProjectName};
use clap::Args;
use dialoguer::{Confirm, Input};
use graphql_client::GraphQLQuery;
use reqwest::Client;
use termimad::crossterm::style::{style, Color, Stylize};

#[derive(Args, Debug)]
pub struct ProjectsEditArgs {
    #[clap(short, long, help = "Project slug")]
    slug: String,
    #[clap(short, long, help = "Project name, not required")]
    name: Option<String>,
    #[clap(short, long, help = "Project description, not required")]
    description: Option<String>,
    #[clap(
        short,
        long,
        help = "Access token, if not specified, the token will be taken from the keychain"
    )]
    access_token: Option<String>,
}

pub fn edit(args: &ProjectsEditArgs) -> () {
    let access_token = match &args.access_token {
        Some(token) => token.clone(),
        None => keyring::get("access_token"),
    };

    let authorization_headers = authorization_headers(&access_token);
    let mut name = args.name.clone();
    let mut description = args.description.clone();

    // If name is passed as an argument, it must be at least 2 characters long
    if name.clone().is_some() && name.clone().unwrap().chars().count() < 2 {
        print_formatted_error(
            "Editing failed. The project name must be at least 2 characters long.",
        );
        std::process::exit(1);
    }

    let client = Client::new();
    let edit_project_info = project_info_by_slug(&args.slug, &access_token);

    // If no arguments are passed, the user will be prompted to enter a new name and description of the project
    if name.is_none() && description.is_none() {
        name = match Input::with_theme(&theme())
            .with_prompt("Type a new project name:")
            .validate_with(|name: &String| -> Result<(), &str> { validate_name(&name) })
            .default(edit_project_info.name.clone())
            .interact()
        {
            Ok(new_name) => Some(new_name),

            Err(_) => {
                print_formatted_error("Editing failed. Failed to get the project name.");
                std::process::exit(1);
            }
        };

        let is_confirmed = match Confirm::new()
            .with_prompt("Do you want to update the project description?")
            .interact()
        {
            Ok(confirmation) => confirmation,

            Err(_) => {
                print_formatted_error("Editing failed. Failed to get the user confirmation.");
                std::process::exit(1);
            }
        };

        if is_confirmed {
            description = match Input::with_theme(&theme())
                .with_prompt("Type a new project description:")
                .interact()
            {
                Ok(new_description) => Some(new_description),

                Err(_) => {
                    print_formatted_error("Editing failed. Failed to get the project description.");
                    std::process::exit(1);
                }
            };
        }
    }

    if let Some(name) = name {
        let update_project_name_error_message = format!(
            "Editing failed. Failed to update the name of the project '{}'.",
            args.slug.clone()
        );

        match execute_graphql_request::<
            update_project_name::Variables,
            update_project_name::ResponseData,
        >(
            authorization_headers.clone(),
            UpdateProjectName::build_query,
            &client,
            &update_project_name_error_message,
            update_project_name::Variables {
                id: edit_project_info.id.clone(),
                name: name.clone(),
                slug: slugify_prompt(&args.slug, "Type a slug for the project:"),
            },
        )
        .update_project
        {
            Some(data) => {
                if data.affected_rows != 1 {
                    print_formatted_error(&update_project_name_error_message);
                    std::process::exit(1);
                }
            }

            None => {
                print_formatted_error(&update_project_name_error_message);
                std::process::exit(1);
            }
        };
    }

    if let Some(description) = description {
        let update_project_description_error_message = format!(
            "Editing failed. Failed to update the description of the project '{}'.",
            args.slug.clone()
        );

        match execute_graphql_request::<
            update_project_description::Variables,
            update_project_description::ResponseData,
        >(
            authorization_headers.clone(),
            UpdateProjectDescription::build_query,
            &client,
            &update_project_description_error_message,
            update_project_description::Variables {
                project_id: edit_project_info.id.clone(),
                project_description: description.clone(),
            },
        )
        .update_project
        {
            Some(data) => {
                if data.affected_rows != 1 {
                    print_formatted_error(&update_project_description_error_message);
                    std::process::exit(1);
                }
            }

            None => {
                print_formatted_error(&update_project_description_error_message);
                std::process::exit(1);
            }
        };
    }

    println!(
        "{} {}",
        "✔".green(),
        "The project has been successfully updated."
    );

    println!(
        "{}",
        style(format!(
            "{}/{}/{}",
            Config::new().webapp_url,
            match &edit_project_info.team {
                Some(team) => {
                    team.slug.clone()
                }
                None => {
                    print_formatted_error("Project must belong to a team");
                    std::process::exit(1);
                }
            },
            &edit_project_info.slug
        ))
        .with(Color::Rgb {
            r: 0,
            g: 135,
            b: 255,
        })
    );
}
