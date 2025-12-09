use std::sync::Arc;
use corelib::Handler;

struct Uppercase;

impl Handler for Uppercase {
    fn name(&self) -> &'static str { "uppercase" }
    fn handle(&self, msg: &str) {
        println!("[uppercase] {}", msg.to_uppercase());
    }
}

pub fn make() -> Arc<dyn Handler> {
    Arc::new(Uppercase)
}