#[cfg(feature = "ssr")]
use {
    crate::backend::db::{DBError, DB},
    sqlx::query,
    sqlx::query_as,
};

use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "ssr", derive(sqlx::Type, sqlx::FromRow))]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Article {
    pub id: Option<i64>,
    pub name: String,
    pub cost: i64,
}

impl Article {
    pub fn new() -> Self {
        Article {
            id: None,
            name: String::new(),
            cost: 0,
        }
    }
}

#[cfg(feature = "ssr")]
impl Article {
    /// Adds the article to DB and sets the id to the resulting id
    pub async fn add_to_db(&mut self, db: &DB) -> Result<(), DBError> {
        let mut conn = db.get_conn().await?;

        let result = query!(
            "
                insert into Articles
                    (name, cost)
                values
                    (?, ?)
                returning id
            ",
            self.name,
            self.cost,
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|e| DBError::new(e.to_string()))?;

        self.id = Some(result.id);

        Ok(())
    }

    // pub async fn add_barcode(&self, db: &DB) -> Result<(), DBError> {
    //     let mut conn = db.get_conn().await?;

    //     let result =
    // }
}
