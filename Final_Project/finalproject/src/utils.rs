pub fn chunkify<T>(items: &[T], num_chunks: usize) -> Vec<Vec<T>>
where
    T: Clone,
{
    let mut chunks = Vec::new();
    let chunk_size = (items.len() + num_chunks - 1) / num_chunks;

    for i in 0..num_chunks {
        let start = i * chunk_size;
        let end = std::cmp::min(start + chunk_size, items.len());
        if start < end {
            chunks.push(items[start..end].to_vec());
        }
    }

    chunks
}
