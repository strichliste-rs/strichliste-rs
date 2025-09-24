use leptos::prelude::*;

use crate::model::UserId;

#[server]
pub async fn update_user(
    id: UserId,
    nickname: String,
    card_number: String,
) -> Result<(), ServerFnError> {
    use crate::backend::core::behaviour::user_get::get_user;
    use crate::backend::core::ServerState;
    let state: ServerState = expect_context();
    use crate::backend::core::User;
    use axum::http::StatusCode;
    use leptos_axum::redirect;
    use leptos_axum::ResponseOptions;
    use tracing::{debug, error, warn};

    let response_opts: ResponseOptions = expect_context();

    let user = match get_user(id).await {
        Ok(user) => user,
        Err(err) => {
            error!("Failed to fetch user: {}", err.to_string());
            response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            return Err(ServerFnError::new("Failed to fetch user!"));
        }
    };

    let mut user = match user {
        Some(user) => user,
        None => {
            warn!("No such user with id '{}' exists!", id);
            response_opts.set_status(StatusCode::BAD_REQUEST);
            return Err(ServerFnError::new("No such user exists!"));
        }
    };

    match User::get_by_card_number(&*state.db.lock().await, card_number.clone()).await {
        Ok(value) => match value {
            None => {}
            Some(user) => {
                if user.id != id {
                    warn!("The card number '{}' is already used!", card_number);
                    response_opts.set_status(StatusCode::BAD_REQUEST);
                    return Err(ServerFnError::new("The card number is already used!"));
                }
            }
        },

        Err(e) => {
            error!("Failed to check for existence of the card number: {}", e);
            response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            return Err(ServerFnError::new(
                "Failed to check if the card number is already used!",
            ));
        }
    }

    let card_number = match card_number.len() {
        0 => None,
        _ => Some(card_number),
    };

    let db = &*state.db.lock().await;

    let mut db_trans = match db.get_conn_transaction().await {
        Ok(value) => value,
        Err(e) => {
            response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            error!("Failed to get database handle: {}", e);
            return Err(ServerFnError::new("Faile to get a database handle!"));
        }
    };

    match user.set_name(&mut *db_trans, nickname).await {
        Ok(_) => {}
        Err(e) => {
            response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            error!("Failed to set a new username: {}", e);
            return Err(ServerFnError::new("Failed to set a new username!"));
        }
    }

    debug!(
        "Changing card number for user '{}' to '{:?}'",
        user.id, user.card_number
    );

    match user.set_card_number(&mut *db_trans, card_number).await {
        Ok(_) => {}
        Err(e) => {
            response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            error!("Failed to set a new card number: {}", e);
            return Err(ServerFnError::new("Failed to set a new card number!"));
        }
    }

    match db_trans.commit().await {
        Ok(_) => {}
        Err(e) => {
            response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            error!("Failed to commit database transaction: {}", e);
            return Err(ServerFnError::new(
                "Failed to commit the database transaction",
            ));
        }
    }

    redirect(&format!("/user/{}", id));

    Ok(())
}
