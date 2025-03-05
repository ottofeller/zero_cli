mod graphql;

use crate::common::{
    authorization_headers::authorization_headers,
    config::Config,
    execute_graphql_request::execute_graphql_request,
    keyring::keyring,
    lengify::lengify,
    pad_to_column_width::pad_to_column_width,
    print_formatted_error::print_formatted_error,
    table::table,
};

use crate::projects::common::project_info_by_slug::project_info_by_slug;
use crate::secrets::list::graphql::project_secrets::{project_secrets, ProjectSecrets};
use clap::Args;
use graphql_client::GraphQLQuery;
use reqwest::Client;
use termimad::crossterm::style::Stylize;

#[derive(Args, Debug)]
pub struct SecretsListArgs {
    #[clap(
        short,
        long,
        help = "Project slug"
    )]
    slug: String,
    #[clap(
        short,
        long,
        help = "Access token, if not specified, the token will be taken from the keychain"
    )]
    access_token: Option<String>,
}

pub fn list(args: &SecretsListArgs) {
    let access_token = match &args.access_token {
        Some(token) => token.clone(),
        None => keyring::get("access_token"),
    };

    let project_info = project_info_by_slug(&args.slug, &access_token);
    let date_format = &Config::new().date_format;

    let project_secrets_error_message = format!(
        "Failed to retrieve secrets for the project with slug '{}'.",
        &args.slug
    );

    let project_secrets_response =
        match execute_graphql_request::<project_secrets::Variables, project_secrets::ResponseData>(
            authorization_headers(&access_token),
            ProjectSecrets::build_query,
            &Client::new(),
            &project_secrets_error_message,
            project_secrets::Variables { id: project_info.id },
        )
        .project_by_pk
        {
            Some(user_secrets) => user_secrets,

            None => {
                print_formatted_error(&project_secrets_error_message);
                std::process::exit(1);
            }
        };

    let mut secrets_list = Vec::new();

    struct ColumnWidthSize {
        slug: usize,
        vendor: usize,
        date: usize,
    }

    let mut column_width_size = ColumnWidthSize {
        slug: 5,
        vendor: 6,
        date: 4,
    };

    let indentation = 2;

    // save the length of the longest column element
    for secret in &project_secrets_response.user_secrets {
        column_width_size.slug = column_width_size
            .slug
            .max(secret.slug.len());

        column_width_size.vendor = column_width_size
            .vendor
            .max(secret.vendor.to_string().len());

        column_width_size.date = column_width_size
            .vendor
            .max(secret.updated_at.format(date_format).to_string().len());
    }

    for secret in project_secrets_response.user_secrets {
        secrets_list.push(format!(
            "{}{}{}{}",
            pad_to_column_width(
                format!("{}", &secret.slug),
                column_width_size.slug + indentation
            )
            .green(),
            lengify(&secret.name),
            pad_to_column_width(
                secret.vendor.to_string(),
                column_width_size.vendor + indentation
            ),
            pad_to_column_width(
                secret.updated_at.format(date_format).to_string(),
                column_width_size.date + indentation,
            )
        ));
    }

    table(
        secrets_list,
        format!("Secrets of the '{}' project", project_secrets_response.name),
        "You don't have any secrets on this project.",
        format!(
            "{}{}{}{}",
            pad_to_column_width("SLUG".to_string(), column_width_size.slug + indentation),
            lengify("NAME"),
            pad_to_column_width("VENDOR".to_string(), column_width_size.vendor + indentation),
            pad_to_column_width("DATE".to_string(), column_width_size.date + indentation)
        ),
    )
}
