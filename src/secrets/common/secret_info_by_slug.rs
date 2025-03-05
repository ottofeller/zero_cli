use crate::common::{
    authorization_headers::authorization_headers, colorful_theme::theme, config::Config,
    execute_graphql_request::execute_graphql_request, print_formatted_error::print_formatted_error,
};
use crate::secrets::common::graphql::secret_info::{secret_info, SecretInfo};
use dialoguer::Select;
use graphql_client::GraphQLQuery;
use reqwest::Client;
use termimad::crossterm::style::Stylize;

/// Retrieves secret information by its slug and handles multiple secret matches.
///
/// This function fetches secret information, and if multiple secrets
/// are found with the same slug, presents an interactive selection menu to the user.
///
/// # Arguments
/// * `slug` - The secret slug to search for
/// * `access_token` - The authentication token for API access
///
/// # Returns
/// The selected secret's information
///
/// # Examples
/// ```
/// let secret = secret_info_by_slug(
///     &String::from("my-secret"),
///     &String::from("access-token")
/// );
/// println!("Secret name: {}", secret.name);
/// ```
pub fn secret_info_by_slug(
    slug: &String,
    access_token: &String,
) -> secret_info::SecretInfoUserSecret {
    let client = Client::new();
    let config = Config::new();
    let items_per_page = config.items_per_page;
    let authorization_headers = authorization_headers(&access_token);
    let secret_info_error_message = "Failed to retrieve secret info.";

    let secret_info_response =
        execute_graphql_request::<secret_info::Variables, secret_info::ResponseData>(
            authorization_headers.clone(),
            SecretInfo::build_query,
            &client,
            &secret_info_error_message,
            secret_info::Variables { slug: slug.clone() },
        )
        .user_secret;

    let options: Vec<String> = secret_info_response
        .iter()
        .map(|secret| {
            format!(
                "{} (Team: {}, Project: {})",
                secret.name,

                match &secret.project.team {
                    Some(team) => team.name.clone(),

                    None => {
                        print_formatted_error("Secret must belong to a project");
                        std::process::exit(1);
                    }
                },

                &secret.project.name
            )
        })
        .collect();

    let selected_index = if options.len() < 1 {
        print_formatted_error(&format!("Secret with slug '{}' not found.", &slug));
        std::process::exit(1);
    } else if options.len() == 1 {
        0
    } else {
        match Select::with_theme(&theme())
            .with_prompt(format!(
                "Select the secret: {}",
                "Use <Up>/<Down> to navigate and <Enter>/<Space> to select".dark_grey()
            ))
            .default(0)
            .items(&options)
            .max_length(items_per_page)
            .interact()
        {
            Ok(selected_index) => selected_index,

            Err(_) => {
                print_formatted_error("Failed to read expiration time.");
                std::process::exit(1);
            }
        }
    };

    return secret_info_response[selected_index].clone();
}
