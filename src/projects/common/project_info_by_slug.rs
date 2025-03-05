use crate::common::{
    authorization_headers::authorization_headers, colorful_theme::theme, config::Config,
    execute_graphql_request::execute_graphql_request, print_formatted_error::print_formatted_error,
};
use crate::projects::common::graphql::project_info::{project_info, ProjectInfo};
use dialoguer::Select;
use graphql_client::GraphQLQuery;
use reqwest::Client;
use termimad::crossterm::style::Stylize;

/// Retrieves project information by its slug and handles multiple project matches.
///
/// This function fetches project information, and if multiple projects
/// are found with the same slug, presents an interactive selection menu to the user.
///
/// # Arguments
/// * `slug` - The project slug to search for
/// * `access_token` - The authentication token for API access
///
/// # Returns
/// The selected project's information
///
/// # Examples
/// ```
/// let project = project_info_by_slug(
///     &String::from("my-project"),
///     &String::from("access-token")
/// );
/// println!("Project name: {}", project.name);
/// ```
pub fn project_info_by_slug(
    slug: &String,
    access_token: &String,
) -> project_info::ProjectInfoProject {
    let client = Client::new();
    let config = Config::new();
    let items_per_page = config.items_per_page;
    let authorization_headers = authorization_headers(&access_token);
    let project_info_error_message = "Failed to retrieve project info.";

    let project_info_response =
        execute_graphql_request::<project_info::Variables, project_info::ResponseData>(
            authorization_headers.clone(),
            ProjectInfo::build_query,
            &client,
            &project_info_error_message,
            project_info::Variables { slug: slug.clone() },
        )
        .project;

    let options: Vec<String> = project_info_response
        .iter()
        .map(|project| {
            format!(
                "{} (Team: {})",
                project.name,
                match &project.team {
                    Some(team) => team.name.clone(),

                    None => {
                        print_formatted_error("Project must belong to a team");
                        std::process::exit(1);
                    }
                }
            )
        })
        .collect();

    let selected_index = if options.len() < 1 {
        print_formatted_error(&format!("Project with slug '{}' not found.", &slug));
        std::process::exit(1);
    } else if options.len() == 1 {
        0
    } else {
        match Select::with_theme(&theme())
            .with_prompt(format!(
                "Select the project: {}",
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

    return project_info_response[selected_index].clone();
}
