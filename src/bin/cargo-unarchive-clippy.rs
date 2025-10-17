use snafu::{prelude::*, Whatever};
use std::process::{exit, Command};

#[snafu::report]
fn main() -> Result<(), Whatever> {
    let current_dir =
        std::env::current_dir().whatever_context("failed to get working directory")?;

    // Gather cargo metadata
    let meta = cargo_archive::get_metadata(current_dir)
        .whatever_context("could not run cargo metadata cmd")?;

    // Unarchive previously archived target
    cargo_archive::unarchive(meta)
        .or_else(|e| {
            if let cargo_archive::UnarchiveError::TargetAlreadyExistError { .. } = e {
                Ok(())
            } else {
                Err(e)
            }
        })
        .whatever_context("could not unarchive")?;

    // This is not passing signals
    if let Some(c) = Command::new("cargo")
        .arg("clippy")
        .args(std::env::args().skip(2))
        .spawn()
        .whatever_context("could not start cargo check")?
        .wait()
        .whatever_context("cargo check failed to run")?
        .code()
    {
        exit(c)
    }

    Ok(())
}
