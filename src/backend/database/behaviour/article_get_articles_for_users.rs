#![cfg(feature = "ssr")]

use sqlx::{query, Executor};

use crate::{
    backend::database::{
        ArticleDB, DBError, DatabaseResponse, DatabaseType, GroupDB, DBGROUP_SNACKBAR_ID,
    },
    model::UserId,
};

impl ArticleDB {
    pub async fn get_articles_for_user<T>(
        conn: &mut T,
        user_id: UserId,
    ) -> DatabaseResponse<Vec<(i64, i64)>>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        let user_group = GroupDB::get_single_group(&mut *conn, user_id).await?;
        let result = query!(
            "
                select
                    t_type_data as article_id, count(id) as amount
                from
                    Transactions
                where
                    sender = ? and is_undone = 0 and receiver = ?
                group by t_type_data
                order by timestamp desc
                limit 50
            ",
            user_group,
            DBGROUP_SNACKBAR_ID.0
        )
        .fetch_all(&mut *conn)
        .await
        .map_err(DBError::new)
        .map(|elem| {
            elem.into_iter()
                .map(|value| (value.article_id.unwrap(), value.amount))
                .collect::<Vec<(i64, i64)>>()
        })?;

        Ok(result)
    }
}
