use serde::{Deserialize, Serialize};

use crate::{backend::core::Barcode, model::Money};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Article {
    pub id: i64,
    pub name: String,
    pub cost: Money,
    pub barcodes: Vec<Barcode>,
    pub is_disabled: bool,
}
