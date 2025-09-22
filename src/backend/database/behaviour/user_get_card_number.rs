#![cfg(feature = "ssr")]

use sqlx::{query, Executor};

use crate::backend::database::{DatabaseResponse, DatabaseType, UserDB};

impl UserDB {
    pub async fn get_card_number<T>(conn: &mut T, user_id: i64) -> DatabaseResponse<Option<String>>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        query!(
            "
                select card_number
                from UserCardNumberMap
                where user_id = ?
            ",
            user_id
        )
        .fetch_optional(&mut *conn)
        .await
        .map_err(From::from)
        .map(|result| result.map(|elem| elem.card_number))
    }
}
