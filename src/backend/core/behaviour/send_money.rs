use leptos::prelude::*;

use crate::model::UserId;

#[server]
pub async fn send_money(
    user_id: UserId,
    to_user: String,
    amount: String,
) -> Result<(), ServerFnError> {
    use crate::backend::core::{ServerState, User};
    let state: ServerState = expect_context();

    use crate::{
        backend::{core::behaviour::user_get::get_user, database::GroupDB},
        model::{GroupId, Money, Transaction, TransactionType},
    };
    use axum::http::StatusCode;
    use leptos_axum::redirect;
    use leptos_axum::ResponseOptions;
    use tracing::error;

    let response_opts: ResponseOptions = expect_context();

    let money: Money = match amount.clone().try_into() {
        Ok(value) => value,
        Err(e) => {
            response_opts.set_status(StatusCode::BAD_REQUEST);
            return Err(ServerFnError::new(format!(
                "Failed to convert '{amount}' to internal representation: {e}"
            )));
        }
    };

    if money.value < 0 {
        response_opts.set_status(StatusCode::BAD_REQUEST);
        return Err(ServerFnError::new("Amount to be sent must be > 0!"));
    }

    let sender = match get_user(user_id).await? {
        Some(value) => value,
        None => {
            response_opts.set_status(StatusCode::BAD_REQUEST);
            return Err(ServerFnError::new(
                "The user you are trying to send the money from does not exist!",
            ));
        }
    };

    let db = state.db.lock().await;

    let mut db_trns = match db.get_conn_transaction().await {
        Ok(value) => value,
        Err(e) => {
            error!("Failed to get db transaction: {}", e);
            response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            return Err(ServerFnError::new("Failed to acquire db transaction!"));
        }
    };

    let recipient = match User::get_by_nick(&mut *db_trns, &to_user.clone()).await {
        Ok(val) => val,
        Err(e) => {
            error!("Failed to lookup db: {e}");
            response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            return Err(ServerFnError::new("Failed to lookup db"));
        }
    };

    let recipient = match recipient {
        Some(val) => val,
        None => {
            response_opts.set_status(StatusCode::BAD_REQUEST);
            return Err(ServerFnError::new(format!(
                "Recipient '{to_user}' was not found!"
            )));
        }
    };

    if sender.id == recipient.id {
        response_opts.set_status(StatusCode::BAD_REQUEST);
        return Err(ServerFnError::new(
            "Sending and receiving user must not be the same!",
        ));
    }

    let sender_group = match GroupDB::get_single_group(&mut *db_trns, sender.id).await {
        Ok(val) => val,
        Err(e) => {
            error!("Failed to find single group: {e}");
            response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            return Err(ServerFnError::new("Failed to find single group"));
        }
    };

    let recipient_group = match GroupDB::get_single_group(&mut *db_trns, recipient.id).await {
        Ok(val) => val,
        Err(e) => {
            error!("Failed to find single group: {e}");
            response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            return Err(ServerFnError::new("Failed to find single group"));
        }
    };

    Transaction::create(
        &mut *db_trns,
        GroupId(sender_group),
        GroupId(recipient_group),
        TransactionType::Sent(GroupId(recipient_group)),
        None,
        money,
        &state.settings,
    )
    .await?;

    if let Err(e) = db_trns.commit().await {
        error!("Failed to commit transaction: {}", e);
        response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
        return Err(ServerFnError::new("Failed to apply transaction!"));
    };

    redirect(&format!("/user/{}", sender.id));

    Ok(())
}
