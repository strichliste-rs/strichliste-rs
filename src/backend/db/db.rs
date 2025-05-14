use core::fmt;
use std::str::FromStr;

use sqlx::{
    pool::PoolConnection,
    query, query_as,
    sqlite::{SqliteConnectOptions, SqlitePool},
    Sqlite,
};
use tracing::info;

// use crate::models::{Jobset, JobsetID, JobsetState, Project};

fn convert_to_string<T: ToString>(some_option: Option<T>) -> String {
    if some_option.is_some() {
        return some_option.unwrap().to_string();
    } else {
        return "null".to_string();
    }
}
#[derive(Debug)]
pub struct DBError(String);

impl DBError {
    pub fn new(error: String) -> Self {
        DBError { 0: error }
    }
}

impl fmt::Display for DBError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct DB {
    pool: SqlitePool,
}

impl DB {
    pub async fn new(path: &str) -> Result<Self, DBError> {
        let path = String::new() + "sqlite://" + path;
        let opts = SqliteConnectOptions::from_str(&path)
            .map_err(|e| DBError::new(e.to_string()))?
            .create_if_missing(true)
            .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal);

        info!("Connecting to database: {}", path);
        let pool = SqlitePool::connect_with(opts).await;

        let db = pool.map_err(|e| DBError::new(e.to_string()))?;

        let db = DB { pool: db };

        let setup = db.setup().await;
        if setup.is_some() {
            return Err(setup.unwrap());
        };

        Ok(db)
    }

    pub async fn get_conn(&self) -> Result<PoolConnection<Sqlite>, DBError> {
        self.pool
            .acquire()
            .await
            .map_err(|e| DBError::new(e.to_string()))
    }

    async fn setup(&self) -> Option<DBError> {
        let mut conn = self.get_conn().await.unwrap();

        let result = sqlx::migrate!("./migrations")
            .run(&mut *conn)
            .await
            .map_err(|e| DBError::new(e.to_string()));

        if result.is_err() {
            return Some(result.err().unwrap());
        }

        info!("Applied database migrations (if necessary)");

        None
    }
}
