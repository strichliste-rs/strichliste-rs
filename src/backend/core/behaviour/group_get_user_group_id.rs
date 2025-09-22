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
    pub async fn get_user_group_id<T>(conn: &mut T, uid: UserId) -> DatabaseResponse<GroupId>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        Ok(GroupDB::get_single_group(conn, uid).await?.into())
    }
}
