#![cfg(feature = "ssr")]

use crate::{
    backend::{
        core::User,
        database::{DatabaseResponse, DB},
    },
    model::Page,
    models::TransactionDB,
};
impl User {
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
}
