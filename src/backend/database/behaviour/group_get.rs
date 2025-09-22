use sqlx::{query_as, Executor};

use crate::{
    backend::database::{DBError, DatabaseResponse, DatabaseType, GroupDB},
    model::GroupId,
};

impl GroupDB {
    pub async fn get<T>(conn: &mut T, gid: GroupId) -> DatabaseResponse<Self>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        let group = query_as!(
            Self,
            "
                select *
                from Groups
                where id = ?
            ",
            gid.0
        )
        .fetch_optional(&mut *conn)
        .await
        .map_err(DBError::new)?;

        match group {
            None => Err(DBError::new(format!("Failed to find group: {}", gid.0))),
            Some(value) => Ok(value),
        }
    }
}
