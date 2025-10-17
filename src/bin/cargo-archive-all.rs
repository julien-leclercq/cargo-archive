use clap::{Parser, Subcommand};
use snafu::{prelude::*, Whatever};
use std::{fs::read_dir, path::Path};

#[snafu::report]
fn main() -> Result<(), Whatever> {
    let Arg {
        command: Command::ArchiveAll {
            path, max_depth, ..
        },
    } = Arg::parse();

    traverse_directory(path, 0, max_depth)
}

#[derive(Subcommand, Debug)]
enum Command {
    #[command(name = "archive-all")]
    ArchiveAll {
        #[arg(default_value_t = String::from("."))]
        path: String,

        #[arg(default_value_t = 1, long("max-depth"))]
        max_depth: usize,
    },
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Arg {
    #[command(subcommand)]
    command: Command,
}

fn traverse_directory(
    path: impl AsRef<Path>,
    depth: usize,
    max_depth: usize,
) -> Result<(), Whatever> {
    dbg!(path.as_ref());
    dbg!(depth);

    if depth > max_depth {
        eprintln!("max depth reached");
        return Ok(());
    };

    read_dir(&path)
        .unwrap_or_else(|_| panic!("could not open {}", path.as_ref().to_str().unwrap()))
        .try_for_each(|dir_entry_res| {
            let dir_entry = dir_entry_res.whatever_context("could not open directory entry")?;

            if !(dir_entry
                .file_type()
                .with_whatever_context(|_| {
                    format!(
                        "could not get file type for `{}`",
                        dir_entry.path().to_string_lossy(),
                    )
                })?
                .is_dir())
            {
                return Ok(());
            }

            if let Ok(meta) = cargo_archive::get_metadata(dir_entry.path()) {
                return cargo_archive::archive(meta).whatever_context("context");
            }

            traverse_directory(dir_entry.path(), depth + 1, max_depth)
        })
}
