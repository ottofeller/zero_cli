mod graphql;
use crate::common::{
    authorization_headers::authorization_headers, colorful_theme::theme,
    execute_graphql_request::execute_graphql_request, keyring::keyring,
    print_formatted_error::print_formatted_error,
};
use crate::projects::common::project_info_by_slug::project_info_by_slug;
use crate::projects::delete::graphql::delete_project::{delete_project, DeleteProject};
use clap::Args;
use dialoguer::Input;
use graphql_client::GraphQLQuery;
use reqwest::Client;
use termimad::crossterm::style::Stylize;

#[derive(Args, Debug)]
pub struct ProjectsDeleteArgs {
    #[clap(short, long, help = "Project slug")]
    slug: String,
    #[clap(
        short,
        long,
        help = "Access token, if not specified, the token will be taken from the keychain"
    )]
    access_token: Option<String>,
}

pub fn delete(args: &ProjectsDeleteArgs) {
    let access_token = match &args.access_token {
        Some(token) => token.clone(),
        None => keyring::get("access_token"),
    };

    let authorization_headers = authorization_headers(&access_token);
    let delete_project_info = project_info_by_slug(&args.slug, &access_token);

    match Input::with_theme(&theme())
        .with_prompt(format!(
            "Type {} to confirm deletion:",
            args.slug.clone().green()
        ))
        .validate_with(|input: &String| -> Result<(), &str> {
            if input == &args.slug.clone() {
                Ok(())
            } else {
                Err("Your reply was invalid.")
            }
        })
        .interact()
    {
        Ok(value) => value.trim().to_string(),

        Err(_) => {
            print_formatted_error("Deletion failed. Failed to read the user's reply.");
            std::process::exit(1);
        }
    };

    let delete_project_error_message = format!(
        "Deletion failed. Failed to delete a project '{}'.",
        args.slug.clone()
    );

    let delete_project_response =
        execute_graphql_request::<delete_project::Variables, delete_project::ResponseData>(
            authorization_headers.clone(),
            DeleteProject::build_query,
            &Client::new(),
            &delete_project_error_message,
            delete_project::Variables {
                id: delete_project_info.id.to_string(),
            },
        )
        .delete_project
        .id;

    if delete_project_response.to_string().trim().is_empty() {
        print_formatted_error(&delete_project_error_message);
        std::process::exit(1);
    }

    println!("{} Project successfully deleted.", "✔".green());
}
