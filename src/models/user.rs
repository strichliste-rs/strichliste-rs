use std::str::FromStr;

#[cfg(feature = "ssr")]
use {
    crate::backend::db::{DBError, DB},
    sqlx::query,
    sqlx::query_as,
};

use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct User {
    pub id: Option<i64>,
    pub nickname: String,
    pub card_number: String,
    pub money: i64,
}

impl User {
    pub fn new() -> Self {
        User {
            id: None,
            nickname: String::new(),
            card_number: String::new(),
            money: 0,
        }
    }

    pub fn get_money(&self) -> String {
        User::calc_money(self.money)
    }

    pub fn calc_money(money: i64) -> String {
        let result = (money as f64) / 100.0;

        let mut string = format!("{result}");

        if money < 0 {
            // the - also gets put into the string
            string = string;
        } else if money > 0 {
            string = String::from_str("+").unwrap() + &string;
        }

        string
    }
}

#[cfg(feature = "ssr")]
impl User {
    pub async fn add_to_db(&mut self, db: &DB) -> Result<(), DBError> {
        let mut conn = db.get_conn().await?;

        let result = query!(
            "
                select *
                from Users
                where nickname = ?
            ",
            self.nickname
        )
        .fetch_optional(&mut *conn)
        .await
        .map_err(|e| DBError::new(e.to_string()))?;

        if result.is_some() {
            return Err(DBError::new("Nickname must be unique".to_string()));
        }

        let result = query!(
            "
                insert into Users
                    (nickname, card_number, money)
                values
                    (?, ?, ?)
                returning id
            ",
            self.nickname,
            self.card_number,
            self.money,
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|e| DBError::new(e.to_string()))?;

        self.id = Some(result.id);

        Ok(())
    }

    pub async fn get_by_card_number(
        db: &DB,
        card_number: &String,
    ) -> Result<Option<User>, DBError> {
        let mut conn = db.get_conn().await?;

        let result = query_as::<_, User>(
            "
                select *
                from Users
                where card_number = ?
            ",
        )
        .bind(card_number)
        .fetch_optional(&mut *conn)
        .await
        .map_err(|err| DBError::new(err.to_string()))?;

        Ok(result)
    }

    pub async fn get_all(db: &DB) -> Result<Vec<Self>, DBError> {
        let mut conn = db.get_conn().await?;

        let result = sqlx::query_as::<_, User>(
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

        let result = sqlx::query_as::<_, User>(
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
            self.money,
            id,
        )
        .execute(&mut *conn)
        .await
        .map_err(|e| DBError::new(e.to_string()))?;

        Ok(())
    }

    pub async fn update_db(&self, db: &DB) -> Result<(), DBError> {
        let mut conn = db.get_conn().await?;

        let id = self.id.unwrap();

        _ = query!(
            "
                update Users
                set
                    nickname = ?,
                    card_number = ?
                where id = ?
            ",
            self.nickname,
            self.card_number,
            id
        )
        .execute(&mut *conn)
        .await
        .map_err(|err| DBError::new(err.to_string()))?;

        Ok(())
    }
}
