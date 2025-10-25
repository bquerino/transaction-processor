use crate::application::Mediator;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub mediator: Arc<Mediator>,
}

impl AppState {
    pub fn new(mediator: Mediator) -> Self {
        Self {
            mediator: Arc::new(mediator),
        }
    }
}
