use rexpect::error::Error;
use rexpect::spawn;

#[test]
fn test_teams_create() -> Result<(), Error> {
    let mut p = spawn("./target/debug/zero-cli teams create", Some(15000))?;
    p.exp_string("Type a name for the team")?;
    p.send_line("tested team")?;
    p.exp_string("Type a slug for the team")?;
    p.send_line("")?;
    p.exp_string("Team link")?;
    p.exp_string("Team name")?;
    p.exp_string("Team slug")?;
    Ok(())
}

#[test]
fn test_teams_delete() -> Result<(), Error> {
    let mut p = spawn(
        "./target/debug/zero-cli teams delete --slug delete-me",
        Some(15000),
    )?;

    p.exp_string("Type delete-me to confirm deletion")?;
    p.send_line("delete-me")?;
    p.exp_string("Team successfully deleted")?;
    Ok(())
}

#[test]
fn test_teams_edit() -> Result<(), Error> {
    let mut p = spawn(
        "./target/debug/zero-cli teams edit --slug edit-me",
        Some(15000),
    )?;
    p.exp_string("Type a new team name")?;
    p.send_line("update team name")?;
    p.exp_string("Type a new team description")?;
    p.send_line("update team description")?;
    p.exp_string("Type a slug for the team")?;
    p.send_line("")?;
    p.exp_string("The team has been successfully updated")?;
    Ok(())
}

#[test]
fn test_teams_leave_and_teams_list_shared() -> Result<(), Error> {
    let mut p = spawn(
        "./target/debug/zero-cli teams leave --slug leave",
        Some(15000),
    )?;
    p.exp_string("Type leave to confirm leaving the team")?;
    p.send_line("leave")?;
    p.exp_string("You have successfully left")?;

    // Check team list after leaving the team
    let mut p = spawn("./target/debug/zero-cli teams list", Some(15000))?;
    p.exp_string("Show teams from")?;
    p.exp_string("Shared Teams")?;
    p.send("\x1B[B")?;
    p.send_line("")?;
    p.exp_string("You don't have any teams yet.")?;
    Ok(())
}

#[test]
fn test_teams_list_my() -> Result<(), Error> {
    let mut p = spawn("./target/debug/zero-cli teams list", Some(15000))?;
    p.exp_string("Show teams from")?;
    p.exp_string("My Teams")?;
    p.send_line("")?;
    p.exp_string("Personal projects")?;
    Ok(())
}

#[test]
fn test_teams_view() -> Result<(), Error> {
    let mut p = spawn(
        "./target/debug/zero-cli teams view --slug personal-projects-22",
        Some(15000),
    )?;
    p.exp_string("URL")?;
    p.exp_string("Personal projects")?;
    p.exp_string("Owner")?;
    p.exp_string("Members")?;
    Ok(())
}

#[test]
fn test_teams_user_invite() -> Result<(), Error> {
    let mut p = spawn(
        "./target/debug/zero-cli teams user invite --email test@test.com --slug edit-me",
        Some(15000),
    )?;

    p.exp_string("The invitation was successfully sent to the user. test@test.com")?;
    Ok(())
}

#[test]
fn test_teams_user_remove() -> Result<(), Error> {
    let mut p = spawn(
        "./target/debug/zero-cli teams user remove --slug edit-me --user-id d541",
        Some(15000),
    )?;

    p.send_line("")?;
    p.exp_string("Type edit-me to confirm deletion")?;
    p.send_line("edit-me")?;
    p.exp_string("User successfully removed from the team")?;
    Ok(())
}
