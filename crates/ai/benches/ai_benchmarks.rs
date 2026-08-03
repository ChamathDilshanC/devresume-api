use ai::embeddings::cosine_similarity;

fn benchmark_cosine_similarity() {
    let vec1 = vec![0.1f32; 1536];
    let vec2 = vec![0.2f32; 1536];

    for _ in 0..10_000 {
        let _ = cosine_similarity(&vec1, &vec2);
    }
}

fn main() {
    println!("Running DevResume AI Benchmarks...");
    let start = std::time::Instant::now();
    benchmark_cosine_similarity();
    let duration = start.elapsed();
    println!(
        "Executed 10,000 1536-dimensional Cosine Similarity vector computations in {:?}",
        duration
    );
}
