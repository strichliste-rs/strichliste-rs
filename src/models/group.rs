use sqlx::Executor;

use crate::backend::{
    core::Group,
    database::{DatabaseResponse, DatabaseType, GroupDB},
};

use super::{GroupId, UserId};

impl Group {
    pub async fn get_user_group_id<T>(conn: &mut T, uid: UserId) -> DatabaseResponse<GroupId>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        Ok(GroupDB::get_single_group(conn, uid).await?.into())
    }

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

    pub async fn get_group_id_for_multiple_users<T>(
        conn: &mut T,
        uids: &[UserId],
    ) -> DatabaseResponse<GroupId>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        Ok(
            match GroupDB::get_group_for_multiple_users_id(&mut *conn, uids).await? {
                Some(val) => val.id,
                None => {
                    GroupDB::create_group_for_multiple_users_id(&mut *conn, uids)
                        .await?
                        .id
                }
            }
            .into(),
        )
    }
}
