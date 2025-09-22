#![cfg(feature = "ssr")]
use sqlx::Executor;

use crate::{
    backend::{
        core::Group,
        database::{DatabaseResponse, DatabaseType, GroupDB},
    },
    model::UserId,
    models::GroupId,
};

impl Group {
    #[allow(dead_code)]
    pub async fn get_groups<T>(conn: &mut T, uid: UserId) -> DatabaseResponse<Vec<Self>>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        let group_ids = GroupDB::get_groups(conn, uid).await?;

        let mut groups: Vec<Self> = Vec::new();

        for group_id in group_ids {
            groups.push(Group::get(conn, GroupId(group_id.id)).await?);
        }

        Ok(groups)
    }
}
