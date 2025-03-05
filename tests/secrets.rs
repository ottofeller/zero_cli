use rexpect::error::Error;
use rexpect::spawn;

#[test]
fn test_secrets_create_with_one_field() -> Result<(), Error> {
    let mut p = spawn(
        "./target/debug/zero-cli secrets create --slug cli-secrets",
        Some(15000),
    )?;

    p.exp_string("Type a name for the secret")?;
    p.send_line("test secret one field")?;
    p.send("\x1B[B")?;
    p.send("\x1B[B")?;
    p.send("\x1B[B")?;
    p.send("\x1B[B")?;
    p.send_line("")?;
    p.exp_string("Type a field name:")?;
    p.send_line("new_secret_one_field")?;
    p.exp_string("Type a field value:")?;
    p.send_line("111222")?;
    p.exp_string("Do you want to add another field")?;
    p.send_line("n")?;
    p.exp_string("Secret link:")?;
    p.exp_string("Secret ID:")?;
    Ok(())
}

#[test]
fn test_secrets_create_with_many_fields() -> Result<(), Error> {
    let mut p = spawn(
        "./target/debug/zero-cli secrets create --slug cli-secrets",
        Some(15000),
    )?;

    p.exp_string("Type a name for the secret")?;
    p.send_line("new_secret_many_fields")?;
    p.send("\x1B[B")?;
    p.send("\x1B[B")?;
    p.send("\x1B[B")?;
    p.send_line("")?;
    p.exp_string("Type a field name:")?;
    p.send_line("first")?;
    p.exp_string("Type a field value:")?;
    p.send_line("11111")?;
    p.exp_string("Do you want to add another field")?;
    p.send_line("y")?;
    p.exp_string("Type a field name:")?;
    p.send_line("second")?;
    p.exp_string("Type a field value:")?;
    p.send_line("222222")?;
    p.exp_string("Do you want to add another field")?;
    p.send_line("y")?;
    p.exp_string("Type a field name:")?;
    p.send_line("third")?;
    p.exp_string("Type a field value:")?;
    p.send_line("3333")?;
    p.exp_string("Do you want to add another field")?;
    p.send_line("n")?;
    p.exp_string("Secret link:")?;
    p.exp_string("Secret ID:")?;
    Ok(())
}

#[test]
fn test_secrets_delete() -> Result<(), Error> {
    let mut p = spawn(
        "./target/debug/zero-cli secrets delete --slug envs-for-test",
        Some(15000),
    )?;

    p.exp_string("Type envs-for-test to confirm deletion")?;
    p.send_line("envs-for-test")?;
    p.exp_string("Secret successfully deleted")?;
    Ok(())
}

#[test]
fn test_secrets_edit() -> Result<(), Error> {
    let mut p = spawn(
        "./target/debug/zero-cli secrets edit --slug keys-for-test",
        Some(15000),
    )?;

    p.exp_string("Type a new secret name")?;
    p.send_line("edited secret")?;
    p.send("\x1B[B")?;
    p.send_line("")?;
    p.exp_string("Type a slug for the secret")?;
    p.send_line("")?;
    p.exp_string("The secret has been successfully updated.")?;
    Ok(())
}

#[test]
fn test_secrets_list() -> Result<(), Error> {
    let mut p = spawn(
        "./target/debug/zero-cli secrets list --slug cli-secrets",
        Some(15000),
    )?;

    p.exp_string("keys-for-test")?;
    p.exp_string("list secret")?;
    Ok(())
}

#[test]
fn test_secrets_share() -> Result<(), Error> {
    let mut p = spawn(
        "./target/debug/zero-cli secrets share --slug share-other-secret",
        Some(15000),
    )?;

    p.exp_string("Type a passphrase of at least 6 character")?;
    p.send_line("123Qwe")?;
    p.exp_string("Expires in")?;
    p.send("\x1B[B")?;
    p.send_line("")?;
    p.exp_string("Pick secret fields")?;
    p.send_line(" ")?;
    p.send_line("")?;
    p.exp_string("Your link for the shared secret")?;
    Ok(())
}

#[test]
fn test_secrets_view() -> Result<(), Error> {
    let mut p = spawn(
        "./target/debug/zero-cli secrets view --slug share-other-secret",
        Some(15000),
    )?;

    p.exp_string("URL")?;
    p.exp_string("Name")?;
    p.exp_string("Vendor")?;
    p.exp_string("Fields")?;
    Ok(())
}

#[test]
fn test_secrets_field_edit() -> Result<(), Error> {
    let mut p = spawn(
        "./target/debug/zero-cli secrets edit --slug keys-for-test  --key API_key",
        Some(15000),
    )?;

    p.exp_string("Type a new field key")?;
    p.send_line("newApiKey")?;
    p.exp_string("Type a new field value")?;
    p.send_line("1234Ssg")?;
    p.exp_string("Type a slug for the secret")?;
    p.send_line("")?;
    p.exp_string("The secret has been successfully updated.")?;
    Ok(())
}
