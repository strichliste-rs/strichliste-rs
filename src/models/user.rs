#[cfg(feature = "ssr")]
use {
    crate::backend::db::{DBError, DB},
    sqlx::query,
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
}

#[cfg(feature = "ssr")]
impl User {
    pub async fn add_to_db(&mut self, db: &DB) -> Result<(), DBError> {
        let mut conn = db.get_conn().await?;

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

    pub async fn get_by_card_number(db: &DB, card_number: String) -> Result<User, DBError> {
        let mut conn = db.get_conn().await?;

        Ok(User::new())
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
}
