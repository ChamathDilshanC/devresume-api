use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pagination {
    pub page: usize,
    pub per_page: usize,
    pub total: usize,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: 1,
            per_page: 20,
            total: 0,
        }
    }
}

pub fn format_slug(input: &str) -> String {
    input.to_lowercase().replace(' ', "-").replace(['/', '\\', ':'], "")
}
