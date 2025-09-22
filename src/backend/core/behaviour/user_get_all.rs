#![cfg(feature = "ssr")]

use crate::{
    backend::{
        core::User,
        database::{DBError, UserDB, DB},
    },
    model::UserId,
};
impl User {
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
}
