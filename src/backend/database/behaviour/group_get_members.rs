use sqlx::{query_as, Executor};

use crate::{
    backend::database::{DatabaseResponse, DatabaseType, GroupDB, UserDB},
    model::DatabaseId,
};

impl GroupDB {
    pub async fn get_members<T>(conn: &mut T, gid: DatabaseId) -> DatabaseResponse<Vec<UserDB>>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        query_as!(
            UserDB,
            "
                select Users.*
                    from UserGroupMap
                join Users on Users.id = UserGroupMap.uid 
                    where UserGroupMap.gid = ?
            ",
            gid
        )
        .fetch_all(&mut *conn)
        .await
        .map_err(From::from)
    }
}
