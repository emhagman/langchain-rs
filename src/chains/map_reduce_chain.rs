use std::{collections::HashMap, vec};

use crate::llm::{Message, LLM};
use serde::Serialize;
use tinytemplate::TinyTemplate;

pub struct MapReduceChain<'a> {
    llm: Box<dyn LLM>,
    templates: TinyTemplate<'a>,
}

#[derive(Serialize)]
struct Context {
    question: String,
    document: String,
}

impl<'a> MapReduceChain<'a> {
    pub fn new(llm: Box<dyn LLM>, map_prompt: &'a str, reduce_prompt: &'a str) -> Self {
        let mut tt = TinyTemplate::new();
        tt.add_template("map", map_prompt).unwrap();
        tt.add_template("reduce", reduce_prompt).unwrap();
        Self { llm, templates: tt }
    }

    pub async fn call(&self, inputs: HashMap<&str, &str>) -> String {
        let question = inputs.get("question").unwrap();
        let text = inputs.get("text").unwrap();
        let documents: Vec<String> = text.split("\n\n").map(|s| s.to_string()).collect();
        let mut map_output = "".to_string();
        for t in documents {
            let prompt = self
                .templates
                .render(
                    "map",
                    &Context {
                        question: question.to_string(),
                        document: t,
                    },
                )
                .unwrap();
            let messages = vec![Message { content: &prompt }];
            let response = self.llm.complete(&messages).await.unwrap();
            println!("MAP SUMMARY: {}", response.content);
            map_output += &response.content;
        }
        let reduce_context = Context {
            question: question.to_string(),
            document: map_output,
        };
        let prompt = self.templates.render("reduce", &reduce_context).unwrap();
        let messages = vec![Message { content: &prompt }];
        let response = self.llm.complete(&messages).await.unwrap();
        response.content
    }
}
