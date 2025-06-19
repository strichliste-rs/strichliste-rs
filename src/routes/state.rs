use reactive_stores::Store;

use crate::models::User;

#[derive(Clone, Debug, Default, Store)]
pub struct FrontendStore {
    // #[store(key: i64 = |user| user.id.unwrap())]
    pub cached_users: Vec<User>,
}
