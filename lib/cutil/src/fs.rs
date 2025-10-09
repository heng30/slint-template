//! File system utilities for file operations, directory management, and size calculations.

use anyhow::Result;
use stacksafe::stacksafe;
use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

/// Kilobytes constant (1024 bytes)
pub const KB: u64 = 1024;

/// Megabytes constant (1024 * 1024 bytes)
pub const MB: u64 = KB * 1024;

/// Gigabytes constant (1024 * 1024 * 1024 bytes)
pub const GB: u64 = MB * 1024;

/// Terabytes constant (1024 * 1024 * 1024 * 1024 bytes)
pub const TB: u64 = GB * 1024;

/// Petabytes constant (1024 * 1024 * 1024 * 1024 * 1024 bytes)
pub const PB: u64 = TB * 1024;

/// Converts bytes to kilobytes.
///
/// # Arguments
///
/// * `bytes` - Number of bytes to convert
///
/// # Returns
///
/// Returns the equivalent value in kilobytes as a floating-point number.
#[inline]
pub fn bytes_to_kb(bytes: u64) -> f64 {
    bytes as f64 / KB as f64
}

/// Converts bytes to megabytes.
///
/// # Arguments
///
/// * `bytes` - Number of bytes to convert
///
/// # Returns
///
/// Returns the equivalent value in megabytes as a floating-point number.
#[inline]
pub fn bytes_to_mb(bytes: u64) -> f64 {
    bytes as f64 / MB as f64
}

/// Converts bytes to gigabytes.
///
/// # Arguments
///
/// * `bytes` - Number of bytes to convert
///
/// # Returns
///
/// Returns the equivalent value in gigabytes as a floating-point number.
#[inline]
pub fn bytes_to_gb(bytes: u64) -> f64 {
    bytes as f64 / GB as f64
}

/// Converts bytes to terabytes.
///
/// # Arguments
///
/// * `bytes` - Number of bytes to convert
///
/// # Returns
///
/// Returns the equivalent value in terabytes as a floating-point number.
#[inline]
pub fn bytes_to_tb(bytes: u64) -> f64 {
    bytes as f64 / TB as f64
}

/// Converts bytes to petabytes.
///
/// # Arguments
///
/// * `bytes` - Number of bytes to convert
///
/// # Returns
///
/// Returns the equivalent value in petabytes as a floating-point number.
#[inline]
pub fn bytes_to_pb(bytes: u64) -> f64 {
    bytes as f64 / PB as f64
}

/// Formats a byte size into a human-readable string with appropriate unit.
///
/// The function automatically selects the most appropriate unit (B, KB, MB, GB, TB, PB)
/// and formats the value with 2 decimal places for larger units.
///
/// # Arguments
///
/// * `bytes` - Number of bytes to format
///
/// # Returns
///
/// Returns a formatted string like "1.23 MB" or "456 B".
///
/// # Examples
///
/// ```
/// use cutil::fs::pretty_bytes_size;
///
/// assert_eq!(pretty_bytes_size(1024), "1.00 KB");
/// assert_eq!(pretty_bytes_size(1500), "1.46 KB");
/// assert_eq!(pretty_bytes_size(1024 * 1024), "1.00 MB");
/// ```
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

/// Gets the size of a file in bytes.
///
/// # Arguments
///
/// * `path` - Path to the file
///
/// # Returns
///
/// Returns the file size in bytes, or 0 if the file doesn't exist or metadata can't be read.
///
/// # Examples
///
/// ```no_run
/// use cutil::fs::file_size;
///
/// let size = file_size("/path/to/file.txt");
/// println!("File size: {} bytes", size);
/// ```
pub fn file_size(path: impl AsRef<Path>) -> u64 {
    match fs::metadata(path) {
        Ok(metadata) => metadata.len(),
        Err(e) => {
            eprintln!("Failed to get file metadata. {e}");
            0
        }
    }
}

