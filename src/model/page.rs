use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Page<T> {
    pub params: PageResponseParams,
    pub items: Vec<T>,
}

use crate::model::{PageRequestParams, PageResponseParams};

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
