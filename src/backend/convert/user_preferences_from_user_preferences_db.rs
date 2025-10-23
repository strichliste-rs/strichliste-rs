#![cfg(feature = "ssr")]

use crate::{backend::database::UserPreferencesDB, model::UserPreferences};
impl From<UserPreferencesDB> for UserPreferences {
    fn from(value: UserPreferencesDB) -> Self {
        let UserPreferencesDB {
            user_id: _,
            alternative_coloring,
        } = value;

        Self {
            alternative_coloring,
        }
    }
}
