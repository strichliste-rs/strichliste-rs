#![cfg(feature = "ssr")]

use chrono::Utc;
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
        let now = Utc::now();
        query!(
            "
                insert into Users
                    (nickname, money, created_at)
                values
                    (?, ?, ?)
                returning id
            ",
            nickname,
            0,
            now,
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(From::from)
        .map(|elem| elem.id.into())
    }
}
