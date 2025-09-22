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
    pub async fn get_by_nick<T>(conn: &mut T, name: &String) -> DatabaseResponse<Option<User>>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        match UserDB::get_by_nick(&mut *conn, name).await? {
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
                    id: UserId(id),
                    nickname,
                    card_number,
                    money: money.into(),
                }))
            }
        }
    }
}
