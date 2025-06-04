use std::path::{self, PathBuf};

#[cfg(feature = "ssr")]
use {
    crate::backend::db::{DBError, DB},
    sqlx::query,
    sqlx::query_as,
};

use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "ssr", derive(sqlx::Type, sqlx::FromRow))]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ArticleDB {
    pub id: i64,
    pub name: String,
    pub cost: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type, sqlx::FromRow))]
pub struct ArticleSound {
    pub id: i64,
    pub name: String,
    pub path: String,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Barcode(String);

#[cfg_attr(feature = "ssr", derive(sqlx::Type, sqlx::FromRow))]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct BarcodeDB {
    article_id: i64,
    barcode_content: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Article {
    pub id: i64,
    pub name: String,
    pub cost: i64,
    pub sounds: Vec<ArticleSound>,
    pub barcodes: Vec<Barcode>,
}

impl Article {
    pub fn new(id: i64, name: String, cost: i64) -> Self {
        Self {
            id,
            name,
            cost,
            barcodes: Vec::new(),
            sounds: Vec::new(),
        }
    }
}

impl From<(ArticleDB, Vec<ArticleSound>, Vec<BarcodeDB>)> for Article {
    fn from(value: (ArticleDB, Vec<ArticleSound>, Vec<BarcodeDB>)) -> Self {
        let ArticleDB { id, name, cost } = value.0.clone();
        Self {
            id,
            name,
            cost,
            sounds: value.1,
            barcodes: value.2.into_iter().map(|e| e.into()).collect(),
        }
    }
}

impl From<BarcodeDB> for Barcode {
    fn from(value: BarcodeDB) -> Self {
        Self(value.barcode_content)
    }
}

#[cfg(feature = "ssr")]
impl Article {
    pub async fn get_all_from_db(db: &DB) -> Result<Vec<Self>, DBError> {
        let mut conn = db.get_conn().await?;
        let article_result = sqlx::query_as::<_, ArticleDB>(
            "
                select * from Articles
            ",
        )
        .fetch_all(&mut *conn)
        .await
        .map_err(DBError::new)?;

        let mut article_no_db = Vec::new();
        for article in article_result {
            let article_sounds = article.get_sounds(db).await?;
            let article_barcodes = article.get_barcodes(db).await?;
            article_no_db.push((article, article_sounds, article_barcodes).into());
        }
        Ok(article_no_db)
    }
    pub async fn get_from_db(db: &DB, id: i64) -> Result<Option<Self>, DBError> {
        let mut conn = db.get_conn().await?;
        let article_result = sqlx::query_as::<_, ArticleDB>(
            "
                select * from Articles
                where id = ?
            ",
        )
        .bind(id)
        .fetch_optional(&mut *conn)
        .await
        .map_err(DBError::new)?;
        match article_result {
            Some(article) => {
                let article_sounds = article.get_sounds(db).await?;
                let article_barcodes = article.get_barcodes(db).await?;
                Ok(Some((article, article_sounds, article_barcodes).into()))
            }
            None => Ok(None),
        }
    }
}
#[cfg(feature = "ssr")]
impl ArticleDB {
    pub async fn get_sounds(&self, db: &DB) -> Result<Vec<ArticleSound>, DBError> {
        let mut conn = db.get_conn().await?;
        let sound_ids = sqlx::query!(
            "
            select sound_id from ArticleSoundMap
            where article_id = ?
                
            ",
            self.id
        )
        .fetch_all(&mut *conn)
        .await
        .map_err(DBError::new)?;

        let mut sounds = Vec::new();
        for sound_id in sound_ids {
            let sound = sqlx::query_as::<_, ArticleSound>(
                "
                    select * from ArticleSounds
                    where id = ?
                ",
            )
            .bind(sound_id.sound_id)
            .fetch_one(&mut *conn)
            .await
            .map_err(DBError::new)?;
            sounds.push(sound);
        }
        Ok(sounds)
    }
    pub async fn get_barcodes(&self, db: &DB) -> Result<Vec<BarcodeDB>, DBError> {
        let mut conn = db.get_conn().await?;

        sqlx::query_as::<_, BarcodeDB>(
            "
                    select * from ArticleBarcodes
                    where article_id = ?
                ",
        )
        .bind(self.id)
        .fetch_all(&mut *conn)
        .await
        .map_err(DBError::new)
    }
}
