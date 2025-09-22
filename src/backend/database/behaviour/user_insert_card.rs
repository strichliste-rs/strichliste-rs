#![cfg(feature = "ssr")]

use sqlx::{query, Executor};

use crate::{
    backend::database::{DatabaseResponse, DatabaseType, UserDB},
    model::UserId,
};
impl UserDB {
    pub async fn insert_card<T>(
        conn: &mut T,
        user_id: UserId,
        card_number: String,
    ) -> DatabaseResponse<()>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        query!(
            "
                insert into UserCardNumberMap
                    (user_id, card_number)
                values
                    (?, ?)
            ",
            user_id.0,
            card_number
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(From::from)
        .map(|_| ())
    }
}
