use std::str::FromStr;

use super::{Money, Transaction, TransactionDB};

#[cfg(feature = "ssr")]
use {
    crate::backend::db::{DBError, DB},
    sqlx::query,
    sqlx::query_as,
};

use serde::{Deserialize, Serialize};
use tracing::debug;

#[cfg_attr(feature = "ssr", derive(sqlx::Type, sqlx::FromRow))]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct UserDB {
    pub id: i64,
    pub nickname: String,
    pub money: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct User {
    pub id: Option<i64>,
    pub nickname: String,
    pub card_number: Option<String>,
    pub money: Money,
}

impl User {
    pub fn new() -> Self {
        User {
            id: None,
            nickname: String::new(),
            card_number: None,
            money: Money::new(),
        }
    }
}

#[cfg(feature = "ssr")]
impl User {
    pub async fn add_to_db(&mut self, db: &DB) -> Result<(), DBError> {
        let mut transaction = db.get_conn_transaction().await?;

        let result = query!(
            "
                select *
                from Users
                where nickname = ?
            ",
            self.nickname
        )
        .fetch_optional(&mut *transaction)
        .await
        .map_err(|e| DBError::new(e.to_string()))?;

        if result.is_some() {
            return Err(DBError::new("Nickname must be unique".to_string()));
        }

        let result = query!(
            "
                insert into Users
                    (nickname, money)
                values
                    (?, ?)
                returning id
            ",
            self.nickname,
            self.money.value,
        )
        .fetch_one(&mut *transaction)
        .await
        .map_err(|e| DBError::new(e.to_string()))?;

        let user_id = result.id;

        if self.card_number.is_some() {
            _ = query!(
                "
                    insert into UserCardNumberMap
                        (user_id, card_number)
                    values
                        (?, ?)
                ",
                user_id,
                self.card_number,
            )
            .execute(&mut *transaction)
            .await
            .map_err(|e| DBError::new(e))?;
        }

        _ = transaction.commit().await.map_err(|e| DBError::new(e))?;

        self.id = Some(user_id);

        Ok(())
    }

    pub async fn get_by_card_number(
        db: &DB,
        card_number: &String,
    ) -> Result<Option<User>, DBError> {
        let mut conn = db.get_conn().await?;

        let result = query!(
            "
                select *
                from UserCardNumberMap
                where card_number = ?
            ",
            card_number,
        )
        .fetch_optional(&mut *conn)
        .await
        .map_err(|err| DBError::new(err.to_string()))?;

        if result.is_none() {
            return Ok(None);
        }

        let result_row = result.unwrap();
        let result = query_as::<_, UserDB>(
            "
                select *
                from Users
                where id = ?
            ",
        )
        .bind(result_row.user_id)
        .fetch_optional(&mut *conn)
        .await
        .map_err(|e| DBError::new(e))?;

        let result = result.map(|user| {
            let UserDB {
                id,
                nickname,
                money,
            } = user;

            User {
                id: Some(id),
                nickname,
                card_number: Some(card_number.clone()),
                money: money.into(),
            }
        });

        Ok(result)
    }

    pub async fn get_all(db: &DB) -> Result<Vec<UserDB>, DBError> {
        let mut conn = db.get_conn().await?;

        let result = sqlx::query_as::<_, UserDB>(
            "
                select *
                from Users
            ",
        )
        .fetch_all(&mut *conn)
        .await
        .map_err(|e| DBError::new(e.to_string()))?;

        return Ok(result);
    }

    pub async fn get_by_id(db: &DB, id: i64) -> Result<Option<User>, DBError> {
        let mut conn = db.get_conn().await?;

        let result = sqlx::query_as::<_, UserDB>(
            "
                select *
                from Users
                where id = ?
            ",
        )
        .bind(id)
        .fetch_optional(&mut *conn)
        .await
        .map_err(|e| DBError::new(e.to_string()))?;

        let result = match result {
            None => None,
            Some(user) => {
                let result_row = query!(
                    "
                        select *
                        from UserCardNumberMap
                        where user_id = ?
                    ",
                    user.id
                )
                .fetch_optional(&mut *conn)
                .await
                .map_err(|e| DBError::new(e))?;

                let UserDB {
                    id,
                    nickname,
                    money,
                } = user;

                Some(User {
                    id: Some(id),
                    nickname,
                    card_number: result_row.map(|number| number.card_number),
                    money: money.into(),
                })
            }
        };

        Ok(result)
    }

    pub async fn update_money(&self, db: &DB) -> Result<(), DBError> {
        let mut conn = db.get_conn().await?;

        let id = self.id.unwrap();

        _ = query!(
            "
                update Users
                set money = ?
                where id = ?
            ",
            self.money.value,
            id,
        )
        .execute(&mut *conn)
        .await
        .map_err(|e| DBError::new(e.to_string()))?;

        Ok(())
    }

    pub async fn update_db(&self, db: &DB) -> Result<(), DBError> {
        let mut transaction = db.get_conn_transaction().await?;

        let id = self.id.unwrap();

        _ = query!(
            "
                update Users
                set
                    nickname = ?
                where id = ?
            ",
            self.nickname,
            id
        )
        .execute(&mut *transaction)
        .await
        .map_err(|err| DBError::new(err.to_string()))?;

        match &self.card_number {
            Some(card_number) => {
                let result = query!(
                    "
                        select *
                        from UserCardNumberMap
                        where user_id = ?
                    ",
                    id
                )
                .fetch_optional(&mut *transaction)
                .await
                .map_err(|e| DBError::new(e))?;

                match result {
                    None => {
                        debug!("No cardnumber was present. Inserting a new one!");
                        _ = query!(
                            "
                                insert into UserCardNumberMap
                                    (user_id, card_number)
                                values
                                    (?, ?)
                            ",
                            id,
                            card_number
                        )
                        .execute(&mut *transaction)
                        .await
                        .map_err(|e| DBError::new(e))?;
                    }

                    Some(_) => {
                        debug!("Found an old card number. Updating!");
                        _ = query!(
                            "
                                update UserCardNumberMap
                                set
                                    card_number = ?
                                where user_id = ?
                            ",
                            card_number,
                            id
                        )
                        .execute(&mut *transaction)
                        .await
                        .map_err(|e| DBError::new(e))?;
                    }
                }
            }

            None => {
                debug!("No card number was sent to server. Deleting entry in table");
                _ = query!(
                    "
                        delete from UserCardNumberMap
                        where user_id = ?
                    ",
                    id
                )
                .execute(&mut *transaction)
                .await
                .map_err(|e| DBError::new(e))?;
            }
        }

        debug!("Updated user. Committing transaction!");

        _ = transaction.commit().await.map_err(|e| DBError::new(e))?;

        Ok(())
    }

    pub async fn get_transactions(
        &self,
        db: &DB,
        limit: i64,
    ) -> Result<Vec<TransactionDB>, DBError> {
        TransactionDB::get_user_transactions(db, self.id.unwrap(), limit).await
    }
}
