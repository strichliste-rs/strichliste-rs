use leptos::prelude::*;

#[cfg(feature = "ssr")]
use {
    crate::{
        backend::{
            core::Group,
            database::{DatabaseResponse, DatabaseType, GroupDB},
        },
        model::GroupId,
    },
    sqlx::Executor,
};

#[cfg(feature = "ssr")]
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
}

#[server]
pub async fn get_group_members(gid: i64) -> Result<Vec<String>, ServerFnError> {
    use crate::backend::core::ServerState;
    use tracing::error;
    let state: ServerState = expect_context();
    use axum::http::StatusCode;
    use leptos_axum::ResponseOptions;
    let response_opts: ResponseOptions = expect_context();

    let db = state.db.lock().await;

    let mut conn = match db.get_conn().await {
        Ok(val) => val,
        Err(e) => {
            error!("Failed to get db handle: {e}");
            response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            return Err(ServerFnError::new("Failed to get database!"));
        }
    };

    let group = match Group::get(&mut *conn, GroupId(gid)).await {
        Ok(val) => val,
        Err(e) => {
            error!("Failed to get group from db: {e}");
            response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            return Err(ServerFnError::new("Failed to get group"));
        }
    };

    Ok(group
        .members
        .into_iter()
        .map(|elem| elem.nickname)
        .collect())
}
