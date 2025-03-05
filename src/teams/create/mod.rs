mod graphql;

use crate::common::slugify::slugify_prompt;
use crate::common::{
    authorization_headers::authorization_headers, colorful_theme::theme, config::Config,
    execute_graphql_request::execute_graphql_request, keyring::keyring,
    print_formatted_error::print_formatted_error, validate_name::validate_name,
};
use crate::teams::create::graphql::create_team::{create_team, CreateTeam};
use clap::Args;
use dialoguer::Input;
use graphql_client::GraphQLQuery;
use reqwest::Client;
use termimad::{
    crossterm::style::{style, Color, Stylize},
    minimad, MadSkin,
};

#[derive(Args, Debug)]
pub struct TeamsCreateArgs {
    #[clap(short, long, help = "The name of the team, not required")]
    name: Option<String>,
    #[clap(
        short,
        long,
        help = "Access token, if not specified, the token will be taken from the keychain"
    )]
    access_token: Option<String>,
}

pub fn create(args: &TeamsCreateArgs) {
    let access_token = match &args.access_token {
        Some(token) => token.clone(),
        None => keyring::get("access_token"),
    };

    let client = Client::new();
    let headers = authorization_headers(&access_token);

    let team_name = if let Some(name) = &args.name {
        if let Err(err) = validate_name(&name) {
            eprintln!("Validation error: {}", err);
            print_formatted_error(&format!("Validation error: {}", err));
            std::process::exit(1);
        } else {
            name.trim().to_owned()
        }
    } else {
        match Input::with_theme(&theme())
            .with_prompt("Type a name for the team:")
            .validate_with(|input: &String| -> Result<(), &str> { validate_name(input) })
            .interact()
        {
            Ok(name) => name.trim().to_owned(),

            Err(_) => {
                print_formatted_error("Failed to read team name input.");
                std::process::exit(1);
            }
        }
    };

    let team_slug = slugify_prompt(&team_name, "Type a slug for the team:");

    execute_graphql_request::<create_team::Variables, create_team::ResponseData>(
        headers.clone(),
        CreateTeam::build_query,
        &client,
        &format!(
            "Creation failed. Failed to create a team with the name '{}'.",
            team_name
        ),
        create_team::Variables {
            slug: team_slug.clone(),
            team_name: team_name.clone(),
        },
    );

    let text_template = minimad::TextTemplate::from(
        r#"
        ##### **✔ Team successfully created**
        Team link: ${team-link}
        Team name: **${team_name}**
        Team slug: **${team-slug}**
        "#,
    );

    let mut expander = text_template.expander();

    let team_link = style(format!(
        "{}/{}/projects",
        Config::new().webapp_url,
        &team_slug
    ))
    .with(Color::Rgb {
        r: 0,
        g: 135,
        b: 255,
    })
    .to_string();

    expander
        .set("team-link", &team_link)
        .set("team_name", &team_name)
        .set("team-slug", &team_slug);

    let mut skin = MadSkin::default();
    skin.bold.set_fg(Color::Green);
    skin.print_expander(expander);
}
