//! String manipulation utilities.
//!
//! This module provides functions for splitting strings, generating random strings,
//! formatting sizes, and character replacement.

use rand::Rng;

/// Splits a string into fixed-length parts.
///
/// # Arguments
///
/// * `input` - The string to split
/// * `length` - The maximum length of each part
///
/// # Returns
///
/// Returns a vector of strings, each with at most `length` characters.
///
/// # Examples
///
/// ```
/// use cutil::str::split_string_to_fixed_length_parts;
///
/// let parts = split_string_to_fixed_length_parts("abcdefghij", 3);
/// assert_eq!(parts, vec!["abc", "def", "ghi", "j"]);
/// ```
pub fn split_string_to_fixed_length_parts(input: &str, length: usize) -> Vec<String> {
    input
        .chars()
        .collect::<Vec<_>>()
        .chunks(length)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect()
}

/// Formats a size in bytes into a human-readable string with units.
///
/// This function formats byte sizes using B, K, M, G units without decimal places.
///
/// # Arguments
///
/// * `size` - The size in bytes
///
/// # Returns
///
/// Returns a formatted string like "1024K" or "1G".
///
/// # Examples
///
/// ```
/// use cutil::str::pretty_size_string;
///
/// assert_eq!(pretty_size_string(1024), "1K");
/// assert_eq!(pretty_size_string(1024 * 1024), "1M");
/// assert_eq!(pretty_size_string(1024 * 1024 * 1024), "1G");
/// ```
pub fn pretty_size_string(size: u64) -> String {
    match size {
        s if s < 1024 => format!("{}B", size),
        s if s < 1024 * 1024 => format!("{}K", size / 1024),
        s if s < 1024 * 1024 * 1024 => format!("{}M", size / (1024 * 1024)),
        _ => format!("{}G", size / (1024 * 1024 * 1024)),
    }
}

/// Generates a random string of lowercase letters.
///
/// # Arguments
///
/// * `length` - The length of the random string to generate
///
/// # Returns
///
/// Returns a string of random lowercase letters with the specified length.
///
/// # Examples
///
/// ```
/// use cutil::str::random_string;
///
/// let random = random_string(10);
/// assert_eq!(random.len(), 10);
/// assert!(random.chars().all(|c| c.is_ascii_lowercase()));
/// ```
pub fn random_string(length: usize) -> String {
    let mut rng = rand::rng();
    let chars: Vec<char> = ('a'..='z').collect();
    (0..length)
        .map(|_| chars[rng.random_range(0..chars.len())])
        .collect()
}

/// Replaces multiple characters in a string with a single replacement character.
///
/// # Arguments
///
/// * `s` - The input string
/// * `chars_to_replace` - List of characters to replace
/// * `replacement` - The character to replace them with
///
/// # Returns
///
/// Returns a new string with all specified characters replaced.
///
/// # Examples
///
/// ```
/// use cutil::str::replace_multiple_chars;
///
/// let result = replace_multiple_chars("hello world", &['l', 'o'], '-');
/// assert_eq!(result, "he--- w-r-d");
/// ```
pub fn replace_multiple_chars(s: &str, chars_to_replace: &[char], replacement: char) -> String {
    s.chars()
        .map(|c| {
            if chars_to_replace.contains(&c) {
                replacement
            } else {
                c
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_string_to_fixed_length_parts() {
        // Test empty string
        assert_eq!(split_string_to_fixed_length_parts("", 3), Vec::<String>::new());
        
        // Test exact multiple of chunk size
        assert_eq!(
            split_string_to_fixed_length_parts("abcdef", 2),
            vec!["ab", "cd", "ef"]
        );
        
        // Test with remainder
        assert_eq!(
            split_string_to_fixed_length_parts("abcdefg", 2),
            vec!["ab", "cd", "ef", "g"]
        );
        
        // Test with Unicode characters
        assert_eq!(
            split_string_to_fixed_length_parts("你好世界", 2),
            vec!["你好", "世界"]
        );
    }

    #[test]
    fn test_pretty_size_string() {
        assert_eq!(pretty_size_string(0), "0B");
        assert_eq!(pretty_size_string(500), "500B");
        assert_eq!(pretty_size_string(1024), "1K");
        assert_eq!(pretty_size_string(1536), "1K"); // 1.5K rounded down
        assert_eq!(pretty_size_string(1024 * 1024), "1M");
        assert_eq!(pretty_size_string(1024 * 1024 * 1024), "1G");
    }

    #[test]
    fn test_random_string() {
        // Test different lengths
        for length in [0, 1, 5, 10, 50] {
            let random = random_string(length);
            assert_eq!(random.len(), length);
            assert!(random.chars().all(|c| c.is_ascii_lowercase()));
        }
        
        // Test that different calls produce different results
        let r1 = random_string(10);
        let r2 = random_string(10);
        // Note: There's a very small chance this could fail, but it's extremely unlikely
        assert_ne!(r1, r2);
    }

    #[test]
    fn test_replace_multiple_chars() {
        // Test basic replacement
        assert_eq!(
            replace_multiple_chars("hello world", &['l', 'o'], '-'),
            "he--- w-r-d"
        );
        
        // Test no replacements
        assert_eq!(
            replace_multiple_chars("hello world", &[], '-'),
            "hello world"
        );
        
        // Test empty string
        assert_eq!(
            replace_multiple_chars("", &['a', 'b'], '-'),
            ""
        );
        
        // Test all characters replaced
        assert_eq!(
            replace_multiple_chars("abc", &['a', 'b', 'c'], '-'),
            "---"
        );
    }
}
