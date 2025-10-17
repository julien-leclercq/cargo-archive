# Cargo archive

🚧 This tool is a WIP 🚧

The goal is to provide a tool for compressing heavy target directories in rust project. 

This is composed of 5 executables: 
- `cargo-archive`: compress the current workspace's target directory in a `target.tar.zstd` file
- `cargo-unarchive`: uncompress a `target.tar.zstd` file in the current workspace's target directory
- `cargo-archive-all`: compress all rust projects in a given folder 
- `cargo-unarchive-check` and `cargo-unarchive-clippy`: unarchive the current workspace before running `cargo check` or `cargo clippy` respectively with the given args. 

For now, run the executables with `--help` to get a help message, better docs to come. 
