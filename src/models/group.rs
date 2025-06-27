use sqlx::{query, Executor};

use crate::{
    backend::db::{DBError, DatabaseResponse, DatabaseType},
    models::DatabaseId,
};

use super::GroupId;

pub(crate) struct Group {
    id: GroupId,
    members: Vec<String>,
}

pub(crate) struct GroupDB {
    id: DatabaseId,
}

impl From<DatabaseId> for GroupDB {
    fn from(id: DatabaseId) -> Self {
        GroupDB { id }
    }
}

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
                returning id
            ",
            id
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(From::from)
        .map(|e| e.id)
        .map(From::from)
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

    pub async fn link_user<T>(&self, conn: &mut T, user_id: DatabaseId) -> DatabaseResponse<()>
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
            user_id
        )
        .execute(&mut *conn)
        .await
        .map_err(From::from)
        .map(|_| ())
    }
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
    pub async fn get_groups<T>(conn: &mut T, )
}
impl Group {
    pub async fn get<T>(conn: &mut T, gid: GroupId) -> DatabaseResponse<Self>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        let members = query!(
            "
                select Users.nickname
                    from UserGroupMap
                join Users on Users.id = UserGroupMap.uid 
                    where UserGroupMap.gid = ?
            ",
            gid.0
        )
        .fetch_all(&mut *conn)
        .await
        .map_err(From::<DBError>::from)?
        .into_iter()
        .map(|elem| elem.nickname)
        .collect();

        Ok(Self { id: gid, members })
    }
}
