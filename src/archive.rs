fn main() {
    let cmd = cargo_metadata::MetadataCommand::new();

    let meta = cmd.exec().expect("could not run cargo metadata cmd");

    if !meta.target_directory.exists() {
        eprintln!("no target directory to archive")
    }

    let archive_target =
        std::fs::File::create(dbg!(format!("{}/target.tar.zstd", &meta.workspace_root)))
            .expect("could not open or create archive");

    let compressor = zstd::stream::write::Encoder::new(archive_target, 0)
        .expect("could not create zstd encoder");

    let mut archive = tar::Builder::new(compressor);

    archive
        .append_dir_all("target", &meta.target_directory)
        .expect("could not add target to archive");

    let compressor = archive.into_inner().expect("could not write this archive");

    compressor.finish().expect("could not finish compression");

    std::fs::remove_dir_all(meta.target_directory).expect("could not remove target directory")
}