/// Calculates the total size of multiple directories.
///
/// This function recursively calculates the size of all files in the specified directories.
///
/// # Arguments
///
/// * `dirs` - List of directories to calculate size for
///
/// # Returns
///
/// Returns the total size in bytes of all files in the directories.
///
/// # Examples
///
/// ```no_run
/// use cutil::fs::dirs_size;
///
/// let dirs = ["/path/to/dir1", "/path/to/dir2"];
/// let total_size = dirs_size(&dirs);
/// println!("Total size: {} bytes", total_size);
/// ```
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

/// Recursively calculates the total size of a directory.
///
/// This function uses stack-safe recursion to handle deep directory structures.
///
/// # Arguments
///
/// * `path` - Path to the directory
///
/// # Returns
///
/// Returns the total size in bytes of all files in the directory.
///
/// # Examples
///
/// ```no_run
/// use cutil::fs::dir_size;
/// use std::path::Path;
///
/// let size = dir_size(Path::new("/path/to/directory"));
/// println!("Directory size: {} bytes", size);
/// ```
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

/// Gets the directory containing the current executable.
///
/// This function returns the directory where the current executable is located,
/// which can be useful for finding configuration files or other resources relative to the executable.
///
/// # Returns
///
/// Returns the path to the directory containing the current executable.
///
/// # Examples
///
/// ```no_run
/// use cutil::fs::working_dir;
///
/// let dir = working_dir().unwrap();
/// println!("Executable directory: {:?}", dir);
/// ```
pub fn working_dir() -> Result<PathBuf> {
    let mut dir = env::current_exe()?;
    dir.pop();

    match dir.to_str() {
        Some(path) => Ok(PathBuf::from(path)),
        _ => Err(anyhow::anyhow!("convert {:?} failed", dir)),
    }
}

/// Removes all files from a directory without removing the directory itself.
///
/// # Arguments
///
/// * `path` - Path to the directory
///
/// # Returns
///
/// Returns `Ok(())` on success, or an IO error if file removal fails.
///
/// # Examples
///
/// ```no_run
/// use cutil::fs::remove_dir_files;
///
/// remove_dir_files("/path/to/temp").unwrap();
/// ```
pub fn remove_dir_files(path: impl AsRef<Path>) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            fs::remove_file(entry.path())?;
        }
    }
    Ok(())
}

/// Recursively removes multiple directories.
///
/// # Arguments
///
/// * `dirs` - List of directories to remove
///
/// # Returns
///
/// Returns `Ok(())` on success, or an IO error if directory removal fails.
///
/// # Examples
///
/// ```no_run
/// use cutil::fs::remove_dirs;
///
/// let dirs = ["/path/to/dir1", "/path/to/dir2"];
/// remove_dirs(&dirs).unwrap();
/// ```
pub fn remove_dirs(dirs: &[impl AsRef<Path>]) -> io::Result<()> {
    for dir in dirs {
        fs::remove_dir_all(dir.as_ref())?;
    }
    Ok(())
}

/// Recursively copies a directory and all its contents.
///
/// This function uses stack-safe recursion to handle deep directory structures.
///
/// # Arguments
///
/// * `src` - Source directory path
/// * `dst` - Destination directory path
///
/// # Returns
///
/// Returns `Ok(())` on success, or an error if copying fails.
///
/// # Examples
///
/// ```no_run
/// use cutil::fs::copy_dir_all;
///
/// copy_dir_all("/path/to/source", "/path/to/destination").unwrap();
/// ```
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

/// Checks if a file exists at the given path.
///
/// # Arguments
///
/// * `path` - Path to check
///
/// # Returns
///
/// Returns `true` if the path exists and is a file, `false` otherwise.
///
/// # Examples
///
/// ```no_run
/// use cutil::fs::file_exist;
///
/// if file_exist("/path/to/file.txt") {
///     println!("File exists!");
/// }
/// ```
pub fn file_exist(path: impl AsRef<Path>) -> bool {
    match fs::metadata(path) {
        Ok(md) => md.is_file(),
        _ => false,
    }
}

/// Extracts the file name from a path.
///
/// # Arguments
///
/// * `path` - Path to extract file name from
///
/// # Returns
///
/// Returns the file name as a string, or an empty string if the path has no file name.
///
/// # Examples
///
/// ```
/// use cutil::fs::file_name;
///
/// assert_eq!(file_name("/path/to/file.txt"), "file.txt");
/// assert_eq!(file_name("/path/to/directory/"), "directory");
/// ```
pub fn file_name(path: impl AsRef<Path>) -> String {
    path.as_ref()
        .to_path_buf()
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string()
}

