use clients::OpenAILLM;

mod chains;
mod clients;
mod llm;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = OpenAILLM::new("OPEN_AI_KEY_GOES_HERE".to_string());
    let mr = chains::SummarizeChain::new(Box::new(client));
    let question = "Can you summarize the text for me and give me bullet points?";
    let text = include_str!("./docs/article.txt");
    let answer = mr.summarize(question, text).await;
    println!("SUMMARY: {}", answer);
    Ok(())
}
