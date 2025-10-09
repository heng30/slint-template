//! Number formatting utilities.
//!
//! This module provides functions for formatting numbers with commas for better readability.

/// Formats a number string with commas for thousands separators.
///
/// This function takes a string representation of a number (which may include decimal places)
/// and inserts commas as thousands separators.
///
/// # Arguments
///
/// * `number_str` - The number as a string (e.g., "1234567.89")
///
/// # Returns
///
/// Returns the formatted string with commas (e.g., "1,234,567.89").
///
/// # Examples
///
/// ```
/// use cutil::number::format_number_with_commas;
///
/// assert_eq!(format_number_with_commas("1234567"), "1,234,567");
/// assert_eq!(format_number_with_commas("1234.56"), "1,234.56");
/// assert_eq!(format_number_with_commas("123"), "123");
/// assert_eq!(format_number_with_commas(""), "");
/// ```
pub fn format_number_with_commas(number_str: &str) -> String {
    if number_str.is_empty() {
        return String::default();
    }

    let chars: Vec<char> = number_str.chars().collect();
    let decimal_index = chars.iter().position(|&c| c == '.').unwrap_or(chars.len());

    let left_part = &mut chars[0..decimal_index]
        .iter()
        .rev()
        .copied()
        .collect::<Vec<char>>();

    let right_part = &number_str[decimal_index..];

    let mut chs = vec![];
    for (i, ch) in left_part.iter().enumerate() {
        chs.push(*ch);
        if (i + 1) % 3 == 0 {
            chs.push(',');
        }
    }

    if chs[chs.len() - 1] == ',' {
        chs.pop();
    }

    format!("{}{}", chs.iter().rev().collect::<String>(), right_part)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_number_with_commas() {
        let verify = vec![
            "", "1.23", "12.12", "123.12", "1,234.12", "1", "12", "123", "1,234", "123,456",
        ];

        let mut output = vec![];
        for item in vec![
            "", "1.23", "12.12", "123.12", "1234.12", "1", "12", "123", "1234", "123456",
        ] {
            output.push(format_number_with_commas(&item));
        }

        assert_eq!(verify, output);
    }

    #[test]
    fn test_format_number_with_commas_edge_cases() {
        // Test empty string
        assert_eq!(format_number_with_commas(""), "");
        
        // Test single digit
        assert_eq!(format_number_with_commas("1"), "1");
        
        // Test two digits
        assert_eq!(format_number_with_commas("12"), "12");
        
        // Test three digits
        assert_eq!(format_number_with_commas("123"), "123");
        
        // Test four digits
        assert_eq!(format_number_with_commas("1234"), "1,234");
        
        // Test six digits
        assert_eq!(format_number_with_commas("123456"), "123,456");
        
        // Test seven digits
        assert_eq!(format_number_with_commas("1234567"), "1,234,567");
        
        // Test with decimal
        assert_eq!(format_number_with_commas("1234.56"), "1,234.56");
        assert_eq!(format_number_with_commas("1234567.89"), "1,234,567.89");
        
        // Test with multiple decimal points (should only consider first one)
        assert_eq!(format_number_with_commas("1234.56.78"), "1,234.56.78");
    }
}
