mod graphql;
use crate::common::{
    authorization_headers::authorization_headers,
    config::Config,
    execute_graphql_request::execute_graphql_request,
    keyring::keyring,
    pad_to_column_width::pad_to_column_width,
    print_formatted_error::print_formatted_error,
    table::table,
};
use crate::projects::common::project_info_by_slug::project_info_by_slug;
use crate::projects::usage::list::graphql::project_usage::{project_usage, ProjectUsage};
use clap::Args;
use graphql_client::GraphQLQuery;
use reqwest::Client;
use termimad::crossterm::style::Stylize;

#[derive(Args, Debug)]
pub struct ProjectsUsageListArgs {
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

pub fn usage(args: &ProjectsUsageListArgs) {
    let access_token = match &args.access_token {
        Some(token) => token.clone(),
        None => keyring::get("access_token"),
    };

    let project_info = project_info_by_slug(&args.slug, &access_token);
    let date_format = &Config::new().date_format;

    let project_usage_error_message = format!(
        "Failed to retrieve usage statistics for the project with slug '{}'.",
        &args.slug
    );

    let project_usage_response =
        execute_graphql_request::<project_usage::Variables, project_usage::ResponseData>(
            authorization_headers(&access_token),
            ProjectUsage::build_query,
            &Client::new(),
            &project_usage_error_message,
            project_usage::Variables { id: project_info.id },
        )
        .project;

    let project_usage_stats = match project_usage_response.first() {
        Some(project_info) => project_info,
        None => {
            print_formatted_error(&project_usage_error_message);
            std::process::exit(1);
        }
    };

    struct ColumnWidthSize {
        id: usize,
        date: usize,
        secrets: usize,
        caller_name: usize,
        remote_ip: usize,
    }

    let mut column_width_size = ColumnWidthSize {
        id: 5,
        secrets: 10,
        date: 4,
        caller_name: 0,
        remote_ip: 9,
    };

    // save the length of the longest column element
    for history in &project_usage_stats.usage_histories {
        if let Some(name) = &history.caller_name {
            column_width_size.caller_name = column_width_size.caller_name.max(name.len());
        }

        let unwrapped_history = history.remote_ip.clone().unwrap_or("N\\A".to_string());
        column_width_size.remote_ip = column_width_size.remote_ip.max(unwrapped_history.len());
        let created_at_data = &history.created_at.format(date_format);

        column_width_size.date = column_width_size
            .date
            .max(created_at_data.to_string().len());
    }

    if column_width_size.caller_name > 0 {
        column_width_size.caller_name = column_width_size.caller_name.max("CALLER NAME".len());
    }

    let mut list = Vec::new();
    let indentation = 2;

    // output data to the console. If no field contains caller_name then do not include it in the text
    for history in &project_usage_stats.usage_histories {
        let created_at_data = &history.created_at.format(date_format);
        let unwrapped_history = history.remote_ip.clone().unwrap_or("N\\A".to_string());

        list.push(format!(
            "{}{}{}{}{}",
            pad_to_column_width(
                format!("#{}", &history.id.to_string()[0..4]),
                column_width_size.id + indentation
            )
            .green(),
            pad_to_column_width(
                created_at_data.to_string(),
                column_width_size.date + indentation
            ),
            pad_to_column_width(
                match &history.secrets_aggregate.aggregate {
                    Some(aggregate) if aggregate.count == 1 =>
                        format!("{} secret", aggregate.count),
                    Some(aggregate) => format!("{} secrets", aggregate.count),
                    None => "not used".to_string(),
                },
                column_width_size.secrets + indentation,
            ),
            if let Some(caller_name) = &history.caller_name {
                if column_width_size.caller_name > 0 {
                    pad_to_column_width(
                        caller_name.to_string(),
                        column_width_size.caller_name + indentation,
                    )
                } else {
                    String::from("")
                }
            } else {
                String::from("")
            },
            pad_to_column_width(unwrapped_history, column_width_size.remote_ip + indentation,)
        ));
    }

    table(
        list,
        format!(
            "Usage statistics for the project named '{}'",
            project_usage_stats.name
        ),
        "You don't have any statistics records for this project.",
        format!(
            "{}{}{}{}{}",
            pad_to_column_width("ID".to_string(), column_width_size.id + indentation),
            pad_to_column_width("DATE".to_string(), column_width_size.date + indentation),
            pad_to_column_width(
                "SECRETS".to_string(),
                column_width_size.secrets + indentation
            ),
            if column_width_size.caller_name > 0 {
                pad_to_column_width(
                    "CALLER NAME".to_string(),
                    column_width_size.caller_name + indentation,
                )
            } else {
                "".to_string()
            },
            pad_to_column_width(
                "REMOTE IP".to_string(),
                column_width_size.remote_ip + indentation
            )
        ),
    )
}
