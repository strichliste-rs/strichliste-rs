#![cfg(feature = "ssr")]

use sqlx::{query, Executor};

use crate::{
    backend::database::{DatabaseResponse, DatabaseType, UserPreferencesDB},
    model::{UserId, UserPreferences},
};

impl UserPreferencesDB {
    pub async fn set<T>(
        conn: &mut T,
        user_id: UserId,
        preferences: UserPreferences,
    ) -> DatabaseResponse<()>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        let UserPreferences {
            alternative_coloring,
        } = preferences;

        if UserPreferencesDB::exists(&mut *conn, user_id).await? {
            query!(
                "
                    update UserPreferences
                    set alternative_coloring = ?
                    where user_id = ?
                ",
                alternative_coloring,
                user_id.0
            )
            .execute(&mut *conn)
            .await?;
        } else {
            query!(
                "
                    insert into
                        UserPreferences(user_id, alternative_coloring)
                    values
                        (?, ?)
                ",
                user_id.0,
                alternative_coloring
            )
            .execute(&mut *conn)
            .await?;
        }

        Ok(())
    }

    pub async fn exists<T>(conn: &mut T, user_id: UserId) -> DatabaseResponse<bool>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        let result = query!(
            "
                select count(*) as count from UserPreferences
                where user_id = ?
            ",
            user_id.0
        )
        .fetch_one(&mut *conn)
        .await?;

        Ok(result.count != 0)
    }
}
