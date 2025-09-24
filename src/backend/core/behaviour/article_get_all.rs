use crate::backend::core::Article;
use leptos::prelude::*;

#[cfg(feature = "ssr")]
use {
    crate::backend::{
        core::Barcode,
        database::{ArticleDB, DatabaseResponse, DB},
    },
    tracing::debug,
};

#[cfg(feature = "ssr")]
impl Article {
    pub async fn get_all(db: &DB, limit: Option<i64>) -> DatabaseResponse<Vec<Self>> {
        let mut conn = db.get_conn().await?;

        let articles = ArticleDB::get_all(&mut *conn, limit).await?;

        let mut article_no_db = Vec::new();
        for article in articles {
            let ArticleDB { id, name } = article;
            let article_sounds = ArticleDB::get_sounds(&mut *conn, id).await?;
            debug!("Fetched sounds");
            let article_barcodes = ArticleDB::get_barcodes(&mut *conn, id)
                .await?
                .into_iter()
                .map(|elem| Barcode(elem.barcode_content))
                .collect();
            debug!("Fetched barcodes");
            let cost = ArticleDB::get_latest_cost(&mut *conn, id).await?;
            debug!("Fetched cost");

            article_no_db.push(Article {
                id,
                name,
                cost: cost.into(),
                sounds: article_sounds,
                barcodes: article_barcodes,
            });
        }
        Ok(article_no_db)
    }
}

#[server]
pub async fn get_all_articles(limit: Option<i64>) -> Result<Vec<Article>, ServerFnError> {
    use crate::backend::core::ServerState;
    use tracing::error;
    let state: ServerState = expect_context();
    use axum::http::StatusCode;
    use leptos_axum::ResponseOptions;

    let response_opts: ResponseOptions = expect_context();

    let articles = Article::get_all(&*state.db.lock().await, limit).await;
    articles.map_err(|e| {
        let err = e.to_string();
        error!("Could not fetch articles {}", err);
        response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
        ServerFnError::new(err)
    })
}
