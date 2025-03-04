mod graphql;
use crate::common::{
    authorization_headers::authorization_headers, colorful_theme::theme,
    execute_graphql_request::execute_graphql_request, keyring::keyring,
    print_formatted_error::print_formatted_error,
};
use crate::secrets::common::secret_info_by_slug::secret_info_by_slug;
use crate::secrets::delete::graphql::delete_user_secret::{delete_user_secret, DeleteUserSecret};
use clap::Args;
use dialoguer::Input;
use graphql_client::GraphQLQuery;
use reqwest::Client;
use termimad::crossterm::style::Stylize;

#[derive(Args, Debug)]
pub struct SecretsDeleteArgs {
    #[clap(short, long, help = "Secret slug")]
    slug: String,
    #[clap(
        short,
        long,
        help = "Access token, if not specified, the token will be taken from the keychain"
    )]
    access_token: Option<String>,
}

pub fn delete(args: &SecretsDeleteArgs) {
    let access_token = match &args.access_token {
        Some(token) => token.clone(),
        None => keyring::get("access_token"),
    };

    let secret_info = secret_info_by_slug(&args.slug, &access_token);

    let input: String = Input::with_theme(&theme())
        .with_prompt(format!("Type {} to confirm deletion:", &args.slug,))
        .interact_text()
        .expect("Deletion failed. Failed to read user input.");

    if input != args.slug.clone() {
        println!(
            "{}",
            format!("X Sorry your reply was invalid: You entered {}.", input).red()
        );

        std::process::exit(0);
    }

    let delete_secret_error_message = format!(
        "Deletion failed. Failed to delete the secret with slug '{}'.",
        &args.slug
    );

    let delete_secret_response =
        execute_graphql_request::<delete_user_secret::Variables, delete_user_secret::ResponseData>(
            authorization_headers(&access_token),
            DeleteUserSecret::build_query,
            &Client::new(),
            &delete_secret_error_message,
            delete_user_secret::Variables { id: secret_info.id },
        )
        .delete_user_secret_by_pk;

    if delete_secret_response.is_none() {
        print_formatted_error(&delete_secret_error_message);
        std::process::exit(1);
    }

    println!("{} {}", "✔".green(), "Secret successfully deleted");
}
