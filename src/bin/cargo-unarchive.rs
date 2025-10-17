use clap::{Parser, Subcommand};
use snafu::{prelude::*, Whatever};

#[snafu::report]
fn main() -> Result<(), Whatever> {
    let Arg {
        command: Command::Unarchive { path, .. },
    } = Arg::parse();

    // Gather cargo metadata
    let meta =
        cargo_archive::get_metadata(path).whatever_context("could not run cargo metadata cmd")?;

    // Unarchive previously archived target
    cargo_archive::unarchive(meta).whatever_context("could not unarchive target")
}

#[derive(Subcommand, Debug)]
enum Command {
    #[command(name = "unarchive")]
    Unarchive {
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
