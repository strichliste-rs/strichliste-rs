#![cfg(feature = "ssr")]

use sqlx::Executor;

use crate::{
    backend::database::{DBError, DatabaseResponse, DatabaseType, TransactionDB},
    model::{Page, PageRequestParams, UserId},
};

impl TransactionDB {
    pub async fn get_user_transactions<T>(
        conn: &mut T,
        user_id: UserId,
        page_request_params: PageRequestParams,
    ) -> DatabaseResponse<Page<Self>>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        let result = sqlx::query_as::<_, Self>(
            "
            select Transactions.* from Transactions
            join Users on Users.id=?
            join UserGroupMap as UGM on UGM.uid = Users.id
            where Transactions.receiver = UGM.gid or Transactions.sender = UGM.gid
            order by timestamp desc
            limit ?
            offset ?
        ",
        )
        .bind(user_id.0)
        .bind(page_request_params.limit as i64)
        .bind(page_request_params.offset as i64);

        let result = result
            .fetch_all(&mut *conn)
            .await
            .map_err(Into::<DBError>::into)?;

        let count = sqlx::query_as::<_, (u64,)>(
            r#"
                select count(*) from Transactions
            join Users on Users.id=?
            join UserGroupMap as UGM on UGM.uid = Users.id
            where Transactions.receiver = UGM.gid or Transactions.sender = UGM.gid
            "#,
        )
        .bind(user_id.0)
        .fetch_one(&mut *conn)
        .await
        .map_err(Into::<DBError>::into)?;

        Ok(Page::new(page_request_params, count.0 as usize, result))
    }
}
