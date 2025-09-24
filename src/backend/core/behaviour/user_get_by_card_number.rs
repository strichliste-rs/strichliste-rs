use {crate::backend::core::User, leptos::prelude::*};

#[cfg(feature = "ssr")]
use crate::backend::database::{DatabaseResponse, UserDB, DB};

#[cfg(feature = "ssr")]
impl User {
    pub async fn get_by_card_number(
        db: &DB,
        card_number: String,
    ) -> DatabaseResponse<Option<User>> {
        let mut conn = db.get_conn().await?;
        let user_id = UserDB::get_id_by_card_number(&mut *conn, card_number).await?;

        match user_id {
            None => Ok(None),
            Some(user_id) => {
                let user = Self::get(&mut *conn, user_id).await?;

                Ok(user)
            }
        }
    }
}

#[server]
pub async fn get_user_by_barcode(barcode_string: String) -> Result<Option<User>, ServerFnError> {
    use crate::backend::core::ServerState;
    let state: ServerState = expect_context();
    use axum::http::StatusCode;
    use leptos_axum::ResponseOptions;
    use tracing::{debug, error};

    let response_opts: ResponseOptions = expect_context();

    debug!("Attempting to fetch a user by barcode '{}'", barcode_string);

    if barcode_string.is_empty() {
        return Ok(None);
    }

    let user = match User::get_by_card_number(&*state.db.lock().await, barcode_string).await {
        Ok(user) => user,
        Err(err) => {
            let err = err.to_string();
            error!("Could not fetch user: {}", err);
            response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            return Err(ServerFnError::new("Failed to fetch user"));
        }
    };

    Ok(user)
}
