use serde::{Deserialize, Serialize};

use crate::model::PageRequestParams;

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
pub struct PageResponseParams {
    pub offset: usize,
    pub len: usize,
    pub total: usize,
}

impl PageResponseParams {
    pub fn next_params(prev: Option<Self>, limit: usize) -> Option<PageRequestParams> {
        match prev {
            Some(prev) => {
                if prev.has_next() {
                    Some(PageRequestParams {
                        offset: prev.offset + prev.len,
                        limit,
                    })
                } else {
                    None
                }
            }
            None => Some(PageRequestParams { offset: 0, limit }),
        }
    }
    pub fn has_next(&self) -> bool {
        self.offset + self.len < self.total
    }
}
