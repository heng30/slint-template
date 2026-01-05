use anyhow::Result;
use stacksafe::stacksafe;
use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

pub const KB: u64 = 1024;

pub const MB: u64 = KB * 1024;

pub const GB: u64 = MB * 1024;

pub const TB: u64 = GB * 1024;

pub const PB: u64 = TB * 1024;

#[inline]
pub fn bytes_to_kb(bytes: u64) -> f64 {
    bytes as f64 / KB as f64
}

#[inline]
pub fn bytes_to_mb(bytes: u64) -> f64 {
    bytes as f64 / MB as f64
}

#[inline]
pub fn bytes_to_gb(bytes: u64) -> f64 {
    bytes as f64 / GB as f64
}

#[inline]
pub fn bytes_to_tb(bytes: u64) -> f64 {
    bytes as f64 / TB as f64
}

#[inline]
pub fn bytes_to_pb(bytes: u64) -> f64 {
    bytes as f64 / PB as f64
}

pub fn pretty_bytes_size(bytes: u64) -> String {
    if bytes >= PB {
        format!("{:.2} PB", bytes_to_pb(bytes))
    } else if bytes >= TB {
        format!("{:.2} TB", bytes_to_tb(bytes))
    } else if bytes >= GB {
        format!("{:.2} GB", bytes_to_gb(bytes))
    } else if bytes >= MB {
        format!("{:.2} MB", bytes_to_mb(bytes))
    } else if bytes >= KB {
        format!("{:.2} KB", bytes_to_kb(bytes))
    } else {
        format!("{} B", bytes)
    }
}

pub fn file_size(path: impl AsRef<Path>) -> u64 {
    match fs::metadata(path) {
        Ok(metadata) => metadata.len(),
        Err(e) => {
            eprintln!("Failed to get file metadata. {e}");
            0
        }
    }
}

pub fn dirs_size(dirs: &[impl AsRef<Path>]) -> u64 {
    let mut total_bytes: u64 = 0;

    for dir in dirs {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_dir() {
                        total_bytes += dir_size(&entry.path());
                    } else {
                        total_bytes += metadata.len();
                    }
                }
            }
        }
    }

    total_bytes
}

#[stacksafe]
pub fn dir_size(path: &Path) -> u64 {
    let mut total = 0;

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_dir() {
                    total += dir_size(&entry.path());
                } else {
                    total += metadata.len();
                }
            }
        }
    }

    total
}

pub fn working_dir() -> Result<PathBuf> {
    let mut dir = env::current_exe()?;
    dir.pop();

    match dir.to_str() {
        Some(path) => Ok(PathBuf::from(path)),
        _ => Err(anyhow::anyhow!("convert {:?} failed", dir)),
    }
}

pub fn remove_dir_files(path: impl AsRef<Path>) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            fs::remove_file(entry.path())?;
        }
    }
    Ok(())
}

pub fn remove_dirs(dirs: &[impl AsRef<Path>]) -> io::Result<()> {
    for dir in dirs {
        fs::remove_dir_all(dir.as_ref())?;
    }
    Ok(())
}

#[stacksafe]
pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<()> {
    fs::create_dir_all(&dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let entry_path = entry.path();
        let dst_path = dst.as_ref().join(entry.file_name());

        if entry_path.is_dir() {
            copy_dir_all(entry_path, dst_path)?;
        } else {
            fs::copy(entry_path, dst_path)?;
        }
    }

    Ok(())
}

pub fn file_exist(path: impl AsRef<Path>) -> bool {
    match fs::metadata(path) {
        Ok(md) => md.is_file(),
        _ => false,
    }
}

pub fn file_name(path: impl AsRef<Path>) -> String {
    path.as_ref()
        .to_path_buf()
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string()
}

