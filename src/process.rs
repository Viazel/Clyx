use std::{
    path::Path,
    process::{Command, Stdio},
};

use anyhow::{Context, Result};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("{program} exited unsuccessfully ({code})")]
struct ProcessFailure {
    program: String,
    code: String,
}

pub fn run(program: &str, args: &[String], cwd: &Path) -> Result<()> {
    let status = Command::new(program)
        .args(args)
        .current_dir(cwd)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .with_context(|| format!("could not start {program}; is it installed and on PATH?"))?;
    if !status.success() {
        let code = status
            .code()
            .map_or_else(|| "a signal".to_owned(), |code| code.to_string());
        return Err(ProcessFailure {
            program: program.to_owned(),
            code,
        }
        .into());
    }
    Ok(())
}

pub fn exists(program: &str) -> bool {
    Command::new(program)
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|status| status.success())
}
