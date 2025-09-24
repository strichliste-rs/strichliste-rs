use leptos::prelude::*;

use crate::{
    model::{Money, Transaction, TransactionType, UserId},
    routes::user::CreateTransactionError,
};

#[cfg(feature = "ssr")]
use {
    crate::{
        backend::{
            core::{Group, Settings},
            database::{DatabaseType, TransactionDB},
        },
        model::{DatabaseId, GroupId},
    },
    sqlx::Executor,
};

#[cfg(feature = "ssr")]
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

#[server]
pub async fn create_transaction(
    user_id: UserId,
    money: Money,
    transaction_type: TransactionType,
) -> Result<Transaction, CreateTransactionError> {
    type Error = CreateTransactionError;
    use crate::backend::{
        core::ServerState,
        database::{DBGROUP_AUFLADUNG_ID, DBGROUP_SNACKBAR_ID},
    };
    use axum::http::StatusCode;
    use leptos_axum::ResponseOptions;
    use tracing::error;

    let state: ServerState = expect_context();

    let response_opts: ResponseOptions = expect_context();

    if money.value < 0 {
        response_opts.set_status(StatusCode::BAD_REQUEST);
        return Err(Error::new("Money may not be negative"));
    }

    let db = state.db.lock().await;
    let mut db_trans = match db.get_conn_transaction().await {
        Ok(value) => value,
        Err(e) => {
            response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            error!("Failed to get database handle: {}", e);
            return Err(Error::new("Failed to get database handle!"));
        }
    };

    let user_group = match Group::get_user_group_id(&mut *db_trans, user_id).await {
        Ok(value) => value,
        Err(e) => {
            response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            error!("Failed to get user group: {}", e);
            return Err(Error::new("Failed to get user group"));
        }
    };

    let (sender_group_id, receiver_group_id) = match transaction_type {
        TransactionType::Deposit => (DBGROUP_AUFLADUNG_ID, user_group),
        TransactionType::Withdraw => (user_group, DBGROUP_AUFLADUNG_ID),
        TransactionType::Bought(_) => (user_group, DBGROUP_SNACKBAR_ID),

        _ => return Err(Error::new("Invalid state")),
    };

    let transaction_id = Transaction::create(
        &mut *db_trans,
        sender_group_id,
        receiver_group_id,
        transaction_type,
        None,
        money,
        &state.settings,
    )
    .await?;

    let transaction = match Transaction::get(&mut *db_trans, transaction_id, user_id).await {
        Ok(val) => val,
        Err(e) => {
            response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            error!("Failed to find transaction during DB-lookup: {}", e);
            return Err(Error::new("Failed to find transaction!"));
        }
    };

    let transaction = match transaction {
        Some(val) => val,
        None => {
            response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            error!("Failed to find transaction");
            return Err(Error::new("Failed to find transaction!"));
        }
    };

    match db_trans.commit().await {
        Ok(_) => {}
        Err(e) => {
            response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            error!("Failed to commit transaction: {}", e);
            return Err(Error::new("Failed to commit transaction!"));
        }
    }

    Ok(transaction)
}
