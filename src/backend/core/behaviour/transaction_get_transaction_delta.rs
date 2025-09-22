#![cfg(feature = "ssr")]

use std::collections::HashMap;

use sqlx::Executor;

use crate::{
    backend::{
        core::{Group, User},
        database::{DBError, DatabaseType, TransactionDB},
    },
    model::UserId,
    models::{Transaction, TransactionDelta},
};

impl Transaction {
    pub async fn get_transaction_delta<T>(
        conn: &mut T,
        sender_group: &Group,
        receiver_group: &Group,
        transaction_db: &TransactionDB,
    ) -> Result<HashMap<User, TransactionDelta>, DBError>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        use tracing::error;

        let mut senders = Vec::<User>::new();
        let mut receivers = Vec::<User>::new();

        for sender in sender_group.members.iter() {
            let user_send = match User::get(&mut *conn, UserId(sender.id)).await? {
                Some(val) => val,
                None => {
                    error!("Failed to find a user that should exist! Id: {}", sender.id);
                    return Err(DBError::new("Failed to find user"));
                }
            };

            senders.push(user_send);
        }

        for receiver in receiver_group.members.iter() {
            let user_recv = match User::get(&mut *conn, UserId(receiver.id)).await? {
                Some(val) => val,
                None => {
                    error!(
                        "Failed to find a user that should exist! Id: {}",
                        receiver.id
                    );
                    return Err(DBError::new("Failed to find user"));
                }
            };

            receivers.push(user_recv);
        }

        let mut delta_map = HashMap::new();

        let mut full_cost = transaction_db.money;
        let cost_share = transaction_db.money / sender_group.members.len() as u64;

        for user in senders.iter().chain(receivers.iter()) {
            _ = delta_map.insert(
                user.clone(),
                TransactionDelta {
                    amount_pre: user.money.value,
                    delta: 0,
                },
            );
        }

        for sender in senders.iter() {
            let user = match delta_map.get_mut(sender) {
                Some(user) => user,
                None => {
                    error!("Failed to find user in HashMap where it should exist!");
                    return Err(DBError::new("Failed to find user"));
                }
            };

            user.delta -= cost_share as i64;
            full_cost -= cost_share;
        }

        while full_cost > 0 {
            for sender in senders.iter_mut() {
                let user = match delta_map.get_mut(sender) {
                    Some(user) => user,
                    None => {
                        error!("Failed to find user in HashMap where it should exist!");
                        return Err(DBError::new("Failed to find user"));
                    }
                };
                user.delta -= 1;
                if full_cost == 0 {
                    break;
                }

                full_cost -= 1;
            }
        }

        let cost_share = transaction_db.money / receiver_group.members.len() as u64;

        for receiver in receivers.iter_mut() {
            let user = match delta_map.get_mut(receiver) {
                Some(user) => user,
                None => {
                    error!("Failed to find user in HashMap where it should exist!");
                    return Err(DBError::new("Failed to find user"));
                }
            };
            user.delta += cost_share as i64;
            full_cost += cost_share;
        }

        while full_cost < transaction_db.money {
            for receiver in receivers.iter_mut() {
                let user = match delta_map.get_mut(receiver) {
                    Some(user) => user,
                    None => {
                        error!("Failed to find user in HashMap where it should exist!");
                        return Err(DBError::new("Failed to find user"));
                    }
                };
                user.delta += 1;
                if full_cost == transaction_db.money {
                    break;
                }

                full_cost += 1;
            }
        }

        Ok(delta_map)
    }
}
