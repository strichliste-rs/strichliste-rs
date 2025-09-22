#![cfg(feature = "ssr")]

use sqlx::Executor;

use crate::{
    backend::{
        core::User,
        database::{DatabaseResponse, DatabaseType, UserDB},
    },
    model::UserId,
};

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
