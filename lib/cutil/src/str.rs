use rand::RngExt;

pub fn split_string_to_fixed_length_parts(input: &str, length: usize) -> Vec<String> {
    input
        .chars()
        .collect::<Vec<_>>()
        .chunks(length)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect()
}

pub fn pretty_size_string(size: u64) -> String {
    match size {
        s if s < 1024 => format!("{}B", size),
        s if s < 1024 * 1024 => format!("{}K", size / 1024),
        s if s < 1024 * 1024 * 1024 => format!("{}M", size / (1024 * 1024)),
        _ => format!("{}G", size / (1024 * 1024 * 1024)),
    }
}

pub fn random_string(length: usize) -> String {
    let mut rng = rand::rng();
    let chars: Vec<char> = ('a'..='z').collect();
    (0..length)
        .map(|_| chars[rng.random_range(0..chars.len())])
        .collect()
}

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
        assert_eq!(
            split_string_to_fixed_length_parts("", 3),
            Vec::<String>::new()
        );

        assert_eq!(
            split_string_to_fixed_length_parts("abcdef", 2),
            vec!["ab", "cd", "ef"]
        );

        assert_eq!(
            split_string_to_fixed_length_parts("abcdefg", 2),
            vec!["ab", "cd", "ef", "g"]
        );

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
        assert_eq!(pretty_size_string(1536), "1K");
        assert_eq!(pretty_size_string(1024 * 1024), "1M");
        assert_eq!(pretty_size_string(1024 * 1024 * 1024), "1G");
    }

    #[test]
    fn test_random_string() {
        for length in [0, 1, 5, 10, 50] {
            let random = random_string(length);
            assert_eq!(random.len(), length);
            assert!(random.chars().all(|c| c.is_ascii_lowercase()));
        }

        let r1 = random_string(10);
        let r2 = random_string(10);
        assert_ne!(r1, r2);
    }

    #[test]
    fn test_replace_multiple_chars() {
        assert_eq!(
            replace_multiple_chars("hello world", &['l', 'o'], '-'),
            "he--- w-r-d"
        );

        assert_eq!(
            replace_multiple_chars("hello world", &[], '-'),
            "hello world"
        );

        assert_eq!(replace_multiple_chars("", &['a', 'b'], '-'), "");

        assert_eq!(replace_multiple_chars("abc", &['a', 'b', 'c'], '-'), "---");
    }
}
