use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct UserPreferences {
    pub alternative_coloring: bool,
}
