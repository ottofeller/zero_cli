pub mod graphql;
use crate::common::{
    authorization_headers::authorization_headers,
    colorful_theme::theme,
    config::Config,
    execute_graphql_request::execute_graphql_request,
    graphql::insert_secret_sharing_record::{
        insert_secret_sharing_record, InsertSecretSharingRecord,
    },
    keyring::keyring,
    print_formatted_error::print_formatted_error,
};
use crate::projects::common::project_info_by_slug::project_info_by_slug;
use crate::projects::share::graphql::project_secrets_ids::{
    project_secrets_ids, ProjectSecretsIds,
};
use chrono::{Duration, Utc};
use clap::Args;
use dialoguer::{MultiSelect, Password, Select};
use graphql_client::GraphQLQuery;
use reqwest::Client;
use termimad::crossterm::style::{Color, Stylize};

#[derive(Args, Debug)]
pub struct ProjectsShareArgs {
    #[clap(short, long, help = "Project slug")]
    slug: String,
    #[clap(
        short,
        long,
        help = "Access token, if not specified, the token will be taken from the keychain"
    )]
    access_token: Option<String>,
}

pub fn share(args: &ProjectsShareArgs) {
    let config = Config::new();
    let items_per_page = config.items_per_page;

    let access_token = match &args.access_token {
        Some(token) => token.clone(),
        None => keyring::get("access_token"),
    };

    let authorization_headers = authorization_headers(&access_token);
    let share_project_info = project_info_by_slug(&args.slug, &access_token);

    let user_secrets_error_message = format!(
        "Sharing failed. No secrets were found for the '{}' project.",
        &args.slug.clone()
    );

    let project = match execute_graphql_request::<
        project_secrets_ids::Variables,
        project_secrets_ids::ResponseData,
    >(
        authorization_headers.clone(),
        ProjectSecretsIds::build_query,
        &Client::new(),
        &user_secrets_error_message,
        project_secrets_ids::Variables {
            project_id: share_project_info.id.clone(),
        },
    )
    .project_by_pk
    {
        Some(project) => project,

        None => {
            print_formatted_error(&user_secrets_error_message);
            std::process::exit(1);
        }
    };

    let user_secrets = match project.user_secrets.len() {
        1.. => &project.user_secrets,

        _ => {
            print_formatted_error(&user_secrets_error_message);
            std::process::exit(1);
        }
    };

    let passphrase = match Password::with_theme(&theme())
        .with_prompt("Type a passphrase of at least 6 character:")
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.trim().chars().count() >= 6 {
                Ok(())
            } else {
                print_formatted_error(
                    "Sharing failed. Passphrase must be at least 6 character long.",
                );

                std::process::exit(1);
            }
        })
        .interact()
    {
        Ok(passphrase) => passphrase,

        Err(_) => {
            print_formatted_error("Sharing failed. Failed to read passphrase.");
            std::process::exit(1);
        }
    };

    let options = ["5 min", "30 min", "1 hour", "1 day"];

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
            print_formatted_error("Sharing failed. Failed to read expiration time.");
            std::process::exit(1);
        }
    };

    let expires_at_minutes = match options[expires_at_selections] {
        "5 min" => 5,
        "30 min" => 30,
        "1 hour" => 60,
        "1 day" => 1440,

        _ => {
            print_formatted_error("Sharing failed. Failed to read expiration time.");
            std::process::exit(1);
        }
    };

    let secrets_selections = match MultiSelect::with_theme(&theme())
        .with_prompt(format!(
            "Pick secrets: {}",
            "Use <Up>/<Down> to navigate, <Space> to select, and <Enter> to confirm".dark_grey()
        ))
        .items(
            &user_secrets
                .iter()
                .map(|user_secret| user_secret.name.to_string())
                .collect::<Vec<String>>(),
        )
        .max_length(items_per_page)
        .interact()
    {
        Ok(secrets_selections) => secrets_selections,

        Err(_) => {
            print_formatted_error("Sharing failed. Failed to read secrets.");
            std::process::exit(1);
        }
    };

    if secrets_selections.is_empty() {
        print_formatted_error("Sharing failed. You must select at least one secret.");
        std::process::exit(1);
    }

    let user_secret_fields_ids_vec: Vec<String> = user_secrets
        .iter()
        .filter_map(|user_secret| {
            secrets_selections
                .iter()
                .find(|&selection| user_secrets[*selection].name == user_secret.name)
                .map(|_| user_secret.fields.iter().map(|field| field.id.to_string()))
        })
        .flatten()
        .collect();

    let secrets_sharing_id = execute_graphql_request::<
        insert_secret_sharing_record::Variables,
        insert_secret_sharing_record::ResponseData,
    >(
        authorization_headers.clone(),
        InsertSecretSharingRecord::build_query,
        &Client::new(),
        &format!(
            "Sharing failed. Failed to generate a secret sharing url for the project '{}'.",
            &args.slug.clone()
        ),
        insert_secret_sharing_record::Variables {
            expires_at: (Utc::now() + Duration::minutes(expires_at_minutes)).to_rfc3339(),
            passphrase: passphrase,
            secrets_field_ids: user_secret_fields_ids_vec,
        },
    )
    .insert_secret_sharing_record
    .id;

    let secrets_sharing_url = format!("{}/private/{}", config.webapp_url, secrets_sharing_id);

    println!(
        "Your link with secrets for the '{}' project",
        &share_project_info.slug.clone().dark_cyan()
    );

    println!(
        "{}",
        secrets_sharing_url
            .with(Color::Rgb {
                r: 0,
                g: 135,
                b: 255,
            })
            .to_string()
    );
}
