#![cfg(feature = "ssr")]

use sqlx::Executor;

use crate::{
    backend::database::{DBError, DatabaseResponse, DatabaseType, TransactionDB},
    model::{DatabaseId, Transaction, UserId},
};

impl Transaction {
    pub async fn get<T>(
        conn: &mut T,
        id: DatabaseId,
        user_id: UserId,
    ) -> DatabaseResponse<Option<Self>>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        use crate::backend::database::GroupDB;

        let transaction_db = TransactionDB::get(conn, id).await?;

        let transaction_db = match transaction_db {
            Some(value) => value,
            None => return Ok(None),
        };

        let user_groups = GroupDB::get_groups(&mut *conn, user_id).await?;

        let transaction: Transaction = (transaction_db, &user_groups)
            .try_into()
            .map_err(DBError::new)?;
        Ok(Some(transaction))
    }
}
