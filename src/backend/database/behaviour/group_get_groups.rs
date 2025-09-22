use sqlx::{query, Executor};

use crate::{
    backend::database::{DatabaseResponse, DatabaseType, GroupDB}, model::UserId,
};

impl GroupDB {
    pub async fn get_groups<T>(conn: &mut T, user_id: UserId) -> DatabaseResponse<Vec<Self>>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        query!(
            "
                select Groups.id as id
                from Groups
                join UserGroupMap on Groups.id = UserGroupMap.gid
                where UserGroupMap.uid = ?
            ",
            user_id.0
        )
        .fetch_all(&mut *conn)
        .await
        .map_err(From::from)
        .map(|elem| elem.into_iter().map(|elem| Self { id: elem.id }).collect())
    }
}
