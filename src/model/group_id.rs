use std::fmt;

use serde::{Deserialize, Serialize};

use crate::model::DatabaseId;

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
pub struct GroupId(pub DatabaseId);
impl fmt::Display for GroupId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
