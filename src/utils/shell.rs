use execute::{shell, Execute};
use std::process::Stdio;

use crate::errors::ServerError;

pub fn execute_command(command_value: String) -> Result<(), ServerError> {
    let mut command = shell(command_value);

    command.stdout(Stdio::piped());

    command.execute_output()?;

    Ok(())
}
