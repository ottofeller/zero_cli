use crate::common::graphql::{
    search_project_by_id::{search_project_by_id, SearchProjectById},
    search_token_by_id::{search_token_by_id, SearchTokenById},
    search_usage_history_by_id::{search_usage_history_by_id, SearchUsageHistoryById},
    search_user_by_id::{search_user_by_id, SearchUserById},
};
use crate::common::{
    authorization_headers::authorization_headers, colorful_theme::theme, config::Config,
    execute_graphql_request::execute_graphql_request, print_formatted_error::print_formatted_error,
};
use dialoguer::Select;
use graphql_client::GraphQLQuery;
use reqwest::Client;
use termimad::crossterm::style::Stylize;
use uuid::Uuid;

pub enum QueryType {
    Project,
    Tokens,
    UsageHistory,
    User,
}

struct SelectItem {
    id: Uuid,
    description: String,
}

/// Retrieves an entity by its ID from the database.
///
/// This function searches for an entity based on the given `QueryType` enumeration and
/// the provided `short_id` (short ID). The ID must be at least 4 characters long, and the
/// function returns the UUID of the found entity. If multiple entities match the given ID,
/// an error will be displayed prompting for a full ID.
///
/// # Parameters
///
/// * `query_type`: The type of entity to search for, represented by the `QueryType` enum.
/// * `short_id`: The short ID of the entity to search for.
/// * `access_token`: User access token for request
///
/// # Returns
///
/// * A UUID representing the ID of the found entity.
///
/// # Errors
///
/// * If the provided `short_id` is shorter than 4 characters.
/// * If no entities match the given ID.
///
/// # Examples
///
/// ```rust
/// let uuid = query_full_id(QueryType::Project, "1234".to_string());
/// ```
///
pub fn query_full_id(query_type: QueryType, short_id: String, access_token: &str) -> Uuid {
    let config = Config::new();
    let min_id_length = config.min_id_length;
    let client = Client::new();
    let authorization_headers = authorization_headers(access_token);

    if short_id.len() < min_id_length {
        print_formatted_error(&format!(
            "The short ID should be at least {} characters long.",
            min_id_length
        ));

        std::process::exit(1);
    }

    let all_matches: Vec<SelectItem> = match query_type {
        QueryType::Project => {
            let project_error_message =
                format!("Failed to retrieve project with ID '{}'.", &short_id);

            let projects = execute_graphql_request::<
                search_project_by_id::Variables,
                search_project_by_id::ResponseData,
            >(
                authorization_headers.clone(),
                SearchProjectById::build_query,
                &client,
                &project_error_message,
                search_project_by_id::Variables {
                    id: short_id.clone(),
                },
            )
            .search_project_by_id;

            projects
                .iter()
                .map(|element| SelectItem {
                    id: element.id,
                    description: element.name.clone(),
                })
                .collect()
        }

        QueryType::UsageHistory => {
            let usage_history_error_message =
                format!("Failed to retrieve usage history with ID '{}'.", &short_id);

            let usage_histories = execute_graphql_request::<
                search_usage_history_by_id::Variables,
                search_usage_history_by_id::ResponseData,
            >(
                authorization_headers.clone(),
                SearchUsageHistoryById::build_query,
                &client,
                &usage_history_error_message,
                search_usage_history_by_id::Variables {
                    id: short_id.clone(),
                },
            )
            .search_usage_history_by_id;

            usage_histories
                .iter()
                .map(|element| SelectItem {
                    id: element.id,
                    description: element.created_at.format(&config.date_format).to_string(),
                })
                .collect()
        }

        QueryType::User => {
            let user_error_message = format!("Failed to retrieve user with ID '{}'.", &short_id);

            let users = execute_graphql_request::<
                search_user_by_id::Variables,
                search_user_by_id::ResponseData,
            >(
                authorization_headers.clone(),
                SearchUserById::build_query,
                &client,
                &user_error_message,
                search_user_by_id::Variables {
                    id: short_id.clone(),
                },
            )
            .search_user_by_id;

            users
                .iter()
                .map(|element| SelectItem {
                    id: element.id,
                    description: format!("{} ({})", element.name, element.email),
                })
                .collect()
        }

        QueryType::Tokens => {
            let token_error_message = format!("Failed to retrieve token with ID '{}'.", &short_id);

            let tokens = execute_graphql_request::<
                search_token_by_id::Variables,
                search_token_by_id::ResponseData,
            >(
                authorization_headers.clone(),
                SearchTokenById::build_query,
                &client,
                &token_error_message,
                search_token_by_id::Variables {
                    id: short_id.clone(),
                },
            )
            .search_token_by_id;

            tokens
                .iter()
                .map(|element| SelectItem {
                    id: element.id,
                    description: element.name.clone(),
                })
                .collect()
        }
    };

    match all_matches.len() {
        0 => {
            match query_type {
                QueryType::Project => {
                    print_formatted_error(&format!(
                        "The project with short ID '{}' does not exist.",
                        &short_id
                    ));
                }

                QueryType::UsageHistory => {
                    print_formatted_error(&format!(
                        "The usage history with short ID '{}' does not exist.",
                        &short_id
                    ));
                }

                QueryType::User => {
                    print_formatted_error(&format!(
                        "The user with short ID '{}' does not exist.",
                        &short_id
                    ));
                }

                QueryType::Tokens => {
                    print_formatted_error(&format!(
                        "The token with short ID '{}' does not exist.",
                        &short_id
                    ));
                }
            }

            std::process::exit(1);
        }

        1 => all_matches[0].id,

        _ => {
            let items: Vec<String> = all_matches
                .iter()
                .map(|item| format!("{}  {}", item.id.to_string().green(), item.description))
                .collect();

            let selected_item_index = match Select::with_theme(&theme())
                .with_prompt(format!(
                    "Select one: {}",
                    "Use <Up>/<Down> to navigate and <Enter>/<Space> to select".dark_grey()
                ))
                .default(0)
                .items(&items)
                .max_length(config.items_per_page)
                .interact()
            {
                Ok(selected_index) => selected_index,

                Err(_) => {
                    print_formatted_error("Service error. Please try again.");
                    std::process::exit(1);
                }
            };

            all_matches[selected_item_index].id
        }
    }
}
