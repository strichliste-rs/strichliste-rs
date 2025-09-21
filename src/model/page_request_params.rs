use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
pub struct PageRequestParams {
    pub offset: usize,
    pub limit: usize,
}

impl PageRequestParams {
    pub fn new(limit: usize) -> Self {
        Self { offset: 0, limit }
    }
}
