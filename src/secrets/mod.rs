/// ## Secrets Create
///
///  The `create` function is responsible for creating a new secret within the Zero platform.
///  Prompts the user for a data for the new secret, validates the input, and creates a new secret.
///  Generates a secret link for the newly created secret and displays it to the user.
///
mod create;
/// ## Secrets Delete
///
///  The `delete` function is responsible for deleting a user's secret from the Zero platform.
///  Prompts the user to confirm the deletion, deletes the secret, and displays a success message.
///
mod delete;
/// ## Secrets Drop
///
///  The `drop` function allows users to write the keys and field values of a secret to a file.
///  If a field key argument is specified, we dump only this key with its value to the file.
///  If no file is found at the specified path, a new one is created.
///  If the key matches an existing one, the value is updated.
///
mod drop;
/// ## Secrets Edit
///
///  The `edit` function allows users to modify an existing secret on the Zero platform.
///  Updates the specified secret:
///    a) If the `key` argument is passed, it updates a specific field within the secret.
///    b) If the `key` argument is not passed, it updates the name and vendor of the secret.
///
mod edit;
/// ## Secrets List
///
///  The `list` function enables users to retrieve and display a list of secrets associated with a
///  specific project on the Zero platform. The function accepts a project ID, extracts various
///  details about each secret and displays them on the screen.
///
mod list;
/// ## Secrets Share
///
///  The `share` function enables users to share specific secret fields with others by generating
///  a shareable link. Users can specify a passphrase, set an expiration time for the link, and select
///  which secret fields to include.
///
mod share;
/// ## Secrets View
///
///  The `view` function allows users to view the details of a specific user secret. It retrieves
///  secret details, including the secret's URL, name, vendor, last usage, and a list of fields.
///
mod view;
/// ## Secrets Common
///
/// The `common` module contains shared functionality and utilities used across different
/// secret-related commands and operations.
///
mod common;
use clap::{Parser, Subcommand};
use create::SecretsCreateArgs;
use delete::SecretsDeleteArgs;
use drop::SecretDropArgs;
use edit::SecretsEditArgs;
use list::SecretsListArgs;
use share::SecretShareArgs;
use view::SecretViewArgs;

#[derive(Parser, Debug)]
pub struct SecretsCli {
    #[clap(subcommand)]
    commands: SecretsCommands,
}

#[derive(Subcommand, Debug)]
#[clap(about = "Manage secrets")]
enum SecretsCommands {
    #[clap(about = "Create a new secret")]
    Create(SecretsCreateArgs),
    #[clap(about = "Delete a secret")]
    Delete(SecretsDeleteArgs),
    #[clap(about = "Drop a secret")]
    Drop(SecretDropArgs),
    #[clap(about = "Edit a secret")]
    Edit(SecretsEditArgs),
    #[clap(about = "List all secrets")]
    List(SecretsListArgs),
    #[clap(about = "Share a secret")]
    Share(SecretShareArgs),
    #[clap(about = "View a secret details")]
    View(SecretViewArgs),
}

pub fn match_command(input: &SecretsCli) {
    match &input.commands {
        SecretsCommands::Create(args) => create::create(&args),
        SecretsCommands::Delete(args) => delete::delete(&args),
        SecretsCommands::Drop(args) => drop::drop(&args),
        SecretsCommands::Edit(args) => edit::edit(&args),
        SecretsCommands::List(args) => list::list(&args),
        SecretsCommands::Share(args) => share::share(&args),
        SecretsCommands::View(args) => view::view(&args),
    }
}
