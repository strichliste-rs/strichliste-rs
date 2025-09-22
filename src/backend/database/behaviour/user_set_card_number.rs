#![cfg(feature = "ssr")]

use sqlx::{query, Executor};

use crate::{
    backend::database::{DatabaseResponse, DatabaseType, UserDB},
    model::UserId,
};

impl UserDB {
    pub async fn set_card_number<T>(
        conn: &mut T,
        user_id: UserId,
        new_value: Option<String>,
    ) -> DatabaseResponse<()>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        let user_id = user_id.0;
        match new_value {
            Some(new_value) => {
                let card_number_exists = Self::get_card_number(&mut *conn, user_id).await?;

                match card_number_exists {
                    None => query!(
                        "
                                insert into UserCardNumberMap
                                    (user_id, card_number)
                                values
                                    (?, ?)
                            ",
                        user_id,
                        new_value
                    )
                    .execute(&mut *conn)
                    .await
                    .map_err(From::from)
                    .map(|_| ()),

                    Some(_) => query!(
                        "
                                update UserCardNumberMap
                                set
                                    card_number = ?
                                where user_id = ?
                            ",
                        new_value,
                        user_id
                    )
                    .execute(&mut *conn)
                    .await
                    .map_err(From::from)
                    .map(|_| ()),
                }
            }

            // No card number was submitted to the server
            None => query!(
                "
                        delete from UserCardNumberMap
                        where user_id = ?
                    ",
                user_id
            )
            .execute(&mut *conn)
            .await
            .map_err(From::from)
            .map(|_| ()),
        }
    }
}
