use crate::backend::core::{misc::custom_binary_encoding::Binary, BarcodeDiff};
use leptos::prelude::*;

#[server(input=Binary, output=Binary)]
pub async fn update_article(
    id: i64,
    name: String,
    cost: String,
    barcodes: Option<Vec<BarcodeDiff>>,
) -> Result<(), ServerFnError> {
    use crate::{
        backend::core::{behaviour::article_get::get_article, ServerState},
        model::Money,
    };
    use axum::http::StatusCode;
    use leptos_axum::redirect;
    use leptos_axum::ResponseOptions;
    use tracing::{debug, error};
    let state: ServerState = expect_context();

    let response_opts: ResponseOptions = expect_context();

    let mut article = get_article(id).await?;

    let cost: Money = match cost.clone().try_into() {
        Ok(value) => value,
        Err(e) => {
            response_opts.set_status(StatusCode::BAD_REQUEST);
            return Err(ServerFnError::new(format!(
                "Failed to convert '{}' to internal money representation: {}",
                cost, e
            )));
        }
    };

    let db = &*state.db.lock().await;

    let mut db_transaction = match db.get_conn_transaction().await {
        Ok(value) => value,
        Err(e) => {
            response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            error!("Failed to get transaction for database: {}", e);
            return Err(ServerFnError::new("Failed to get transaction handle!"));
        }
    };

    if article.name != name {
        match article
            .set_name(&mut *db_transaction, name.trim().to_string())
            .await
        {
            Ok(_) => {}
            Err(e) => {
                response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
                error!("Failed to update article name: {}", e);
                return Err(ServerFnError::new(format!(
                    "Failed to update article name: {}",
                    e
                )));
            }
        }
    }

    debug!(
        "Old money: {} | New money: {}",
        article.cost.value, cost.value
    );

    if article.cost.value != cost.value {
        match article.set_cost(&mut *db_transaction, cost).await {
            Ok(_) => {}
            Err(e) => {
                response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
                error!("Failed to update article cost: {}", e);
                return Err(ServerFnError::new("FAiled to update article cost"));
            }
        }
    }

    match barcodes {
        None => {}
        Some(barcodes) => {
            let result = article.set_barcodes(&mut *db_transaction, barcodes).await;

            if let Err(e) = result {
                response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
                error!("Failed to set barcodes: {}", e);
                return Err(ServerFnError::new(format!("Failed to set barcodes: {}", e)));
            }
        }
    }

    match db_transaction.commit().await {
        Ok(_) => {}
        Err(e) => {
            response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            error!("Failed to commit transaction: {}", e);
            return Err(ServerFnError::new("Failed to commit transaction"));
        }
    }

    redirect("/articles");

    Ok(())
}
