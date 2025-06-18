use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use {
    crate::backend::db::{DBError, DB},
    sqlx::{query, query_as},
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Barcode(pub String);

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type, sqlx::FromRow))]
pub struct BarcodeDB {
    pub article_id: i64,
    pub barcode_content: String,
}

impl From<BarcodeDB> for Barcode {
    fn from(value: BarcodeDB) -> Self {
        Self(value.barcode_content)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum BarcodeDiff {
    Removed(String),
    Added(String),
}
