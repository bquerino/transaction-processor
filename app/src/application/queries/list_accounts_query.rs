use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListAccountsQuery;

impl ListAccountsQuery {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ListAccountsQuery {
    fn default() -> Self {
        Self::new()
    }
}
