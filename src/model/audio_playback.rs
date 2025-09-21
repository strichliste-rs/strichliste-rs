use serde::{Deserialize, Serialize};

use crate::model::Money;


#[derive(Serialize, Deserialize, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum AudioPlayback {
    Failed,
    Undo,
    Deposit(Money),
    Sent(Money),
    Withdraw(Money),
    Bought(i64),
}
