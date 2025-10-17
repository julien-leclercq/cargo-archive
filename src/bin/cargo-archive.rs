use clap::{Parser, Subcommand};
use snafu::prelude::*;
use snafu::Whatever;

#[snafu::report]
fn main() -> Result<(), Whatever> {
    let Arg {
        command: Command::Archive { path },
    } = Arg::parse();

    let meta = cargo_archive::get_metadata(&path)
        .with_whatever_context(|_| format!("could not run cargo metadata cmd for `{path}`"))?;

    cargo_archive::archive(meta)
        .with_whatever_context(|_| format!("could not archive for `{path}`"))
}

#[derive(Subcommand, Debug)]
enum Command {
    #[command(name = "archive")]
    Archive {
        #[arg(default_value_t = String::from("."))]
        path: String,
    },
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Arg {
    #[command(subcommand)]
    command: Command,
}
