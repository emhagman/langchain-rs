use clients::OpenAILLM;

use crate::callbacks::{LoggerCallback, WebsocketCallback};

mod callbacks;
mod chains;
mod clients;
mod llm;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let question = "Can you summarize the text for me and give me bullet points?";
    let text = include_str!("./docs/article.txt");

    let client = OpenAILLM::new("<OPEN_AI_KEY_HERE>".to_string());

    let mr = chains::SummarizeChain::new(Box::new(client));
    let callback = LoggerCallback {};
    let callback2 = WebsocketCallback {};
    mr.add_callback(&callback);
    mr.add_callback(&callback2);

    let answer = mr.summarize(question, text).await;
    println!("SUMMARY: {}", answer);
    Ok(())
}
