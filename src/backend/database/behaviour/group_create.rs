use sqlx::{query, Executor};

use crate::{backend::database::{DBError, DatabaseResponse, DatabaseType, GroupDB}, model::DatabaseId};

impl GroupDB {
    pub async fn _create<T>(conn: &mut T, id: DatabaseId) -> DatabaseResponse<Self>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        query!(
            "
                insert or ignore into Groups
                (id)
                values
                (?)
            ",
            id
        )
        .execute(&mut *conn)
        .await
        .map_err(DBError::new)?;

        Ok(GroupDB { id })
    }
    pub async fn create<T>(conn: &mut T) -> DatabaseResponse<Self>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        query!(
            "
              insert into Groups
              default values
              returning id  
            "
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(From::from)
        .map(|e| e.id)
        .map(From::from)
    }
}
