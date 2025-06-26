use super::Money;

#[cfg(feature = "ssr")]
use {
    super::TransactionDB,
    crate::backend::db::{DBError, DB},
    crate::backend::db::{DatabaseId, DatabaseResponse, DatabaseType},
    sqlx::query,
    sqlx::query_as,
    sqlx::Executor,
};

use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, sqlx::Type, sqlx::FromRow)]
pub struct UserDB {
    pub id: i64,
    pub nickname: String,
    pub money: i64,
    pub is_system_user: bool,
}

#[cfg(feature = "ssr")]
impl UserDB {
    pub async fn set_money<T>(
        conn: &mut T,
        user_id: DatabaseId,
        new_value: i64,
    ) -> DatabaseResponse<()>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        _ = query!(
            "
                update Users
                set money = ?
                where id = ?
            ",
            new_value,
            user_id
        )
        .execute(&mut *conn)
        .await
        .map_err(DBError::new)?;

        Ok(())
    }

    pub async fn insert<T>(conn: &mut T, nickname: String) -> DatabaseResponse<DatabaseId>
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
        .map(|elem| elem.id)
    }

    pub async fn insert_card<T>(
        conn: &mut T,
        user_id: DatabaseId,
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
            user_id,
            card_number
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(From::from)
        .map(|_| ())
    }

    pub async fn get_id_by_card_number<T>(
        conn: &mut T,
        card_number: String,
    ) -> DatabaseResponse<Option<DatabaseId>>
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
        .map(|elem| elem.map(|e| e.user_id))
    }

    async fn set_name<T>(conn: &mut T, id: i64, new_value: String) -> Result<(), DBError>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        query!(
            "
                update Users
                set nickname = ?
                where id = ?
            ",
            new_value,
            id
        )
        .execute(&mut *conn)
        .await
        .map_err(From::from)
        .map(|_| ())
    }

    async fn set_card_number<T>(
        conn: &mut T,
        user_id: i64,
        new_value: Option<String>,
    ) -> DatabaseResponse<()>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
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

    async fn get_card_number<T>(conn: &mut T, user_id: i64) -> DatabaseResponse<Option<String>>
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

    async fn get<T>(conn: &mut T, id: i64) -> DatabaseResponse<Option<Self>>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        query_as!(
            UserDB,
            "
                select *
                from Users
                where id = ?
            ",
            id
        )
        .fetch_optional(&mut *conn)
        .await
        .map_err(From::from)
    }

    async fn get_all<T>(conn: &mut T) -> DatabaseResponse<Vec<Self>>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        query_as!(
            UserDB,
            "
                select *
                from Users
            ",
        )
        .fetch_all(&mut *conn)
        .await
        .map_err(From::from)
    }

    async fn get_by_nick<T>(conn: &mut T, nick: String) -> DatabaseResponse<Option<Self>>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        query_as!(
            UserDB,
            "
                select *
                from Users
                where nickname = ?
            ",
            nick
        )
        .fetch_optional(&mut *conn)
        .await
        .map_err(From::from)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct User {
    pub id: i64,
    pub nickname: String,
    pub card_number: Option<String>,
    pub money: Money,
}

#[cfg(feature = "ssr")]
impl User {
    pub async fn set_money<T>(&mut self, conn: &mut T, new_value: i64) -> DatabaseResponse<()>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        UserDB::set_money(&mut *conn, self.id, new_value).await
    }

    pub async fn set_name<T>(&mut self, conn: &mut T, new_value: String) -> DatabaseResponse<()>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        _ = UserDB::set_name(&mut *conn, self.id, new_value.clone()).await?;

        self.nickname = new_value;

        Ok(())
    }

    pub async fn set_card_number<T>(
        &mut self,
        conn: &mut T,
        new_value: Option<String>,
    ) -> DatabaseResponse<()>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        _ = UserDB::set_card_number(&mut *conn, self.id, new_value.clone()).await?;

        self.card_number = new_value;

        Ok(())
    }

    pub async fn create(
        db: &DB,
        nickname: String,
        card_number: Option<String>,
    ) -> DatabaseResponse<DatabaseId> {
        let mut transaction = db.get_conn_transaction().await?;

        let id = UserDB::insert(&mut *transaction, nickname).await?;

        match card_number {
            None => {}
            Some(card_number) => {
                UserDB::insert_card(&mut *transaction, id, card_number).await?;
            }
        }

        transaction.commit().await.map_err(DBError::new)?;
        Ok(id)
    }

    pub async fn get_all(db: &DB) -> Result<Vec<Self>, DBError> {
        let mut conn = db.get_conn().await?;

        let users_db = UserDB::get_all(&mut *conn).await?;
        let mut users = Vec::<User>::new();

        for user_db in users_db.into_iter() {
            users.push(
                Self::get(&mut *conn, user_db.id)
                    .await?
                    .expect("user should exist"),
            )
        }

        return Ok(users);
    }

    pub async fn get_transactions(
        &self,
        db: &DB,
        limit: i64,
    ) -> DatabaseResponse<Vec<TransactionDB>> {
        let mut conn = db.get_conn().await?;
        TransactionDB::get_user_transactions(&mut *conn, self.id, limit, 0).await
    }

    pub async fn get_by_card_number(
        db: &DB,
        card_number: String,
    ) -> DatabaseResponse<Option<User>> {
        let mut conn = db.get_conn().await?;
        let user_id = UserDB::get_id_by_card_number(&mut *conn, card_number).await?;

        match user_id {
            None => return Ok(None),
            Some(user_id) => {
                let user = Self::get(&mut *conn, user_id).await?;

                Ok(user)
            }
        }
    }

    pub async fn get<T>(conn: &mut T, id: DatabaseId) -> DatabaseResponse<Option<User>>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
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
                    id,
                    nickname,
                    card_number,
                    money: money.into(),
                }))
            }
        }
    }

    pub async fn get_by_nick<T>(conn: &mut T, name: String) -> DatabaseResponse<Option<User>>
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
                    id,
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
