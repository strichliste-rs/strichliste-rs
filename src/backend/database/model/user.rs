use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, sqlx::Type, sqlx::FromRow)]
pub struct UserDB {
    pub id: i64,
    pub nickname: String,
    pub money: i64,
    pub is_system_user: bool,
}
