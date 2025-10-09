//! Backup and restore utilities for creating and restoring compressed archives.
//!
//! This module provides functions to create compressed tar.gz backups of directories
//! and restore them later.

use anyhow::Result;
use flate2::{write::GzEncoder, Compression};
use std::{
    fs, io,
    path::{Path, PathBuf},
};
use tar::{Archive, Builder};

/// Creates a compressed backup archive from multiple source directories.
///
/// # Arguments
///
/// * `sources` - List of directories to include in the backup
/// * `output` - Path where the backup archive will be created
/// * `excludes` - List of paths to exclude from the backup
///
/// # Returns
///
/// Returns `Ok(())` on success, or an error if the backup creation fails.
///
/// # Examples
///
/// ```no_run
/// use std::path::PathBuf;
/// use cutil::backup_recover::create_backup;
///
/// let sources = [PathBuf::from("/path/to/source1"), PathBuf::from("/path/to/source2")];
/// let output = PathBuf::from("/path/to/backup.tar.gz");
/// let excludes = vec![PathBuf::from("/path/to/source1/exclude")];
/// create_backup(&sources, &output, &excludes).unwrap();
/// ```
pub fn create_backup(sources: &[PathBuf], output: &Path, excludes: &[PathBuf]) -> Result<()> {
    for source in sources {
        if !source.exists() {
            anyhow::bail!(format!("Can't find source directory: {}", source.display()));
        }
    }

    let file = fs::File::create(output)?;
    let encoder = GzEncoder::new(file, Compression::default());
    let mut builder = Builder::new(encoder);

    for source in sources {
        let parent = source.parent().unwrap_or_else(|| Path::new(""));
        visit_dirs(parent, source, &mut builder, excludes)?;
    }

    builder.into_inner()?.finish()?;
    Ok(())
}

/// Recursively visits directories and adds files to the tar archive.
///
/// This is an internal helper function used by `create_backup`.
fn visit_dirs(
    root: &Path,
    current: &Path,
    builder: &mut Builder<GzEncoder<fs::File>>,
    excludes: &[PathBuf],
) -> io::Result<()> {
    for entry in fs::read_dir(current)? {
        let entry = entry?;
        let path = entry.path();
        let metadata = fs::metadata(&path)?;
        let rel_path = path.strip_prefix(root).unwrap();

        if excludes
            .iter()
            .any(|exclude| path.starts_with(exclude) || rel_path.starts_with(exclude))
        {
            continue;
        }

        if metadata.is_dir() {
            visit_dirs(root, &path, builder, excludes)?;
        } else if metadata.is_file() {
            let mut file = fs::File::open(&path)?;
            builder.append_file(rel_path, &mut file)?;
        }
    }
    Ok(())
}

/// Restores a backup archive to the specified target directory.
///
/// # Arguments
///
/// * `input` - Path to the backup archive to restore
/// * `target` - Directory where the backup will be restored
///
/// # Returns
///
/// Returns `Ok(())` on success, or an error if the restoration fails.
///
/// # Examples
///
/// ```no_run
/// use std::path::PathBuf;
/// use cutil::backup_recover::restore_backup;
///
/// let input = PathBuf::from("/path/to/backup.tar.gz");
/// let target = PathBuf::from("/path/to/restore");
/// restore_backup(&input, &target).unwrap();
/// ```
pub fn restore_backup(input: &Path, target: &Path) -> Result<()> {
    if !input.exists() {
        anyhow::bail!(format!("Can't find backup file: {}", input.display()));
    }

    if !target.exists() {
        fs::create_dir_all(target)?;
    }

    let file = fs::File::open(input)?;
    let decoder = flate2::read::GzDecoder::new(file);
    let mut archive = Archive::new(decoder);

    archive.unpack(target)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backup_recover() -> Result<()> {
        let sources = ["target/test1", "target/test2"]
            .into_iter()
            .map(|item| Path::new(item).to_path_buf())
            .collect::<Vec<_>>();
        let excludes = vec![Path::new("target/test2/test3").to_path_buf()];
        // let excludes: Vec<PathBuf> = vec![];
        let output = Path::new("target/test.tar.gz");

        for (index, path) in sources.iter().enumerate() {
            fs::create_dir_all(path)?;
            fs::write(path.join(&format!("foo-{index}")), &format!("{index}"))?;
        }

        for (index, path) in excludes.iter().enumerate() {
            fs::create_dir_all(path)?;
            fs::write(path.join(&format!("foo-{index}")), &format!("{index}"))?;
        }

        if let Err(e) = create_backup(&sources, &output, &excludes) {
            anyhow::bail!("backup failed: {}", e);
        }

        println!("backup file: {}", output.display());

        let input = Path::new("target/test.tar.gz");
        let target = Path::new("target/test3");
        if let Err(e) = restore_backup(input, target) {
            anyhow::bail!("recover failed: {}", e);
        }

        println!("backup directory: {}", target.display());

        Ok(())
    }

    #[test]
    fn test_backup_recover2() -> Result<()> {
        let sources = ["/tmp/test1", "/tmp/test2"]
            .into_iter()
            .map(|item| Path::new(item).to_path_buf())
            .collect::<Vec<_>>();
        // let excludes = vec![Path::new("/tmp/test2/test3").to_path_buf()];
        let excludes: Vec<PathBuf> = vec![];
        let output = Path::new("/tmp/test.tar.gz");

        for (index, path) in sources.iter().enumerate() {
            fs::create_dir_all(path)?;
            fs::write(path.join(&format!("foo-{index}")), &format!("{index}"))?;
        }

        for (index, path) in excludes.iter().enumerate() {
            fs::create_dir_all(path)?;
            fs::write(path.join(&format!("foo-{index}")), &format!("{index}"))?;
        }

        if let Err(e) = create_backup(&sources, &output, &excludes) {
            anyhow::bail!("backup failed: {}", e);
        }

        println!("backup file: {}", output.display());

        let input = Path::new("/tmp/test.tar.gz");
        let target = Path::new("/tmp/test3");
        if let Err(e) = restore_backup(input, target) {
            anyhow::bail!("recover failed: {}", e);
        }

        println!("backup directory: {}", target.display());

        Ok(())
    }
}
