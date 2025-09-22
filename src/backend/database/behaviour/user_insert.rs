#![cfg(feature = "ssr")]

use sqlx::query;
use sqlx::Executor;

use crate::{
    backend::database::{DatabaseResponse, DatabaseType, UserDB},
    model::UserId,
};

impl UserDB {
    pub async fn insert<T>(conn: &mut T, nickname: String) -> DatabaseResponse<UserId>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        query!(
            "
                insert into Users
                    (nickname, money)
                values
                    (?, ?)
                returning id
            ",
            nickname,
            0
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(From::from)
        .map(|elem| elem.id.into())
    }
}
