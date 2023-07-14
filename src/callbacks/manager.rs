pub struct CallbackManager {}

pub struct LoggerCallback {}

impl Callback for LoggerCallback {
    fn on_llm_token(&self, token: &str) {
        println!("LOGGER CALLBACK on_llm_token: {}", token);
    }
    fn on_llm_text(&self, token: &str) {
        println!("LOGGER CALLBACK on_llm_text: {}", token);
    }
    fn on_chain(&self, token: &str) {
        println!("LOGGER CALLBACK on_chain: {}", token);
    }
}

pub struct WebsocketCallback {}

impl Callback for WebsocketCallback {
    fn on_llm_token(&self, token: &str) {
        println!("WEBSOCKET on_llm_token: {}", token);
    }
    fn on_llm_text(&self, token: &str) {
        println!("WEBSOCKET on_llm_text: {}", token);
    }
    fn on_chain(&self, token: &str) {
        println!("WEBSOCKET on_chain: {}", token);
    }
}

pub trait Callback {
    fn on_llm_token(&self, token: &str);
    fn on_llm_text(&self, token: &str);
    fn on_chain(&self, token: &str);
}

impl CallbackManager {
    pub fn on_llm_text<T>(callbacks: &[Box<&T>], token: &str)
    where
        T: Callback + ?Sized,
    {
        for callback in callbacks {
            callback.on_llm_text(token);
        }
    }
    pub fn on_chain<T>(callbacks: &[Box<&T>], token: &str)
    where
        T: Callback + ?Sized,
    {
        for callback in callbacks {
            callback.on_chain(token);
        }
    }
    pub fn on_llm_token<T>(callbacks: &[Box<&T>], token: &str)
    where
        T: Callback + ?Sized,
    {
        for callback in callbacks {
            callback.on_llm_token(token);
        }
    }
}
