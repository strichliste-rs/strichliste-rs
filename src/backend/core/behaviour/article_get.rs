use crate::backend::core::{misc::custom_binary_encoding::Binary, Article};
use leptos::prelude::*;

#[cfg(feature = "ssr")]
use crate::backend::{
    core::Barcode,
    database::{ArticleDB, DatabaseResponse, DB},
};
#[cfg(feature = "ssr")]
impl Article {
    pub async fn get(db: &DB, id: i64) -> DatabaseResponse<Option<Self>> {
        let mut conn = db.get_conn().await?;

        match ArticleDB::get_single(&mut *conn, id).await? {
            Some(article) => {
                let article_sounds = ArticleDB::get_sounds(&mut *conn, article.id).await?;
                let article_barcodes = ArticleDB::get_barcodes(&mut *conn, article.id)
                    .await?
                    .into_iter()
                    .map(|elem| Barcode(elem.barcode_content))
                    .collect();

                let cost = ArticleDB::get_latest_cost(&mut *conn, article.id).await?;

                let ArticleDB { id, name } = article;
                Ok(Some(Article {
                    id,
                    name,
                    cost: cost.into(),
                    sounds: article_sounds,
                    barcodes: article_barcodes,
                }))
            }
            None => Ok(None),
        }
    }
}
#[server(input=Binary, output=Binary)]
pub async fn get_article(article_id: i64) -> Result<Article, ServerFnError> {
    use crate::backend::core::ServerState;
    let state: ServerState = expect_context();
    use axum::http::StatusCode;
    use leptos_axum::ResponseOptions;

    let response_opts: ResponseOptions = expect_context();

    let article = Article::get(&*state.db.lock().await, article_id).await;

    let article = match article {
        Ok(value) => value,
        Err(e) => {
            response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            return Err(ServerFnError::new(format!(
                "Error getting article from db: {}",
                e
            )));
        }
    };

    match article {
        None => {
            response_opts.set_status(StatusCode::BAD_REQUEST);
            Err(ServerFnError::new(format!(
                "Unknown Article id '{}'",
                article_id
            )))
        }

        Some(value) => Ok(value),
    }
}
