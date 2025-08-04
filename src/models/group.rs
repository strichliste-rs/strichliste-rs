use sqlx::{error::DatabaseError, query, query_as, Executor};

use crate::{
    backend::db::{DBError, DatabaseResponse, DatabaseType},
    models::DatabaseId,
};

use super::{GroupId, UserId};

pub struct Group {
    id: GroupId,
    members: Vec<String>,
}

impl Group {
    pub async fn get<T>(conn: &mut T, gid: GroupId) -> DatabaseResponse<Self>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        let group_db = GroupDB::get(conn, gid).await?;

        let members = GroupDB::get_members(conn, group_db.id).await?;

        Ok(Self {
            id: group_db.id.into(),
            members,
        })
    }

    pub async fn get_user_group<T>(conn: &mut T, uid: UserId) -> DatabaseResponse<GroupId>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        Ok(GroupDB::get_single_group(conn, uid).await?.into())
    }
}

pub struct GroupDB {
    pub id: DatabaseId,
}

impl From<DatabaseId> for GroupDB {
    fn from(id: DatabaseId) -> Self {
        GroupDB { id }
    }
}

impl GroupDB {
    pub async fn get_members<T>(conn: &mut T, gid: DatabaseId) -> DatabaseResponse<Vec<String>>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        query!(
            "
                select Users.nickname
                    from UserGroupMap
                join Users on Users.id = UserGroupMap.uid 
                    where UserGroupMap.gid = ?
            ",
            gid
        )
        .fetch_all(&mut *conn)
        .await
        .map_err(From::from)
        .map(|vec| {
            vec.into_iter()
                .map(|record| record.nickname)
                .collect::<Vec<String>>()
        })
    }
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
            None => Err(DBError::new(&format!("Failed to find group: {}", gid.0))),
            Some(value) => Ok(value),
        }
    }
}
