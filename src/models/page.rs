use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Page<T> {
    pub params: PageResponseParams,
    pub items: Vec<T>,
}

use crate::model::PageResponseParams;
#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
pub struct PageRequestParams {
    pub offset: usize,
    pub limit: usize,
}

impl<T> Page<T> {
    pub fn new(request: PageRequestParams, total: usize, items: Vec<T>) -> Self {
        Self {
            params: PageResponseParams {
                offset: request.offset,
                len: items.len(),
                total,
            },
            items,
        }
    }
}

impl PageRequestParams {
    pub fn new(limit: usize) -> Self {
        Self { offset: 0, limit }
    }
}
