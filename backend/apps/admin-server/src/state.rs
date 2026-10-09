use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    _counter: Arc<std::sync::atomic::AtomicU64>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            _counter: Arc::new(std::sync::atomic::AtomicU64::new(0)),
        }
    }
}
