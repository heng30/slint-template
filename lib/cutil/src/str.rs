use rand::Rng;

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
    let mut rng = rand::thread_rng();
    let chars: Vec<char> = ('a'..='z').collect();
    (0..length)
        .map(|_| chars[rng.gen_range(0..chars.len())])
        .collect()
}
