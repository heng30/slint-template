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
}
