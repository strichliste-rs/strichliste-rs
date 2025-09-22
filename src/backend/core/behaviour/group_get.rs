#![cfg(feature = "ssr")]
use sqlx::Executor;

use crate::{
    backend::{
        core::Group,
        database::{DatabaseResponse, DatabaseType, GroupDB},
    },
    model::GroupId,
};

impl Group {
    pub async fn get<T>(conn: &mut T, gid: GroupId) -> DatabaseResponse<Self>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        let group_db = GroupDB::get(conn, gid).await?;

        let members = GroupDB::get_members(conn, group_db.id).await?;

        Ok(Self {
            id: group_db.id.into(),
            members,
        })
    }
}
