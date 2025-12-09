use std::sync::Arc;

pub trait Handler: Send + Sync {
    fn name(&self) -> &'static str;
    fn handle(&self, msg: &str);
}

pub struct Bus {
    handlers: Vec<Arc<dyn Handler>>,
}

impl Bus {
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
        }
    }

    pub fn register(&mut self, h: Arc<dyn Handler>) {
        self.handlers.push(h);
    }

    pub fn dispatch(&self, msg: &str) {
        for h in &self.handlers {
            h.handle(msg);
        }
    }

    pub fn list(&self) -> impl Iterator<Item = &'static str> + '_ {
        self.handlers.iter().map(|h| h.name())
    }
}