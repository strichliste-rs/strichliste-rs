use sqlx::Executor;

use crate::backend::{
    core::Group,
    database::{DatabaseResponse, DatabaseType, GroupDB},
};

use super::{GroupId, UserId};

impl Group {
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
