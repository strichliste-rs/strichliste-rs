use sqlx::{query, Executor};

use crate::{
    backend::database::{DatabaseResponse, DatabaseType, GroupDB},
    model::UserId,
};

impl GroupDB {
    pub async fn link_user<T>(&self, conn: &mut T, user_id: UserId) -> DatabaseResponse<()>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        query!(
            "
                insert into UserGroupMap
                (gid, uid)
                values
                (?, ?)
                ",
            self.id,
            user_id.0
        )
        .execute(&mut *conn)
        .await
        .map_err(From::from)
        .map(|_| ())
    }
}
