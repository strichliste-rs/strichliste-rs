use serde::{Deserialize, Serialize};

use crate::models::Money;

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub enum AudioPlayback {
    Failed,
    Undo,
    Nothing,
    Deposit(Money),
    Sent(Money),
    Withdraw(Money),
    Bought(i64),
}
