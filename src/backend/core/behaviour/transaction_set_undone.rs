#![cfg(feature = "ssr")]

use sqlx::Executor;

use crate::{
    backend::database::{DatabaseResponse, DatabaseType, TransactionDB},
    model::Transaction,
};

impl Transaction {
    pub async fn set_undone<T>(&mut self, conn: &mut T, new_value: bool) -> DatabaseResponse<()>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        TransactionDB::set_undone(&mut *conn, self.id, new_value).await
    }
}
