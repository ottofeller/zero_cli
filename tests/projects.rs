use rexpect::error::Error;
use rexpect::spawn;

#[test]
fn test_projects_list() -> Result<(), Error> {
    let mut p = spawn("./target/debug/zero-cli projects list", Some(15000))?;
    p.exp_string("Showing projects")?;
    p.exp_string("SLUG")?;
    p.exp_string("NAME")?;
    p.exp_string("LAST USAGE")?;
    p.exp_string("cli-secrets")?;
    Ok(())
}

#[test]
fn test_projects_create_without_token() -> Result<(), Error> {
    let mut p = spawn("./target/debug/zero-cli projects create", Some(15000))?;
    p.exp_string("Type a project name")?;
    p.send_line("project from test")?;
    p.exp_string("Do you want to generate a new token for this project?")?;
    p.send_line("n")?;
    p.exp_string("Type a slug for the project")?;
    p.send_line("")?;
    p.exp_string("You created the project named 'project from test'.")?;
    p.exp_string("Project link")?;
    p.exp_string("Project slug for CLI")?;
    Ok(())
}

#[test]
fn test_projects_create_with_token_endless() -> Result<(), Error> {
    let mut p = spawn("./target/debug/zero-cli projects create", Some(15000))?;
    p.exp_string("Type a project name")?;
    p.send_line("project with token")?;
    p.exp_string("Do you want to generate a new token for this project?")?;
    p.send_line("y")?;
    p.exp_string("Enter the token name")?;
    p.send_line("testToken")?;
    p.exp_string("Endless")?;
    p.send_line(" ")?;
    p.exp_string("Type a slug for the project")?;
    p.send_line("")?;
    p.exp_string("Project link")?;
    p.exp_string("Project slug for CLI")?;
    p.exp_string("Your token")?;
    Ok(())
}

#[test]
fn test_projects_create_with_token_7_days() -> Result<(), Error> {
    let mut p = spawn("./target/debug/zero-cli projects create", Some(15000))?;
    p.exp_string("Type a project name")?;
    p.send_line("token7days")?;
    p.exp_string("Do you want to generate a new token for this project?")?;
    p.send_line("y")?;
    p.exp_string("Enter the token name")?;
    p.send_line("test")?;
    p.exp_string("Expires in")?;
    p.send("\x1B[B")?;
    p.send_line("")?;
    p.exp_string("Type a slug for the project")?;
    p.send_line("")?;
    p.exp_string("You created the project")?;
    p.exp_string("Project link")?;
    p.exp_string("Project slug for CLI")?;
    p.exp_string("Your token")?;
    Ok(())
}

#[test]
fn test_projects_delete() -> Result<(), Error> {
    let mut p = spawn(
        "./target/debug/zero-cli projects delete --slug cli-delete",
        Some(15000),
    )?;

    p.exp_string("Type")?;
    p.exp_string("to confirm deletion:")?;
    p.send_line("cli-delete")?;
    p.exp_string("Project successfully deleted")?;
    Ok(())
}

#[test]
fn test_projects_edit() -> Result<(), Error> {
    let mut p = spawn(
        "./target/debug/zero-cli projects edit --slug cli-edit",
        Some(15000),
    )?;

    p.exp_string("Type a new project name")?;
    p.send_line("edited")?;
    p.exp_string("Do you want to update the project description")?;
    p.send_line("y")?;
    p.exp_string("Type a new project description")?;
    p.send_line("description")?;
    p.exp_string("Type a slug for the project")?;
    p.send_line("")?;
    p.exp_string("The project has been successfully updated.")?;
    Ok(())
}

#[test]
fn test_projects_share() -> Result<(), Error> {
    let mut p = spawn(
        "./target/debug/zero-cli projects share --slug cli-secrets",
        Some(15000),
    )?;

    p.exp_string("Type a passphrase of at least 6 character")?;
    p.send_line("123456")?;
    p.exp_string("Expires in")?;
    p.send_line("")?;
    p.exp_string("Pick secrets")?;
    p.send_line(" ")?;
    p.send("\x1B[B")?;
    p.send_line(" ")?;
    p.send_line("")?;
    p.exp_string("Your link with secrets ")?;
    Ok(())
}

#[test]
fn test_projects_share_one_secret_expires_a_hour() -> Result<(), Error> {
    let mut p = spawn(
        "./target/debug/zero-cli projects share --slug cli-secrets",
        Some(15000),
    )?;

    p.exp_string("Type a passphrase of at least 6 character")?;
    p.send_line("123456")?;
    p.exp_string("Expires in")?;
    p.send("\x1B[B")?;
    p.send("\x1B[B")?;
    p.send_line("")?;
    p.exp_string("Pick secrets")?;
    p.send_line(" ")?;
    p.send_line("")?;
    p.exp_string("Your link with secrets ")?;
    Ok(())
}

#[test]
fn test_projects_usage_list() -> Result<(), Error> {
    let mut p = spawn(
        "./target/debug/zero-cli projects usage list --slug cli-secrets",
        Some(15000),
    )?;

    p.exp_string("Usage statistics for the project named 'Cli secrets'")?;
    p.exp_string("Use <Esc>/<q> to exit")?;
    p.exp_string("#7c15")?;
    Ok(())
}

#[test]
fn test_projects_usage_details() -> Result<(), Error> {
    let mut p = spawn(
        "./target/debug/zero-cli projects usage details --id 7c15",
        Some(15000),
    )?;

    p.exp_string("Record ID")?;
    p.exp_string("Caller")?;
    p.exp_string("Date")?;
    p.exp_string("Project")?;
    p.exp_string("Secrets fetched")?;
    Ok(())
}

#[test]
fn test_projects_view() -> Result<(), Error> {
    let mut p = spawn(
        "./target/debug/zero-cli projects view --slug cli-secrets",
        Some(15000),
    )?;

    p.exp_string("Name")?;
    p.exp_string("Slug")?;
    p.exp_string("Secrets")?;
    p.exp_string("Integrations")?;
    p.exp_string("Team")?;
    p.exp_string("URL")?;
    p.exp_string("Last used")?;
    Ok(())
}

#[test]
fn test_projects_view_with_same_slug() -> Result<(), Error> {
    let mut p = spawn(
        "./target/debug/zero-cli projects view --slug same-slug",
        Some(15000),
    )?;

    p.exp_string("Select the project")?;
    p.exp_string("Second same slug project (Team: Edit me)")?;
    p.exp_string("Same slug project (Team: Personal projects)")?;
    p.send("\x1B[B")?;
    p.send_line("")?;
    p.exp_string("Name")?;
    p.exp_string("Same slug project")?;
    p.exp_string("Slug")?;
    p.exp_string("same-slug")?;
    p.exp_string("Team")?;
    p.exp_string("Personal projects")?;
    Ok(())
}
