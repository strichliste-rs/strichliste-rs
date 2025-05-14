#[cfg(feature = "ssr")]
use {
    crate::backend::db::{DBError, DB},
    sqlx::query,
};

pub struct User {
    pub id: i64,
    pub nickname: String,
    pub card_number: String,
}

#[cfg(feature = "ssr")]
impl User {
    pub async fn add_to_db(&mut self, db: &DB) -> Result<(), DBError> {
        let mut conn = db.get_conn().await?;

        let result = query!(
            "
                insert into Users
                    (nickname, card_number)
                values
                    (?, ?)
                returning id
            ",
            self.nickname,
            self.card_number
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|e| DBError::new(e.to_string()))?;

        self.id = result.id;

        Ok(())
    }

    pub async fn get_by_card_number(db: &DB, card_number: String) -> Result<User, DBError> {
        let mut conn = db.get_conn().await?;

        Ok(User {
            id: 0,
            nickname: String::new(),
            card_number: String::new(),
        })
    }
}
