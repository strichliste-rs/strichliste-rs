#[cfg(feature = "ssr")]
use crate::{
    backend::{core::User, database::UserDB},
    model::{Money, UserId},
};

#[cfg(feature = "ssr")]
use {
    crate::backend::database::{DatabaseResponse, DatabaseType},
    sqlx::Executor,
};

#[cfg(feature = "ssr")]
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

    pub async fn add_money<T>(&mut self, conn: &mut T, money: Money) -> DatabaseResponse<()>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        let new_money = self.money.value + money.value;

        self.set_money(conn, new_money).await
    }
}
