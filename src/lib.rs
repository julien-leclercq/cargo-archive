use cargo_metadata::camino::Utf8PathBuf;
use cargo_metadata::Metadata;
use snafu::prelude::*;
use std::path::PathBuf;
use std::{io, path::Path};

#[derive(Debug, Snafu)]
pub enum ArchiveError {
    #[snafu(display("could not create archive at `{}`", archive_path.display()))]
    CreateArchiveError {
        source: io::Error,
        archive_path: PathBuf,
    },

    #[snafu(display("could not create zstd encoder"))]
    ZstdEncoderError { source: io::Error },

    #[snafu(display("could not add path to archive"))]
    BuildArchiveError { source: io::Error, path: PathBuf },

    #[snafu(display("writing archive failed"))]
    WriteArchiveError { source: io::Error },

    #[snafu(display("successfully archived target, but failed to cleanup target directory"))]
    CleaningError { source: io::Error },
}

/// # Errors
///
/// Will return an `ArchiveError` see variants for more details.
pub fn archive(meta: &Metadata) -> Result<(), ArchiveError> {
    let archive_path = meta.workspace_root.as_std_path().join("target.tar.zstd");
    let to_archive_dir = meta.target_directory.as_std_path();

    if !to_archive_dir.exists() {
        eprintln!("no target directory to archive");
        return Ok(());
    }

    let archive_target =
        std::fs::File::create(&archive_path).context(CreateArchiveSnafu { archive_path })?;

    // Compress and drop compression utilities
    {
        let compressor = zstd::stream::write::Encoder::new(archive_target, 0)
            .context(ZstdEncoderSnafu {})?
            .auto_finish();

        let mut archive_builder = tar::Builder::new(compressor);

        archive_builder
            .append_dir_all("target", to_archive_dir)
            .context(BuildArchiveSnafu {
                path: to_archive_dir,
            })?;

        archive_builder.into_inner().context(WriteArchiveSnafu)?;
    }

    std::fs::remove_dir_all(to_archive_dir).context(CleaningSnafu {})
}

#[derive(Debug, Snafu)]
pub enum UnarchiveError {
    #[snafu(display("target directory already exists at `{path}`, not unarchiving"))]
    TargetAlreadyExistError { path: Utf8PathBuf },

    #[snafu(display("target archive not found at `{path}`"))]
    TargetArchiveNotFound { path: Utf8PathBuf },

    #[snafu(display("could not open target archive at `{path}`"))]
    TargetArchiveOpenError {
        path: Utf8PathBuf,
        source: io::Error,
    },

    #[snafu(display("could not create zstd decoder"))]
    ZstdDecoderError { source: io::Error },

    #[snafu(display("could not unpack archive"))]
    ArchiveUnpackError { source: io::Error },
}

/// # Errors
///
/// Will return an `UnarchiveError` see variants for more details.
pub fn unarchive(meta: Metadata) -> Result<(), UnarchiveError> {
    let target = meta.target_directory;
    let target_archive = meta.workspace_root.join("target.tar.zstd");

    ensure!(!target.exists(), TargetAlreadyExistSnafu { path: target });

    ensure!(
        target_archive.exists(),
        TargetArchiveNotFoundSnafu {
            path: target_archive
        }
    );

    let file = std::fs::File::open(&target_archive).with_context(|_| TargetArchiveOpenSnafu {
        path: target_archive.clone(),
    })?;

    let dec = zstd::Decoder::new(file).context(ZstdDecoderSnafu)?;

    tar::Archive::new(dec)
        .unpack(meta.workspace_root)
        .context(ArchiveUnpackSnafu {})?;

    std::fs::remove_file(target_archive).context(ArchiveUnpackSnafu)
}

/// Runs configured `cargo metadata` and returns parsed `Metadata`.
///
/// # Errors
///
/// Will return a `cargo_metadata::Error` see variants for more details.
pub fn get_metadata(folder_path: impl AsRef<Path>) -> cargo_metadata::Result<Metadata> {
    cargo_metadata::MetadataCommand::new()
        .manifest_path(Path::join(folder_path.as_ref(), "Cargo.toml"))
        .exec()
}
