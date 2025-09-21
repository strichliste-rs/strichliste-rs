use serde::{Deserialize, Serialize};

use crate::models::Money;

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum AudioPlayback {
    Failed,
    Undo,
    Deposit(Money),
    Sent(Money),
    Withdraw(Money),
    Bought(i64),
}
