use leptos::prelude::*;
#[cfg(feature = "ssr")]
use sqlx::Executor;

#[cfg(not(debug_assertions))]
use crate::backend::core::misc::custom_binary_encoding::Binary;

#[cfg(feature = "ssr")]
use crate::backend::database::{DatabaseResponse, DatabaseType, UserDB};
use crate::model::{SetUserPreferencesError, UserId, UserPreferences};

#[cfg_attr(not(debug_assertions), server(input=Binary, output=Binary))]
#[cfg_attr(debug_assertions, server)]
pub async fn set_user_preferences(
    user_id: UserId,
    preferences: UserPreferences,
) -> Result<(), SetUserPreferencesError> {
    use crate::backend::core::ServerState;
    use axum::http::StatusCode;
    use leptos_axum::ResponseOptions;
    let state: ServerState = expect_context();

    let response_opts: ResponseOptions = expect_context();

    let db = state.db.lock().await;

    let mut conn = db.get_conn().await?;

    if (UserDB::get(&mut *conn, user_id.0).await?).is_none() {
        response_opts.set_status(StatusCode::BAD_REQUEST);
        return Err(SetUserPreferencesError::UserDoesNotExist(user_id));
    }

    UserPreferences::set(&mut *conn, user_id, preferences).await?;

    Ok(())
}

#[cfg(feature = "ssr")]
impl UserPreferences {
    pub async fn set<T>(conn: &mut T, user_id: UserId, preferences: Self) -> DatabaseResponse<()>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        use crate::backend::database::UserPreferencesDB;

        UserPreferencesDB::set(&mut *conn, user_id, preferences).await?;

        Ok(())
    }
}
