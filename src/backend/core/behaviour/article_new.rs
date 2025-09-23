use leptos::prelude::*;

#[cfg(feature = "ssr")]
use crate::{
    backend::{
        core::Article,
        database::{ArticleDB, DBError, DatabaseResponse, DB},
    },
    model::Money,
};

#[cfg(feature = "ssr")]
impl Article {
    pub async fn new(db: &DB, name: String, cost: Money) -> DatabaseResponse<Self> {
        let mut transaction = db.get_conn_transaction().await?;

        let id = ArticleDB::create(&mut transaction, name, cost.value).await?;

        transaction.commit().await.map_err(DBError::new)?;

        let article = Article::get(db, id).await?;

        Ok(article.expect("Newly created article should exist!"))
    }
}
#[server]
pub async fn create_article(name: String, cost: String) -> Result<(), ServerFnError> {
    use crate::backend::core::ServerState;
    use axum::http::StatusCode;
    use leptos_axum::redirect;
    use leptos_axum::ResponseOptions;
    use tracing::{debug, error};
    let state: ServerState = expect_context();

    let response_opts: ResponseOptions = expect_context();
    debug!("Creating article!");

    if name.is_empty() {
        response_opts.set_status(StatusCode::BAD_REQUEST);
        return Err(ServerFnError::new("Name cannot be empty!"));
    }

    if cost.is_empty() {
        response_opts.set_status(StatusCode::BAD_REQUEST);
        return Err(ServerFnError::new("Cost cannot be empty!"));
    }

    let money: Money = match cost.try_into() {
        Ok(value) => value,
        Err(err) => {
            response_opts.set_status(StatusCode::BAD_REQUEST);
            return Err(ServerFnError::new(err));
        }
    };

    let article = Article::new(&*state.db.lock().await, name, money).await;

    let article = match article {
        Ok(value) => value,
        Err(e) => {
            response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            error!("Failed to create new article: {}", e);
            return Err(ServerFnError::new("Failed to create article!"));
        }
    };

    redirect(&format!("/articles/{}", article.id));

    Ok(())
}
