use sqlx::Executor;

use crate::{
    backend::database::{DatabaseResponse, DatabaseType, GroupDB},
    models::{DatabaseId, UserDB},
};

use super::{GroupId, UserId};

#[derive(Debug)]
pub struct Group {
    #[allow(dead_code)]
    pub id: GroupId,
    pub members: Vec<UserDB>,
}

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

impl From<DatabaseId> for GroupDB {
    fn from(id: DatabaseId) -> Self {
        GroupDB { id }
    }
}

impl GroupDB {
    pub async fn create_group_for_multiple_users_id<T>(
        conn: &mut T,
        user_ids: &[UserId],
    ) -> DatabaseResponse<Self>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        let new_group = GroupDB::create(&mut *conn).await?;

        for user in user_ids.iter() {
            new_group.link_user(&mut *conn, *user).await?;
        }

        Ok(new_group)
    }
}

impl From<GroupDB> for GroupId {
    fn from(val: GroupDB) -> Self {
        GroupId(val.id)
    }
}

impl From<&GroupDB> for GroupId {
    fn from(value: &GroupDB) -> Self {
        Self(value.id)
    }
}
