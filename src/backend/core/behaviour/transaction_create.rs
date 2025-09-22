#![cfg(feature = "ssr")]

use sqlx::Executor;

use crate::{
    backend::{
        core::{Group, Settings},
        database::{DatabaseType, TransactionDB},
    },
    model::{GroupId, Money, Transaction, TransactionType},
    models::DatabaseId,
    routes::user::CreateTransactionError,
};

impl Transaction {
    pub async fn create<T>(
        conn: &mut T,
        sender: GroupId,
        receiver: GroupId,
        t_type: TransactionType,
        description: Option<String>,
        money: Money,
        settings: &Settings,
    ) -> Result<DatabaseId, CreateTransactionError>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        type Error = CreateTransactionError;

        let t_type_data = match t_type {
            TransactionType::Bought(id) => Some(id),
            TransactionType::Received(id) => Some(id.0),

            _ => None,
        };

        let t_id = TransactionDB::create(
            &mut *conn,
            sender,
            receiver,
            t_type_data,
            description,
            money.value,
        )
        .await?;

        let transaction_db = match TransactionDB::get(&mut *conn, t_id).await? {
            Some(val) => val,
            None => return Err(Error::new("Failed to find newly created transaction")),
        };

        let (sender_group, receiver_group) = (
            Group::get(&mut *conn, GroupId(transaction_db.sender)).await?,
            Group::get(&mut *conn, GroupId(transaction_db.receiver)).await?,
        );

        let deltas = Transaction::get_transaction_delta(
            &mut *conn,
            &sender_group,
            &receiver_group,
            &transaction_db,
        )
        .await?;

        let mut users_too_low = Vec::<String>::new();
        let mut users_too_high = Vec::<String>::new();

        for (key, value) in deltas.iter() {
            use crate::backend::database::{DBUSER_AUFLADUNG_ID, DBUSER_SNACKBAR_ID};

            if key.id.0 == DBUSER_AUFLADUNG_ID.0 || key.id.0 == DBUSER_SNACKBAR_ID.0 {
                // don't do tracking on the system users
                continue;
            }

            if value.post_amount() > settings.accounts.upper_limit {
                if value.delta < 0 {
                    // allow users to loose money
                    continue;
                }

                users_too_high.push(key.nickname.clone());
            } else if value.post_amount() < settings.accounts.lower_limit {
                if value.delta > 0 {
                    // allow users to get money
                    continue;
                }

                users_too_low.push(key.nickname.clone());
            }
        }

        if !users_too_low.is_empty() {
            return Err(CreateTransactionError::TooLittleMoneyError(users_too_low));
        }

        if !users_too_high.is_empty() {
            return Err(CreateTransactionError::TooMuchMoneyError(users_too_high));
        }

        for (mut key, value) in deltas.into_iter() {
            key.add_money(&mut *conn, Money { value: value.delta })
                .await?;
        }

        Ok(transaction_db.id)
    }
}
