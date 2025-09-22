#![cfg(feature = "ssr")]

use sqlx::Executor;

use crate::backend::{
    core::User,
    database::{DatabaseResponse, DatabaseType, UserDB},
};

impl User {
    pub async fn set_card_number<T>(
        &mut self,
        conn: &mut T,
        new_value: Option<String>,
    ) -> DatabaseResponse<()>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        UserDB::set_card_number(&mut *conn, self.id, new_value.clone()).await?;

        self.card_number = new_value;

        Ok(())
    }
}
