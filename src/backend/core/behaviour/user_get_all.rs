use {crate::backend::core::User, leptos::prelude::*};

#[cfg(feature = "ssr")]
use crate::{
    backend::database::{DBError, UserDB, DB},
    model::UserId,
};

#[cfg(feature = "ssr")]
impl User {
    pub async fn get_all(db: &DB) -> Result<Vec<Self>, DBError> {
        let mut conn = db.get_conn().await?;

        let users_db = UserDB::get_all(&mut *conn).await?;
        let mut users = Vec::<User>::new();

        for user_db in users_db.into_iter() {
            users.push(
                Self::get(&mut *conn, UserId(user_db.id))
                    .await?
                    .expect("user should exist"),
            )
        }

        Ok(users)
    }
}

#[server(output=server_fn::codec::Cbor)]
pub async fn get_all_users() -> Result<Vec<User>, ServerFnError> {
    use crate::backend::core::ServerState;
    let state: ServerState = expect_context();
    use axum::http::StatusCode;
    use leptos_axum::ResponseOptions;
    use tracing::error;

    let response_opts: ResponseOptions = expect_context();

    let users = match User::get_all(&*state.db.lock().await).await {
        Ok(users) => users,
        Err(err) => {
            let err = err.to_string();
            error!("Could not fetch users: {}", err);
            response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            return Err(ServerFnError::new(err));
        }
    };

    Ok(users)
}
