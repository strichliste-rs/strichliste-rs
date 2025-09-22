#![cfg(feature = "ssr")]

use crate::{
    backend::{
        core::User,
        database::{DBError, DatabaseResponse, UserDB, DB},
    },
    model::UserId,
};

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
}
