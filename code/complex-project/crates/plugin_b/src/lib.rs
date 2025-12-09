use std::sync::Arc;
use corelib::Handler;

struct WordCount;

impl Handler for WordCount {
    fn name(&self) -> &'static str { "word_count" }
    fn handle(&self, msg: &str) {
        let words = msg.split_whitespace().count();
        println!("[word_count] {} words", words);
    }
}

pub fn make() -> Arc<dyn Handler> {
    Arc::new(WordCount)
}