pub fn chunk_with_merge<T: Clone>(vec: &[T], chunk_size: usize) -> Vec<Vec<T>> {
    if vec.len() <= chunk_size {
        return if vec.is_empty() {
            vec![]
        } else {
            vec![vec.to_vec()]
        };
    }

    let mut chunks: Vec<Vec<T>> = vec.chunks(chunk_size).map(|chunk| chunk.to_vec()).collect();

    // if last chunk size is less then chunk_size，merge into second last chunk
    if chunks.len() > 1 {
        let last_chunk = chunks.pop().unwrap();
        if last_chunk.len() < chunk_size {
            let second_last = chunks.last_mut().unwrap();
            second_last.extend(last_chunk);
        } else {
            chunks.push(last_chunk);
        }
    }

    chunks
}

#[cfg(test)]
mod test {
    use super::*;

    // cargo test --features=vec test_vec -- --no-capture
    #[test]
    fn test_vec() {
        let numbers: Vec<i32> = (1..25).collect();
        let chunks = chunk_with_merge(&numbers, 10);
        assert_eq!(chunks[0], (1..11).collect::<Vec<i32>>());
        assert_eq!(chunks[1], (11..25).collect::<Vec<i32>>());

        let small_vec = vec![1, 2, 3];
        let small_chunks = chunk_with_merge(&small_vec, 10);
        assert_eq!(small_chunks[0], small_vec);

        let exact_vec: Vec<i32> = (1..11).collect();
        let exact_chunks = chunk_with_merge(&exact_vec, 10);
        assert_eq!(exact_chunks[0], exact_vec);

        let eleven_vec: Vec<i32> = (1..12).collect();
        let eleven_chunks = chunk_with_merge(&eleven_vec, 10);
        assert_eq!(eleven_chunks[0], eleven_vec);
    }
}
