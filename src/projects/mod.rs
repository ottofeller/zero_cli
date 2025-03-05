/// ## Projects Create
///
///  The `create` function is responsible for creating a project in the Zero system. The user enters
///  a name and team name. If the team is not passed, the project will be added to personal projects.
///  Once created, displays project information in the terminal.
///
mod create;
/// ## Projects Delete
///
///  The `delete` function allow delete the project using the passed slug.
///  To confirm the user must enter the slug.
///
mod delete;
/// ## Projects Edit
///
///  The `edit` function allows to update the name and description of the project.
///  If neither of these arguments are passed, the function will prompt you to enter the name
///  and description at runtime.
///
mod edit;
/// ## Projects List
///
///  The `list` function displays a list of all projects from personal projects.
///  If the slug argument is passed, it will show all projects that belong to the specified team.
///
mod list;
/// ## Projects Share
///
///  The `share` function enables users to share specific secret fields with others by generating
///  a shareable link. If secrets are found in the project, than user can specify a passphrase,
///  set an expiration time for the link, and choose which secrets to share.
///
mod share;
/// ## Projects Usage
///
/// The `usage` module contains subcommands that allow you to view the usage of the project.
///
mod usage;
/// ## Projects View
///
///  The `view` function displays detailed information about the project.
///  Name, owner, number of secrets, and link to view the project page in the Zero web application.
///
mod view;

/// ## Projects Common
///
/// The `common` module contains shared functionality and utilities used across different
/// project-related commands and operations.
///
pub mod common;
use self::create::ProjectsCreateArgs;
use self::delete::ProjectsDeleteArgs;
use self::edit::ProjectsEditArgs;
use self::list::ProjectsListArgs;
use self::share::ProjectsShareArgs;
use self::usage::ProjectsUsageCommands;
use self::view::ProjectsViewArgs;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct ProjectsCli {
    #[clap(subcommand)]
    commands: ProjectsCommands,
}

#[derive(Subcommand, Debug)]
#[clap(about = "Manage projects")]
enum ProjectsCommands {
    #[clap(about = "Create a new project")]
    Create(ProjectsCreateArgs),
    #[clap(about = "Delete a project")]
    Delete(ProjectsDeleteArgs),
    #[clap(about = "Edit a project")]
    Edit(ProjectsEditArgs),
    #[clap(about = "List all projects")]
    List(ProjectsListArgs),
    #[clap(about = "Share a project")]
    Share(ProjectsShareArgs),
    #[clap(about = "View a project usage")]
    Usage(ProjectsUsageCommands),
    #[clap(about = "View a project details")]
    View(ProjectsViewArgs),
}

pub fn match_command(input: &ProjectsCli) {
    match &input.commands {
        ProjectsCommands::Create(args) => create::create(&args),
        ProjectsCommands::Delete(args) => delete::delete(&args),
        ProjectsCommands::Edit(args) => edit::edit(&args),
        ProjectsCommands::List(args) => list::list(&args),
        ProjectsCommands::Share(args) => share::share(&args),
        ProjectsCommands::Usage(args) => usage::usage_commands(&args),
        ProjectsCommands::View(args) => view::view(&args),
    }
}
