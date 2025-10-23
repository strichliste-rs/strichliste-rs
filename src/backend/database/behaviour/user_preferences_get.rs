#![cfg(feature = "ssr")]

use sqlx::{query_as, Executor};

use crate::{
    backend::database::{DatabaseResponse, DatabaseType, UserPreferencesDB},
    model::UserId,
};

impl UserPreferencesDB {
    pub async fn get<T>(conn: &mut T, user_id: UserId) -> DatabaseResponse<Option<Self>>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        let result = query_as!(
            Self,
            "
                select * from UserPreferences
                where user_id = ?
            ",
            user_id.0
        )
        .fetch_optional(&mut *conn)
        .await?;

        Ok(result)
    }
}
