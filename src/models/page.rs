use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
pub struct PageRequestParams {
    pub offset: usize,
    pub limit: usize,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Page<T> {
    pub params: PageResponseParams,
    pub items: Vec<T>,
}

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
