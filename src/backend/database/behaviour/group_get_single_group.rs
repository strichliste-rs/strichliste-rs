use sqlx::{query, Executor};

use crate::{
    backend::database::{DBError, DatabaseResponse, DatabaseType, GroupDB},
    models::{DatabaseId, UserId},
};

impl GroupDB {
    /// get primitive group id
    pub async fn get_single_group<T>(conn: &mut T, user_id: UserId) -> DatabaseResponse<DatabaseId>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        let opt = query!(
            "
                select gid
                from UserGroupMap
                group by gid
                having count(uid) = 1 AND MAX(uid) = ?;
            ",
            user_id.0
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(From::from)
        .map(|elem| elem.gid);
        match opt {
            Ok(Some(value)) => Ok(value),
            Ok(None) => Err(DBError::new(format!(
                "FATAL user with id {} did not have group",
                user_id
            ))),
            Err(e) => Err(e),
        }
    }
}
