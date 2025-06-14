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

#[cfg(feature = "ssr")]
impl BarcodeDB {
    pub async fn get_article_id_from_barcode(
        db: &DB,
        barcode: &String,
    ) -> Result<Option<i64>, DBError> {
        let mut conn = db.get_conn().await?;

        let result = query!(
            "
                select article_id from ArticleBarcodes
                where barcode_content = ?
            ",
            barcode
        )
        .fetch_optional(&mut *conn)
        .await
        .map_err(DBError::new)?;

        Ok(result.map(|value| value.article_id))
    }
}
