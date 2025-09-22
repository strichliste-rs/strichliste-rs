use sqlx::Executor;

use crate::{
    backend::database::{DatabaseResponse, DatabaseType, GroupDB}, model::UserId,
};

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
