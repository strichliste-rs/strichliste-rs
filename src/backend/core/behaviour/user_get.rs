#[cfg(feature = "ssr")]
use {
    crate::backend::database::{DatabaseResponse, DatabaseType, UserDB},
    sqlx::Executor,
};

use {
    crate::{
        backend::core::{misc::custom_binary_encoding::Binary, User},
        model::UserId,
    },
    leptos::prelude::*,
};

#[cfg(feature = "ssr")]
impl User {
    pub async fn get<T>(conn: &mut T, id: UserId) -> DatabaseResponse<Option<User>>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        let id = id.0;
        match UserDB::get(&mut *conn, id).await? {
            None => Ok(None),
            Some(value) => {
                let UserDB {
                    id,
                    nickname,
                    money,
                    ..
                } = value;
                let card_number = UserDB::get_card_number(&mut *conn, id).await?;

                Ok(Some(User {
                    id: id.into(),
                    nickname,
                    card_number,
                    money: money.into(),
                }))
            }
        }
    }
}

#[server(input=Binary, output=Binary)]
pub async fn get_user(id: UserId) -> Result<Option<User>, ServerFnError> {
    use crate::backend::core::ServerState;
    let state: ServerState = expect_context();
    use crate::backend::database::{DBUSER_AUFLADUNG_ID, DBUSER_SNACKBAR_ID};
    use axum::http::StatusCode;
    use leptos_axum::ResponseOptions;
    use tracing::error;

    let response_opts: ResponseOptions = expect_context();

    let db = state.db.lock().await;
    let mut conn = match db.get_conn().await {
        Ok(value) => value,
        Err(e) => {
            response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            error!("Failed to create database transaction: {}", e);
            return Err(ServerFnError::new("Failed to create database transaction"));
        }
    };

    if id == DBUSER_AUFLADUNG_ID || id == DBUSER_SNACKBAR_ID {
        response_opts.set_status(StatusCode::BAD_REQUEST);
        return Err(ServerFnError::new("Failed to fetch user"));
    }

    let user = match User::get(&mut *conn, id).await {
        Ok(value) => value,
        Err(e) => {
            error!("Failed to fetch user: {}", e);
            response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            return Err(ServerFnError::new(e));
        }
    };

    Ok(user)
}
