mod graphql;

use crate::common::{
    authorization_headers::authorization_headers, execute_graphql_request::execute_graphql_request,
    keyring::keyring, print_formatted_error::print_formatted_error, read_env_file::read_env_file,
    write_env_file::write_env_file,
};

use crate::secrets::common::secret_info_by_slug::secret_info_by_slug;
use clap::Args;
use graphql::view_secret::{view_secret, ViewSecret};
use graphql_client::GraphQLQuery;
use reqwest::Client;
use termimad::crossterm::style::Stylize;

#[derive(Args, Debug)]
pub struct SecretDropArgs {
    #[clap(short, long, help = "Secret slug")]
    slug: String,
    #[clap(short, long, help = "Path to the env file")]
    env_file: String,
    #[clap(
        short,
        long,
        help = "Field key to drop (If not specified, all fields will be dropped)"
    )]
    field_key: Option<String>,
    #[clap(
        short,
        long,
        help = "Access token, if not specified, the token will be taken from the keychain"
    )]
    access_token: Option<String>,
}

pub fn drop(args: &SecretDropArgs) {
    let access_token = match &args.access_token {
        Some(token) => token.clone(),
        None => keyring::get("access_token"),
    };

    let secret_info = secret_info_by_slug(&args.slug, &access_token);

    let user_secret_fields: Vec<view_secret::ViewSecretViewSecretFieldsSecretFields> =
        execute_graphql_request::<view_secret::Variables, view_secret::ResponseData>(
            authorization_headers(&access_token),
            ViewSecret::build_query,
            &Client::new(),
            "Dump failed. Failed to retrieve the user's secret field value.",
            view_secret::Variables {
                secret_id: secret_info.id.to_string(),
            },
        )
        .view_secret_fields
        .secret_fields;

    if user_secret_fields.len() == 0 {
        print_formatted_error("Dump failed. The user's secret has no fields.");
        std::process::exit(1);
    }

    let mut new_fields = read_env_file(&args.env_file);

    // If the field key is specified, only drop that field.
    if let Some(field_key) = &args.field_key {
        let user_secret_field = user_secret_fields
            .iter()
            .find(|field| &field.key == field_key)
            .unwrap_or_else(|| {
                print_formatted_error(&format!(
                    "Dump failed. The '{}' field was not found.",
                    field_key
                ));

                std::process::exit(1);
            });

        new_fields.insert(
            user_secret_field.key.clone(),
            user_secret_fields
                .iter()
                .find(|field| field.key.eq(field_key))
                .unwrap()
                .value
                .clone(),
        );

        // If the field key is not specified, drop all fields.
    } else {
        user_secret_fields.iter().for_each(|field| {
            new_fields.insert(field.key.clone(), field.value.clone());
        });
    };

    write_env_file(&args.env_file, &new_fields);
    println!("{} {}", "✓".green(), "Secret fields dropped successfully.");
}
