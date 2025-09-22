#![cfg(feature = "ssr")]
use crate::{model::GroupId, models::UserDB};

#[derive(Debug)]
pub struct Group {
    #[allow(dead_code)]
    pub id: GroupId,
    pub members: Vec<UserDB>,
}
