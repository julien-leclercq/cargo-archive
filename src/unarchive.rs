fn main() {
    let cmd = cargo_metadata::MetadataCommand::new();
    let meta = cmd.exec().expect("could not run cargo metadata cmd");

    if meta.target_directory.exists() {
        return eprintln!("target directory already exists, not unarchiving");
    }

    let archive_path = std::path::PathBuf::from(format!("{}/target.tar.zstd", meta.workspace_root));

    assert!(archive_path.exists(), "target archive not found");

    let file = std::fs::File::open(&archive_path).expect("could not open archive");

    let dec = zstd::Decoder::new(file).expect("could not instantiate zstd decoder");

    tar::Archive::new(dec)
        .unpack(meta.workspace_root)
        .expect("could not unpack archive");

    std::fs::remove_file(archive_path).expect("could not remove archive")
}
