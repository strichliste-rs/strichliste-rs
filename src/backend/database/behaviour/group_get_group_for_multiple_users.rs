use itertools::Itertools;
use sqlx::{query, Executor};

use crate::{
    backend::database::{DatabaseResponse, DatabaseType, GroupDB}, model::UserId,
};

impl GroupDB {
    pub async fn get_group_for_multiple_users_id<T>(
        conn: &mut T,
        user_ids: &[UserId],
    ) -> DatabaseResponse<Option<Self>>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        let count = user_ids.len() as i64;
        let user_ids_string: String =
            Itertools::intersperse(user_ids.iter().map(ToString::to_string), String::from(", "))
                .collect();

        let group_id = query!(
            "
                select gid
                from UserGroupMap
                group by gid
                having
                    count(distinct uid) = ?
                    and sum(case when uid not in (?) then 1 else 0 end) = 0
                    and count(distinct case when uid in (?) then uid end) = ?
            ",
            count,
            user_ids_string,
            user_ids_string,
            count
        )
        .fetch_optional(&mut *conn)
        .await?;

        Ok(group_id.map(|val| GroupDB { id: val.gid }))
    }
}
