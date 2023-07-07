use std::collections::HashMap;

use crate::llm::LLM;

use super::MapReduceChain;

pub struct SummarizeChain<'a> {
    map_reduce_chain: MapReduceChain<'a>,
}

const MAP_PROMPT: &str = "
Your job is to summarize the following document `DOCUMENT` below using the question provided `QUESTION`.
Your summarizing this as part of a larger summary.
 
DOCUMENT: {document}
QUESTION: {question}
";

const REDUCE_PROMPT: &str = "
Your job is to finalize a summary from smaller summarizes we've already collected from documents using the final output `DOCUMENT`.
Be sure to use the `QUESTION` to collect the final summary.
 
DOCUMENT: {document}
QUESTION: {question}
";

impl SummarizeChain<'_> {
    pub fn new(llm: Box<dyn LLM>) -> Self {
        let chain = MapReduceChain::new(llm, MAP_PROMPT, REDUCE_PROMPT);
        Self {
            map_reduce_chain: chain,
        }
    }

    pub async fn summarize(&self, question: &str, text: &str) -> String {
        let mut inputs = HashMap::new();
        inputs.insert("question", question);
        inputs.insert("text", text);
        self.call(inputs).await
    }

    pub async fn call(&self, inputs: HashMap<&str, &str>) -> String {
        self.map_reduce_chain.call(inputs).await
    }
}
