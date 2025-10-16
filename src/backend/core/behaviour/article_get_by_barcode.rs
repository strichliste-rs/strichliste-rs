use {crate::backend::core::Article, leptos::prelude::*};

#[cfg(not(debug_assertions))]
use crate::backend::core::misc::custom_binary_encoding::Binary;
#[cfg(feature = "ssr")]
use crate::backend::database::{ArticleDB, DatabaseResponse, DB};

#[cfg(feature = "ssr")]
impl Article {
    pub async fn get_by_barcode(db: &DB, barcode: String) -> DatabaseResponse<Option<Article>> {
        let mut conn = db.get_conn().await?;

        let result = ArticleDB::get_article_id_by_barcode(&mut *conn, barcode).await?;

        match result {
            None => Ok(None),
            Some(value) => {
                let article = Article::get(db, value).await?;
                Ok(article)
            }
        }
    }
}

#[cfg_attr(not(debug_assertions), server(input=Binary, output=Binary))]
#[cfg_attr(debug_assertions, server)]
pub async fn get_article_by_barcode(barcode: String) -> Result<Option<Article>, ServerFnError> {
    use crate::backend::core::ServerState;
    let state: ServerState = expect_context();
    use axum::http::StatusCode;
    use leptos_axum::ResponseOptions;

    let response_opts: ResponseOptions = expect_context();

    let db = state.db.lock().await;

    match Article::get_by_barcode(&db, barcode).await {
        Err(e) => {
            response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            Err(ServerFnError::new(format!(
                "Failed to get article from db: {}",
                e
            )))
        }

        Ok(value) => Ok(value),
    }
}
