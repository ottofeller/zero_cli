mod graphql;

use crate::common::{
    authorization_headers::authorization_headers, colorful_theme::theme,
    execute_graphql_request::execute_graphql_request, keyring::keyring, lengify::lengify,
    pad_to_column_width::pad_to_column_width, print_formatted_error::print_formatted_error,
    table::table, take_user_id_from_token::take_user_id_from_token,
};

use clap::Args;
use dialoguer::Select;
use graphql::my_teams::{my_teams, MyTeams};
use graphql::shared_teams::{shared_teams, SharedTeams};
use graphql_client::GraphQLQuery;
use reqwest::Client;
use termimad::crossterm::style::Stylize;
use uuid::Uuid;

#[derive(Args, Debug)]
pub struct TeamListArgs {
    #[clap(
        short,
        long,
        help = "Access token, if not specified, the token will be taken from the keychain"
    )]
    access_token: Option<String>,
}

enum TeamsOwnership {
    MyTeams,
    SharedTeams,
}

impl TeamsOwnership {
    fn get_all_values() -> Vec<String> {
        vec!["My Teams".to_string(), "Shared Teams".to_string()]
    }
}

pub fn list(args: &TeamListArgs) {
    let access_token = match &args.access_token {
        Some(token) => token.clone(),
        None => keyring::get("access_token"),
    };

    let from_team = Select::with_theme(&theme())
        .with_prompt(format!(
            "Show teams from: {}",
            "Use <Up>/<Down> to navigate and <Enter>/<Space> to select".dark_grey()
        ))
        .items(&TeamsOwnership::get_all_values())
        .default(0)
        .interact();

    let select_error = "Failed to select the team.";

    let ownership = match from_team {
        Ok(index) => match index {
            0 => TeamsOwnership::MyTeams,
            1 => TeamsOwnership::SharedTeams,

            _ => {
                print_formatted_error(select_error);
                std::process::exit(1);
            }
        },

        Err(_) => {
            print_formatted_error(select_error);
            std::process::exit(1);
        }
    };

    let authorization_headers = authorization_headers(&access_token);
    let teams_error_message = "Failed to retrieve teams.";
    let client = Client::new();

    let user_id = match Uuid::parse_str(&take_user_id_from_token(&access_token)) {
        Ok(uuid) => uuid,

        Err(err) => {
            print_formatted_error(&format!("Invalid user id: {}", err));
            std::process::exit(1);
        }
    };

    let mut list = Vec::new();

    struct ColumnWidthSize {
        slug: usize,
        id: usize,
        members: usize,
        owner: usize,
    }

    let mut column_width_size = ColumnWidthSize {
        slug: 5,
        id: 5,
        members: 7,
        owner: 5,
    };

    let indentation = 2;

    match ownership {
        TeamsOwnership::MyTeams => {
            let my_teams = execute_graphql_request::<my_teams::Variables, my_teams::ResponseData>(
                authorization_headers.clone(),
                MyTeams::build_query,
                &client,
                &teams_error_message,
                my_teams::Variables { user_id },
            )
            .team;

            // save the length of the longest column element
            for team in &my_teams {
                column_width_size.members = column_width_size.members.max(
                    if let Some(aggregate) = &team.members_aggregate.aggregate {
                        aggregate.count.to_string().len()
                    } else {
                        0
                    },
                );

                column_width_size.owner = column_width_size.owner.max(team.owner.name.len());
                column_width_size.slug = column_width_size.slug.max(team.slug.len());
            }

            for team in my_teams {
                list.push(format!(
                    "{}{}{}{}{}",
                    pad_to_column_width(
                        format!("#{}", &team.id.to_string()[0..4]),
                        column_width_size.id + indentation
                    )
                    .green(),
                    pad_to_column_width(
                        format!("{}", &team.slug),
                        column_width_size.slug + indentation
                    ),
                    lengify(&team.name),
                    pad_to_column_width(
                        match &team.members_aggregate.aggregate {
                            Some(aggregate) => {
                                aggregate.count.to_string()
                            }

                            None => {
                                print_formatted_error(&teams_error_message);
                                std::process::exit(1);
                            }
                        },
                        column_width_size.members + indentation
                    ),
                    pad_to_column_width(
                        team.owner.name.clone(),
                        column_width_size.owner + indentation
                    )
                ))
            }
        }

        TeamsOwnership::SharedTeams => {
            let shared_teams =
                execute_graphql_request::<shared_teams::Variables, shared_teams::ResponseData>(
                    authorization_headers.clone(),
                    SharedTeams::build_query,
                    &client,
                    &teams_error_message,
                    shared_teams::Variables { user_id },
                )
                .team;

            // save the length of the longest column element
            for team in &shared_teams {
                column_width_size.members = column_width_size.members.max(
                    if let Some(aggregate) = &team.members_aggregate.aggregate {
                        aggregate.count.to_string().len()
                    } else {
                        0
                    },
                );

                column_width_size.owner = column_width_size.owner.max(team.owner.name.len());
                column_width_size.slug = column_width_size.slug.max(team.slug.len());
            }

            for team in shared_teams {
                list.push(format!(
                    "{}{}{}{}{}",
                    pad_to_column_width(
                        format!("#{}", &team.id.to_string()[0..4]),
                        column_width_size.id + indentation
                    )
                    .green(),
                    pad_to_column_width(
                        format!("{}", &team.slug),
                        column_width_size.slug + indentation
                    ),
                    lengify(&team.name),
                    pad_to_column_width(
                        match &team.members_aggregate.aggregate {
                            Some(aggregate) => {
                                aggregate.count.to_string()
                            }

                            None => {
                                print_formatted_error(&teams_error_message);
                                std::process::exit(1);
                            }
                        },
                        column_width_size.members + indentation
                    ),
                    pad_to_column_width(
                        team.owner.name.clone(),
                        column_width_size.owner + indentation
                    )
                ))
            }
        }
    }

    table(
        list,
        format!(
            "{} teams:",
            match ownership {
                TeamsOwnership::MyTeams => "My",
                TeamsOwnership::SharedTeams => "Shared",
            }
        ),
        "You don't have any teams yet.",
        format!(
            "{}{}{}{}{}",
            pad_to_column_width("ID".to_string(), column_width_size.id + indentation),
            pad_to_column_width("SLUG".to_string(), column_width_size.slug + indentation),
            lengify("NAME"),
            pad_to_column_width(
                "MEMBERS".to_string(),
                column_width_size.members + indentation
            ),
            pad_to_column_width("OWNER".to_string(), column_width_size.owner + indentation)
        ),
    )
}
