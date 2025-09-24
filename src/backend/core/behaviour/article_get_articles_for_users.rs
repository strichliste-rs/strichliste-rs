use {
    crate::{backend::core::Article, model::UserId},
    leptos::prelude::*,
};

#[cfg(feature = "ssr")]
use crate::backend::database::{ArticleDB, DatabaseResponse, DB};

#[cfg(feature = "ssr")]
impl Article {
    pub async fn get_articles_for_user(db: &DB, user_id: UserId) -> DatabaseResponse<Vec<Self>> {
        let mut conn = db.get_conn().await?;

        let mut articles_amount_bought =
            ArticleDB::get_articles_for_user(&mut *conn, user_id).await?;

        //sort by most bought
        articles_amount_bought.sort_by(|a, b| b.1.cmp(&a.1));

        let mut full_articles = Vec::<Article>::new();

        for (article_id, _amount_bought) in articles_amount_bought.iter() {
            full_articles.push(
                Article::get(db, *article_id)
                    .await?
                    .expect("fetched article should exist!"),
            );
        }

        let mut articles = Self::get_all(db, None).await?;

        for article in full_articles.iter() {
            articles.retain(|value| value.id != article.id);
        }

        full_articles.reverse();

        for article in full_articles.into_iter() {
            articles.insert(0, article);
        }

        Ok(articles)
    }
}

#[server]
pub async fn get_articles_per_user(user_id: UserId) -> Result<Vec<Article>, ServerFnError> {
    use crate::backend::core::ServerState;
    let state: ServerState = expect_context();
    use axum::http::StatusCode;
    use leptos_axum::ResponseOptions;
    use tracing::error;

    let response_opts: ResponseOptions = expect_context();

    let db = state.db.lock().await;

    match Article::get_articles_for_user(&db, user_id).await {
        Ok(value) => Ok(value),

        Err(e) => {
            response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            error!("Failed to fetch per user articles: {}", e);
            Err(ServerFnError::new("Failed to fetch per user articles!"))
        }
    }
}
