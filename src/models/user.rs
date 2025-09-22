#[cfg(feature = "ssr")]
use crate::{
    backend::{core::User, database::UserDB},
    model::Page,
    model::{Money, UserId},
};

#[cfg(feature = "ssr")]
use {
    super::TransactionDB,
    crate::backend::database::{DBError, DB},
    crate::backend::database::{DatabaseResponse, DatabaseType},
    sqlx::Executor,
};

#[cfg(feature = "ssr")]
impl User {
    pub async fn create(
        db: &DB,
        nickname: String,
        card_number: Option<String>,
    ) -> DatabaseResponse<UserId> {
        use crate::backend::database::GroupDB;

        let mut transaction = db.get_conn_transaction().await?;

        let id = UserDB::insert(&mut *transaction, nickname).await?;

        match card_number {
            None => {}
            Some(card_number) => {
                UserDB::insert_card(&mut *transaction, id, card_number).await?;
            }
        }

        let group = GroupDB::create(&mut *transaction).await?;
        group.link_user(&mut *transaction, id).await?;

        transaction.commit().await.map_err(DBError::new)?;
        Ok(id)
    }

    pub async fn get_all(db: &DB) -> Result<Vec<Self>, DBError> {
        let mut conn = db.get_conn().await?;

        let users_db = UserDB::get_all(&mut *conn).await?;
        let mut users = Vec::<User>::new();

        for user_db in users_db.into_iter() {
            users.push(
                Self::get(&mut *conn, UserId(user_db.id))
                    .await?
                    .expect("user should exist"),
            )
        }

        Ok(users)
    }

    pub async fn get_transactions(
        &self,
        db: &DB,
        limit: usize,
    ) -> DatabaseResponse<Page<TransactionDB>> {
        use crate::model::PageRequestParams;

        let mut conn = db.get_conn().await?;
        TransactionDB::get_user_transactions(&mut *conn, self.id, PageRequestParams::new(limit))
            .await
    }

    pub async fn get_by_card_number(
        db: &DB,
        card_number: String,
    ) -> DatabaseResponse<Option<User>> {
        let mut conn = db.get_conn().await?;
        let user_id = UserDB::get_id_by_card_number(&mut *conn, card_number).await?;

        match user_id {
            None => Ok(None),
            Some(user_id) => {
                let user = Self::get(&mut *conn, user_id).await?;

                Ok(user)
            }
        }
    }

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
