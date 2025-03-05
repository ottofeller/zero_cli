mod graphql;

use crate::common::{
    authorization_headers::authorization_headers,
    colorful_theme::theme,
    config::Config,
    execute_graphql_request::execute_graphql_request,
    keyring::keyring,
    print_formatted_error::print_formatted_error,
    slugify::slugify_prompt,
    validate_name::validate_name,
    validate_secret_field_name::validate_secret_field_name,
    vendors::Vendors,
};

use crate::projects::common::project_info_by_slug::project_info_by_slug;

use crate::secrets::create::graphql::create_secret::{create_secret, CreateSecret};
use clap::Args;
use dialoguer::{Confirm, Input, Password, Select};
use graphql_client::GraphQLQuery;
use reqwest::Client;

use termimad::{
    crossterm::style::{style, Color, Stylize},
    minimad, MadSkin,
};

#[derive(Args, Debug)]
pub struct SecretsCreateArgs {
    #[clap(short, long, help = "Project slug")]
    slug: String,
    #[clap(
        short,
        long,
        help = "Secret name, not required, if not provided will be prompted"
    )]
    name: Option<String>,
    #[clap(
        short,
        long,
        help = "Access token, if not specified, the token will be taken from the keychain"
    )]
    access_token: Option<String>,
}

pub fn create(args: &SecretsCreateArgs) {
    let config = Config::new();

    let access_token = match &args.access_token {
        Some(token) => token.clone(),
        None => keyring::get("access_token"),
    };

    let client = Client::new();
    let headers = authorization_headers(&access_token);

    let secret_name = match &args.name {
        Some(name) => match validate_name(&name) {
            Ok(_) => name.clone(),

            Err(error) => {
                print_formatted_error(error);
                std::process::exit(1);
            }
        },

        None => {
            match Input::with_theme(&theme())
                .with_prompt("Type a name for the secret:")
                .validate_with(|input: &String| -> Result<(), &str> { validate_name(&input) })
                .interact()
            {
                Ok(name) => name,

                Err(_) => {
                    print_formatted_error("Creation failed. Failed to get a secret name.");
                    std::process::exit(1);
                }
            }
        }
    };

    let vendors = Vendors::new().prettified_vendor_options;

    let selected_vendor = match Select::with_theme(&theme())
        .with_prompt(format!(
            "Select a vendor: {}",
            "Use <Up>/<Down> to navigate and <Enter>/<Space> to select".dark_grey()
        ))
        .default(0)
        .items(&vendors)
        .max_length(config.items_per_page)
        .interact()
    {
        Ok(selected_index) => Vendors::vendor_normalize(vendors[selected_index]),

        Err(_) => {
            print_formatted_error("Creation failed. Failed to get a vendor.");
            std::process::exit(1);
        }
    };

    let mut secret_fields = Vec::new();
    let mut field_names = Vec::new();

    loop {
        let field_name = match Input::with_theme(&theme())
            .with_prompt("Type a field name:")
            .validate_with(|input: &String| -> Result<(), &str> {
                validate_secret_field_name(&input, "", &field_names)
            })
            .interact()
        {
            Ok(name) => {
                field_names.push(name.clone());
                name
            }

            Err(_) => {
                print_formatted_error("Creation failed. Failed to get a field name.");
                std::process::exit(1);
            }
        };

        let field_value = match Password::with_theme(&theme())
            .with_prompt("Type a field value:")
            .validate_with(|input: &String| -> Result<(), &str> {
                if input.trim().chars().count() > 0 {
                    Ok(())
                } else {
                    Err("You are trying to set an empty value.")
                }
            })
            .interact()
        {
            Ok(value) => value,
            Err(_) => {
                print_formatted_error("Creation failed. Failed to get a field value.");
                std::process::exit(1);
            }
        };

        secret_fields.push(create_secret::CreateSecretFieldInput {
            name: field_name.clone(),
            value: field_value,
        });

        let is_confirmed = match Confirm::new()
            .with_prompt("Do you want to add another field?")
            .interact()
        {
            Ok(confirmation) => confirmation,
            Err(_) => {
                print_formatted_error("Creation failed. Failed to get a confirmation.");
                std::process::exit(1);
            }
        };

        if !is_confirmed {
            break;
        }
    }

    let project_info = project_info_by_slug(&args.slug, &access_token);
    let secret_slug = slugify_prompt(&secret_name, "Type a slug for the secret:");

    let secret_id =
        execute_graphql_request::<create_secret::Variables, create_secret::ResponseData>(
            headers.clone(),
            CreateSecret::build_query,
            &client,
            "Failed to create a secret",
            create_secret::Variables {
                fields: secret_fields,
                secret: create_secret::CreateSecretInput {
                    name: secret_name.clone(),
                    slug: secret_slug.clone(),
                    project_id: project_info.id.to_string(),
                    vendor: selected_vendor.to_string(),
                },
            },
        )
        .create_secret
        .id;

    let text_template = minimad::TextTemplate::from(
        r#"
        ##### **✔ Secret successfully created**
        Secret link: ${secret-link}
        Secret ID: **${secret-id}**
        "#,
    );

    let mut expander = text_template.expander();

    let secret_link = style(format!(
        "{}/{}/{}/{}",
        config.webapp_url,
        match &project_info.team {
            Some(team) => team.slug.clone(),
            None => {
                print_formatted_error("Project must belong to a team");
                std::process::exit(1);
            }
        },
        &project_info.slug.to_string(),
        &secret_slug,
    ))
    .with(Color::Rgb {
        r: 0,
        g: 135,
        b: 255,
    })
    .to_string();

    expander
        .set("secret-link", &secret_link)
        .set("secret-id", &secret_id);

    let mut skin = MadSkin::default();
    skin.bold.set_fg(Color::Green);
    skin.print_expander(expander);
}
