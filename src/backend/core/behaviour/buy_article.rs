use crate::model::{Transaction, UserId};
use leptos::prelude::*;

#[server]
pub async fn buy_article_by_id(
    user_id: UserId,
    article_id: i64,
) -> Result<Transaction, ServerFnError> {
    use crate::backend::core::behaviour::article_get::get_article;
    use crate::backend::core::Group;
    use crate::backend::core::ServerState;
    use crate::{backend::database::DBGROUP_SNACKBAR_ID, model::TransactionType};
    use tracing::error;
    let state: ServerState = expect_context();
    use axum::http::StatusCode;
    use leptos_axum::ResponseOptions;

    let response_opts: ResponseOptions = expect_context();

    let article = get_article(article_id).await?;

    let db = state.db.lock().await;
    let mut db_trans = match db.get_conn_transaction().await {
        Ok(value) => value,
        Err(e) => {
            response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            error!("Failed to create db_transaction: {}", e);
            return Err(ServerFnError::new("Failed to create db connection"));
        }
    };

    let user_group = match Group::get_user_group_id(&mut *db_trans, user_id).await {
        Ok(value) => value,
        Err(e) => {
            response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            error!("Failed to get user single group: {}", e);
            return Err(ServerFnError::new(
                "Failed to find single user group for user",
            ));
        }
    };

    let transaction_id = Transaction::create(
        &mut *db_trans,
        user_group,
        DBGROUP_SNACKBAR_ID,
        TransactionType::Bought(article_id),
        Some(article.name.clone()),
        article.cost,
        &state.settings,
    )
    .await?;

    let transaction = match Transaction::get(&mut *db_trans, transaction_id, user_id).await {
        Ok(Some(o)) => o,
        _ => {
            response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            error!("Failed to read back db_transaction");
            return Err(ServerFnError::new("Failed to read back db_transaction"));
        }
    };

    match db_trans.commit().await {
        Ok(_) => {}
        Err(e) => {
            response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            error!("Failed to commit transaction: {e}");
            return Err(ServerFnError::new("Failed to commit the db transaction!"));
        }
    }

    Ok(transaction)
}
