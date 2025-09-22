#![cfg(feature = "ssr")]

use sqlx::Executor;

use crate::{
    backend::database::{DatabaseResponse, DatabaseType, TransactionDB},
    model::Transaction,
};

impl Transaction {
    pub async fn set_money<T>(&mut self, conn: &mut T, new_value: i64) -> DatabaseResponse<()>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        TransactionDB::set_money(&mut *conn, self.id, new_value).await
    }
}
