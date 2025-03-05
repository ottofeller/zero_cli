mod graphql;
use crate::common::slugify::slugify_prompt;

use crate::common::{
    authorization_headers::authorization_headers, colorful_theme::theme, config::Config,
    execute_graphql_request::execute_graphql_request, keyring::keyring,
    print_formatted_error::print_formatted_error, validate_name::validate_name,
};

use chrono::{Duration, Utc};
use clap::Args;
use dialoguer::{console::Style, theme::ColorfulTheme, Confirm, Input, Select, Sort};

use graphql::{
    create_project::{create_project, CreateProject},
    team_slug::{team_slug, TeamSlug},
};

use graphql_client::GraphQLQuery;
use reqwest::Client;

use termimad::{
    crossterm::style::{style, Color, Stylize},
    minimad, MadSkin,
};

#[derive(Args, Debug)]
pub struct ProjectsCreateArgs {
    #[clap(
        short,
        long,
        help = "Project name, if not specified, the user will be prompted to enter the name"
    )]
    name: Option<String>,
    #[clap(
        short,
        long,
        help = "Access token, if not specified, the token will be taken from the keychain"
    )]
    access_token: Option<String>,
}

pub fn create(args: &ProjectsCreateArgs) {
    let items_per_page = Config::new().items_per_page;

    let access_token = match &args.access_token {
        Some(token) => token.clone(),
        None => keyring::get("access_token"),
    };

    let client = Client::new();
    let headers = authorization_headers(&access_token);

    let project_name = match &args.name {
        Some(name) => {
            let _ = validate_name(name);
            name
        }

        None => {
            match Input::with_theme(&theme())
                .with_prompt("Type a project name:")
                .validate_with(|input: &String| -> Result<(), &str> { validate_name(&input) })
                .interact()
            {
                Ok(new_name) => &new_name.clone(),

                Err(_) => {
                    print_formatted_error("Creation failed. Failed to read the project name.");
                    std::process::exit(1);
                }
            }
        }
    };

    let is_token_needed = match Confirm::with_theme(&theme())
        .with_prompt("Do you want to generate a new token for this project?")
        .interact()
    {
        Ok(token_needed) => token_needed,

        Err(_) => {
            print_formatted_error("Creation failed. Failed to read the token generation option.");
            std::process::exit(1);
        }
    };

    let token;

    if is_token_needed {
        let token_name = match Input::with_theme(&theme())
            .with_prompt("Enter the token name:")
            .validate_with(|input: &String| -> Result<(), &str> {
                if input.trim().chars().count() == 0 {
                    return Err("The token name must contain at least 1 character.");
                } else {
                    Ok(())
                }
            })
            .interact()
        {
            Ok(new_token_name) => new_token_name,

            Err(_) => {
                print_formatted_error("Creation failed. Failed to read the token name.");
                std::process::exit(1);
            }
        };

        let options = ["Endless", "7 days", "30 days", "60 days", "90 days"];

        let expires_at_selections = match Select::with_theme(&theme())
            .with_prompt(format!(
                "Expires in: {}",
                "Use <Up>/<Down> to navigate and <Enter>/<Space> to select".dark_grey()
            ))
            .default(0)
            .items(
                &options
                    .iter()
                    .map(|item| item.to_string())
                    .collect::<Vec<String>>(),
            )
            .max_length(items_per_page)
            .interact()
        {
            Ok(expires_at_selections) => expires_at_selections,

            Err(_) => {
                print_formatted_error("Creation failed. Failed to read expiration time.");
                std::process::exit(1);
            }
        };

        let token_expires_at = match options[expires_at_selections] {
            "Endless" => None,
            "7 days" => Some((Utc::now() + Duration::days(7)).to_string()),
            "30 days" => Some((Utc::now() + Duration::days(30)).to_string()),
            "60 days" => Some((Utc::now() + Duration::days(60)).to_string()),
            "90 days" => Some((Utc::now() + Duration::days(90)).to_string()),

            _ => {
                print_formatted_error("Creation failed. Failed to read expiration time.");
                std::process::exit(1);
            }
        };

        token = Some(create_project::TokenObject {
            expires_at: token_expires_at,
            name: token_name,
        });
    } else {
        token = None;
    }

    let create_project_error_message = format!(
        "Creation failed. Failed to create a project with the name '{}'.",
        project_name.clone()
    );

    let first_letters: String = project_name
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .take(2)
        .flat_map(|c| c.to_uppercase())
        .collect();

    // Create a project
    let project_response =
        execute_graphql_request::<create_project::Variables, create_project::ResponseData>(
            headers.clone(),
            CreateProject::build_query,
            &client,
            &create_project_error_message,
            create_project::Variables {
                icon: format!(
                    ">{}#{:02x}{:02x}{:02x}",
                    first_letters,
                    rand::random::<u8>(),
                    rand::random::<u8>(),
                    rand::random::<u8>()
                ),
                slug: slugify_prompt(&project_name, "Type a slug for the project:"),
                name: project_name.clone(),
                token,
            },
        )
        .create_project;

    let project_team_error_message = format!(
        "Creation failed. Failed to create a project with the name '{}'.",
        project_name.clone()
    );

    let project_team_id = match &project_response.project {
        Some(project) => project.team_id,

        None => {
            print_formatted_error("Project must belong to a team");
            std::process::exit(1);
        }
    };

    let project_team_response =
        execute_graphql_request::<team_slug::Variables, team_slug::ResponseData>(
            headers.clone(),
            TeamSlug::build_query,
            &client,
            &project_team_error_message,
            team_slug::Variables {
                id: project_team_id.clone(),
            },
        )
        .team_by_pk;

    let project_team_slug = match project_team_response {
        Some(team_info) => team_info.slug,

        None => {
            print_formatted_error("Project must belong to a team");
            std::process::exit(1);
        }
    };

    // MD template for the project creation message
    let text_template = if is_token_needed {
        minimad::TextTemplate::from(
            r#"
        ##### ✔ Project successfully created
        ${description}
        **Project link**: ${project-link}
        **Project slug for CLI**: ${project-slug}

        > *Please make sure to store this token in a safe place.*
        > *If you ever lose or forget this token, you can regenerate it.*
        > *However, be aware that any scripts or applications using this token will need to be updated accordingly.*
        "#,
        )
    } else {
        minimad::TextTemplate::from(
            r#"
        ##### ✔ Project successfully created
        ${description}
        **Project link**: ${project-link}
        **Project slug for CLI**: ${project-slug}
        "#,
        )
    };

    let mut expander = text_template.expander();

    let project_slug = match project_response.project {
        Some(project) => project.slug.clone(),

        None => {
            print_formatted_error(&create_project_error_message);
            std::process::exit(1);
        }
    };

    let description = format!("You created the project named '{}'.", &project_name);

    let project_link = style(format!(
        "{}/{}/{}",
        Config::new().webapp_url,
        &project_team_slug,
        &project_slug
    ))
    .with(Color::Rgb {
        r: 0,
        g: 135,
        b: 255,
    })
    .to_string();

    let styled_slug: String = format!("{}", &project_slug.with(Color::Green));

    expander
        .set("description", &description)
        .set("project-link", &project_link)
        .set("project-slug", &styled_slug);

    let mut skin = MadSkin::default();
    skin.headers[4].set_fg(Color::Green);
    skin.print_expander(expander);

    let sort_theme = ColorfulTheme {
        active_item_style: Style::new(),
        picked_item_prefix: dialoguer::console::style("".to_string()),
        unpicked_item_prefix: dialoguer::console::style("".to_string()),
        prompt_prefix: dialoguer::console::style("".to_string()),
        ..ColorfulTheme::default()
    };

    if let Some(token_value) = project_response.token_value {
        match Sort::with_theme(&sort_theme)
            .item(format!(
                "{} '{}'. {}",
                "Your token:",
                style(format!("{}", &token_value)).with(Color::Green),
                "Copy this token and press 'Enter' to remove it from the screen."
            ))
            .clear(true)
            .report(false)
            .interact()
        {
            Ok(confirmation) => confirmation,

            Err(_) => {
                print_formatted_error("Failed to display the project on the screen. You can see it in the web application or try creating it again.");
                std::process::exit(1);
            }
        };
    }
}