/// Extracts the file name without extension from a path.
///
/// # Arguments
///
/// * `path` - Path to extract file name from
///
/// # Returns
///
/// Returns the file name without extension as a string, or an empty string if the path has no file name.
///
/// # Examples
///
/// ```
/// use cutil::fs::file_name_without_ext;
///
/// assert_eq!(file_name_without_ext("/path/to/file.txt"), "file");
/// assert_eq!(file_name_without_ext("/path/to/file.tar.gz"), "file.tar");
/// ```
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
        // Test bytes to various units
        assert_eq!(bytes_to_kb(1024), 1.0);
        assert_eq!(bytes_to_mb(1024 * 1024), 1.0);
        assert_eq!(bytes_to_gb(1024 * 1024 * 1024), 1.0);
        assert_eq!(bytes_to_tb(1024 * 1024 * 1024 * 1024), 1.0);
        assert_eq!(bytes_to_pb(1024 * 1024 * 1024 * 1024 * 1024), 1.0);
        
        // Test partial values
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
        
        // Create a file with known content
        fs::write(&file_path, "Hello, World!").unwrap();
        
        // Test file size
        let size = file_size(&file_path);
        assert!(size > 0);
        
        // Test non-existent file
        let non_existent = temp_dir.path().join("nonexistent.txt");
        assert_eq!(file_size(&non_existent), 0);
    }

    #[test]
    fn test_file_exist() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        
        // Test non-existent file
        assert!(!file_exist(&file_path));
        
        // Create file and test existence
        fs::write(&file_path, "test").unwrap();
        assert!(file_exist(&file_path));
        
        // Test with directory (should return false)
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
        
        // Create some files with known sizes
        let file1 = temp_dir.path().join("file1.txt");
        let file2 = temp_dir.path().join("file2.txt");
        let subdir = temp_dir.path().join("subdir");
        fs::create_dir(&subdir).unwrap();
        let file3 = subdir.join("file3.txt");
        
        fs::write(&file1, "content1").unwrap(); // 8 bytes
        fs::write(&file2, "content2").unwrap(); // 8 bytes  
        fs::write(&file3, "content3").unwrap(); // 8 bytes
        
        // Test dir_size
        let size = dir_size(temp_dir.path());
        assert!(size >= 24); // At least 24 bytes (3 files × 8 bytes each)
        
        // Test dirs_size
        let dirs = [temp_dir.path()];
        let total_size = dirs_size(&dirs);
        assert!(total_size >= 24);
    }

    #[test]
    fn test_copy_dir_all() -> Result<()> {
        let source_dir = tempdir().unwrap();
        let dest_dir = tempdir().unwrap();
        
        // Create source structure
        let file1 = source_dir.path().join("file1.txt");
        let subdir = source_dir.path().join("subdir");
        fs::create_dir(&subdir).unwrap();
        let file2 = subdir.join("file2.txt");
        
        fs::write(&file1, "file1 content").unwrap();
        fs::write(&file2, "file2 content").unwrap();
        
        // Copy directory
        copy_dir_all(source_dir.path(), dest_dir.path())?;
        
        // Verify copy
        assert!(dest_dir.path().join("file1.txt").exists());
        assert!(dest_dir.path().join("subdir").exists());
        assert!(dest_dir.path().join("subdir/file2.txt").exists());
        
        Ok(())
    }

    #[test]
    fn test_remove_dir_files() -> Result<()> {
        let temp_dir = tempdir().unwrap();
        
        // Create some files
        let file1 = temp_dir.path().join("file1.txt");
        let file2 = temp_dir.path().join("file2.txt");
        fs::write(&file1, "content1").unwrap();
        fs::write(&file2, "content2").unwrap();
        
        // Remove files
        remove_dir_files(temp_dir.path())?;
        
        // Verify files are gone but directory remains
        assert!(!file1.exists());
        assert!(!file2.exists());
        assert!(temp_dir.path().exists());
        
        Ok(())
    }
}