pub fn file_name_without_ext(path: impl AsRef<Path>) -> String {
    path.as_ref()
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_bytes_conversion() {
        assert_eq!(bytes_to_kb(1024), 1.0);
        assert_eq!(bytes_to_mb(1024 * 1024), 1.0);
        assert_eq!(bytes_to_gb(1024 * 1024 * 1024), 1.0);
        assert_eq!(bytes_to_tb(1024 * 1024 * 1024 * 1024), 1.0);
        assert_eq!(bytes_to_pb(1024 * 1024 * 1024 * 1024 * 1024), 1.0);

        assert_eq!(bytes_to_kb(512), 0.5);
        assert_eq!(bytes_to_mb(1024 * 512), 0.5);
    }

    #[test]
    fn test_pretty_bytes_size() {
        assert_eq!(pretty_bytes_size(0), "0 B");
        assert_eq!(pretty_bytes_size(500), "500 B");
        assert_eq!(pretty_bytes_size(1024), "1.00 KB");
        assert_eq!(pretty_bytes_size(1500), "1.46 KB");
        assert_eq!(pretty_bytes_size(1024 * 1024), "1.00 MB");
        assert_eq!(pretty_bytes_size(1024 * 1024 * 1024), "1.00 GB");
        assert_eq!(pretty_bytes_size(1024 * 1024 * 1024 * 1024), "1.00 TB");
        assert_eq!(pretty_bytes_size(1024 * 1024 * 1024 * 1024 * 1024), "1.00 PB");
    }

    #[test]
    fn test_file_size() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test.txt");

        fs::write(&file_path, "Hello, World!").unwrap();

        let size = file_size(&file_path);
        assert!(size > 0);

        let non_existent = temp_dir.path().join("nonexistent.txt");
        assert_eq!(file_size(&non_existent), 0);
    }

    #[test]
    fn test_file_exist() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test.txt");

        assert!(!file_exist(&file_path));

        fs::write(&file_path, "test").unwrap();
        assert!(file_exist(&file_path));

        assert!(!file_exist(temp_dir.path()));
    }

    #[test]
    fn test_file_name() {
        assert_eq!(file_name("/path/to/file.txt"), "file.txt");
        assert_eq!(file_name("file.txt"), "file.txt");
        assert_eq!(file_name("/path/to/directory/"), "directory");
        assert_eq!(file_name(""), "");
    }

    #[test]
    fn test_file_name_without_ext() {
        assert_eq!(file_name_without_ext("/path/to/file.txt"), "file");
        assert_eq!(file_name_without_ext("file.txt"), "file");
        assert_eq!(file_name_without_ext("/path/to/file.tar.gz"), "file.tar");
        assert_eq!(file_name_without_ext("file"), "file");
        assert_eq!(file_name_without_ext("/path/to/directory/"), "directory");
        assert_eq!(file_name_without_ext(""), "");
    }

    #[test]
    fn test_working_dir() -> Result<()> {
        let wd = working_dir()?;
        assert!(wd.is_dir());
        Ok(())
    }

    #[test]
    fn test_dir_size_and_dirs_size() {
        let temp_dir = tempdir().unwrap();

        let file1 = temp_dir.path().join("file1.txt");
        let file2 = temp_dir.path().join("file2.txt");
        let subdir = temp_dir.path().join("subdir");
        fs::create_dir(&subdir).unwrap();
        let file3 = subdir.join("file3.txt");

        fs::write(&file1, "content1").unwrap();
        fs::write(&file2, "content2").unwrap();
        fs::write(&file3, "content3").unwrap();

        let size = dir_size(temp_dir.path());
        assert!(size >= 24);

        let dirs = [temp_dir.path()];
        let total_size = dirs_size(&dirs);
        assert!(total_size >= 24);
    }

    #[test]
    fn test_copy_dir_all() -> Result<()> {
        let source_dir = tempdir().unwrap();
        let dest_dir = tempdir().unwrap();

        let file1 = source_dir.path().join("file1.txt");
        let subdir = source_dir.path().join("subdir");
        fs::create_dir(&subdir).unwrap();
        let file2 = subdir.join("file2.txt");

        fs::write(&file1, "file1 content").unwrap();
        fs::write(&file2, "file2 content").unwrap();

        copy_dir_all(source_dir.path(), dest_dir.path())?;

        assert!(dest_dir.path().join("file1.txt").exists());
        assert!(dest_dir.path().join("subdir").exists());
        assert!(dest_dir.path().join("subdir/file2.txt").exists());

        Ok(())
    }

    #[test]
    fn test_remove_dir_files() -> Result<()> {
        let temp_dir = tempdir().unwrap();

        let file1 = temp_dir.path().join("file1.txt");
        let file2 = temp_dir.path().join("file2.txt");
        fs::write(&file1, "content1").unwrap();
        fs::write(&file2, "content2").unwrap();

        remove_dir_files(temp_dir.path())?;

        assert!(!file1.exists());
        assert!(!file2.exists());
        assert!(temp_dir.path().exists());

        Ok(())
    }
}
