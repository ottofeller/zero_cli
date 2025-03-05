mod common;
/// ## Teams Create
///
///  The `create` function allows users to create a new team. You can pass the command name as an
///  argument or enter it in an input. Checks the uniqueness of the name. Creates a team where the
///  user is the owner.
///
mod create;
/// ## Teams Delete
///
///  The `delete` function is responsible for deleting the team.
///
mod delete;
/// ## Teams Edit
///
///  The `edit` function allows to update the name and description of the team.
///  Update only the `name` or `description` depending on the passed argument
///  If no arguments are passed, the user will be prompted to enter a new name and description of the team
///
mod edit;
/// ## Teams Leave
///
///  The “leave” function is responsible for leaving the user from the team whose slug they passed as an argument.
///  The function check team slug, ask for confirmation by entering the slug.
///  Removes the user from the team and sends a notification by email.
///  Shows a success or failure message
///
mod leave;
/// ## Teams List
///
///  The `list` function asks the user which category of commands to show, own or shared
///  If the user select "My teams" they will see a list of their own teams
///  If the user select "Shared teams" they will see a list of shared teams with the team owner's email address.
///
mod list;
/// ## Teams User
///
/// The `user` module contains subcommands for managing team users.
mod user;
/// ## Teams View
///
///  The `view` function allows users to view the details of a specific team. It retrieves
///  team details, including the team's URL, name, description, owner, and a list of members.
///
mod view;
use clap::{Parser, Subcommand};
use create::TeamsCreateArgs;
use delete::TeamsDeleteArgs;
use edit::TeamsEditArgs;
use leave::TeamLeaveArgs;
use list::TeamListArgs;
use user::TeamsUserCommands;
use view::TeamViewArgs;

#[derive(Parser, Debug)]
pub struct TeamsCli {
    #[clap(subcommand)]
    commands: TeamsCommands,
}

#[derive(Subcommand, Debug)]
#[clap(about = "Manage teams")]
enum TeamsCommands {
    #[clap(about = "Create a new team")]
    Create(TeamsCreateArgs),
    #[clap(about = "Delete a team")]
    Delete(TeamsDeleteArgs),
    #[clap(about = "Edit a team")]
    Edit(TeamsEditArgs),
    #[clap(about = "Leave a team")]
    Leave(TeamLeaveArgs),
    #[clap(about = "List all teams")]
    List(TeamListArgs),
    #[clap(about = "Manage team users")]
    User(TeamsUserCommands),
    #[clap(about = "View a team details")]
    View(TeamViewArgs),
}

pub fn match_command(input: &TeamsCli) {
    match &input.commands {
        TeamsCommands::Create(args) => create::create(&args),
        TeamsCommands::Delete(args) => delete::delete(&args),
        TeamsCommands::Edit(args) => edit::edit(&args),
        TeamsCommands::Leave(args) => leave::leave(&args),
        TeamsCommands::List(args) => list::list(&args),
        TeamsCommands::User(args) => user::teams_user_commands(args),
        TeamsCommands::View(args) => view::view(&args),
    }
}
