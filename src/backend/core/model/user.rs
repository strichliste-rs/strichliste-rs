use serde::{Deserialize, Serialize};

use crate::model::{Money, UserId};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct User {
    pub id: UserId,
    pub nickname: String,
    pub card_number: Option<String>,
    pub money: Money,
}
