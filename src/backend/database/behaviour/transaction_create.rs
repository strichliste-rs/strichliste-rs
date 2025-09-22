#![cfg(feature = "ssr")]

use chrono::Utc;
use sqlx::{query, Executor};

use crate::{
    backend::database::{DatabaseResponse, DatabaseType, TransactionDB},
    model::GroupId,
    models::DatabaseId,
};

impl TransactionDB {
    pub async fn create<T>(
        conn: &mut T,
        sender: GroupId,
        receiver: GroupId,
        t_type_data: Option<i64>,
        description: Option<String>,
        money: i64,
    ) -> DatabaseResponse<DatabaseId>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        let now = Utc::now();
        query!(
            "
                insert into Transactions
                    (receiver, sender, is_undone, t_type_data, money, description, timestamp)
                values
                    (?, ?, ?, ?, ?, ?, ?)
                returning id
            ",
            receiver.0,
            sender.0,
            false,
            t_type_data,
            money,
            description,
            now
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(From::from)
        .map(|elem| elem.id)
    }
}
