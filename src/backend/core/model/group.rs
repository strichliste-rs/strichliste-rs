#![cfg(feature = "ssr")]
use crate::{backend::database::UserDB, model::GroupId};

#[derive(Debug)]
pub struct Group {
    #[allow(dead_code)]
    pub id: GroupId,
    pub members: Vec<UserDB>,
}
