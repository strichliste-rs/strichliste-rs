#![cfg(feature = "ssr")]

use sqlx::{query, Executor};

use crate::{
    backend::database::{DatabaseResponse, DatabaseType, UserDB},
    model::UserId,
};

impl UserDB {
    pub async fn get_id_by_card_number<T>(
        conn: &mut T,
        card_number: String,
    ) -> DatabaseResponse<Option<UserId>>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        query!(
            "
                select user_id
                from UserCardNumberMap
                where card_number = ?
            ",
            card_number
        )
        .fetch_optional(&mut *conn)
        .await
        .map_err(From::from)
        .map(|elem| elem.map(|e| e.user_id.into()))
    }
}
