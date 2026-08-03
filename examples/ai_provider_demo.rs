use ai::{AIProvider, OpenAIProvider};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let provider = OpenAIProvider {
        api_key: "demo_key".to_string(),
    };
    let response = provider.generate("Generate resume summary for Rust Developer").await?;
    println!("Response: {}", response);
    Ok(())
}
