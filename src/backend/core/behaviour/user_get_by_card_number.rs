#![cfg(feature = "ssr")]

use crate::backend::{
    core::User,
    database::{DatabaseResponse, UserDB, DB},
};
impl User {
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
}
