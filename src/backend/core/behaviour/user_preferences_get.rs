use leptos::prelude::*;
#[cfg(feature = "ssr")]
use sqlx::Executor;

#[cfg(not(debug_assertions))]
use crate::backend::core::misc::custom_binary_encoding::Binary;

use crate::model::{GetUserPreferencesError, UserId, UserPreferences};

#[cfg(feature = "ssr")]
use crate::backend::database::{DatabaseResponse, DatabaseType, UserPreferencesDB};

#[cfg_attr(not(debug_assertions), server(input=Binary, output=Binary))]
#[cfg_attr(debug_assertions, server)]
pub async fn get_user_preferences(
    user_id: UserId,
) -> Result<UserPreferences, GetUserPreferencesError> {
    use crate::backend::core::ServerState;
    let state: ServerState = expect_context();

    let db = state.db.lock().await;

    let mut conn = db.get_conn().await?;

    let preference = UserPreferences::get(&mut *conn, user_id).await?;

    Ok(preference)
}

#[cfg(feature = "ssr")]
impl UserPreferences {
    pub async fn get<T>(conn: &mut T, user_id: UserId) -> DatabaseResponse<Self>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        let value = UserPreferencesDB::get(&mut *conn, user_id)
            .await?
            .map(Into::<UserPreferences>::into)
            .unwrap_or_default();

        Ok(value)
    }
}
