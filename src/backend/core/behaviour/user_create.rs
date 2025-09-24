use leptos::prelude::*;

#[cfg(feature = "ssr")]
use crate::{
    backend::{
        core::User,
        database::{DBError, DatabaseResponse, UserDB, DB},
    },
    model::UserId,
};

#[cfg(feature = "ssr")]
impl User {
    pub async fn create(
        db: &DB,
        nickname: String,
        card_number: Option<String>,
    ) -> DatabaseResponse<UserId> {
        use crate::backend::database::GroupDB;

        let mut transaction = db.get_conn_transaction().await?;

        let id = UserDB::insert(&mut *transaction, nickname).await?;

        match card_number {
            None => {}
            Some(card_number) => {
                UserDB::insert_card(&mut *transaction, id, card_number).await?;
            }
        }

        let group = GroupDB::create(&mut *transaction).await?;
        group.link_user(&mut *transaction, id).await?;

        transaction.commit().await.map_err(DBError::new)?;
        Ok(id)
    }
}

#[server]
pub async fn create_user(username: String) -> Result<(), ServerFnError> {
    use crate::backend::core::ServerState;
    let state: ServerState = expect_context();

    use axum::http::StatusCode;
    use leptos_axum::redirect;
    use leptos_axum::ResponseOptions;
    use tracing::{debug, error};

    let response_opts: ResponseOptions = expect_context();
    debug!("Creating account!");

    if username.is_empty() {
        response_opts.set_status(StatusCode::BAD_REQUEST);
        return Err(ServerFnError::new("Name cannot be empty!"));
    }
    let username = username.trim().to_string();

    let user_id = match User::create(&*state.db.lock().await, username, None).await {
        Ok(value) => value,
        Err(e) => {
            response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            error!("Failed to add user: {}", e);
            return Err(ServerFnError::new(e));
        }
    };

    redirect(&format!("/user/{}", user_id));

    Ok(())
}
