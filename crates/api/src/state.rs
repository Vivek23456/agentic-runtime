use std::sync::Arc;

use runtime::{DefaultExecutor, RuntimeManager};

#[derive(Clone)]
pub struct AppState {
    pub runtime: Arc<RuntimeManager<DefaultExecutor>>,
}