use core::fmt;
use std::str::FromStr;

use sqlx::{
    pool::PoolConnection,
    query,
    sqlite::{SqliteConnectOptions, SqlitePool},
    Sqlite, Transaction,
};
use tracing::info;

#[derive(Debug)]
pub struct DBError(String);

impl DBError {
    pub fn new(error: impl ToString) -> Self {
        DBError {
            0: error.to_string(),
        }
    }
}

impl fmt::Display for DBError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<sqlx::Error> for DBError {
    fn from(value: sqlx::Error) -> Self {
        DBError::new(value)
    }
}

pub type DatabaseId = i64;
pub type DatabaseResponse<T> = Result<T, DBError>;

pub type DatabaseType = Sqlite;

pub const DBUSER_KASSE_ID: i64 = 0;
pub const DBUSER_AUFLADUNG_ID: i64 = 1;

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

        _ = db.setup().await?;

        Ok(db)
    }

    pub async fn get_conn(&self) -> Result<PoolConnection<Sqlite>, DBError> {
        self.pool
            .acquire()
            .await
            .map_err(|e| DBError::new(e.to_string()))
    }

    pub async fn get_conn_transaction(&self) -> Result<Transaction<Sqlite>, DBError> {
        self.pool.begin().await.map_err(|e| DBError::new(e))
    }

    async fn setup(&self) -> Result<(), DBError> {
        let mut conn = self.get_conn().await.unwrap();

        _ = sqlx::migrate!("./migrations")
            .run(&mut *conn)
            .await
            .map_err(DBError::new)?;

        info!("Applied database migrations (if necessary)");

        _ = query!(
            "
                insert or ignore into Users
                    (id, nickname, money, is_system_user)
                values
                    (?, ?, ?, ?)
            ",
            DBUSER_KASSE_ID,
            "kasse",
            0,
            true,
        )
        .execute(&mut *conn)
        .await
        .map_err(DBError::new)?;

        _ = query!(
            "
                insert or ignore into Users
                    (id, nickname, money, is_system_user)
                values
                    (?, ?, ?, ?)
            ",
            DBUSER_AUFLADUNG_ID,
            "aufladung",
            0,
            true
        )
        .execute(&mut *conn)
        .await
        .map_err(DBError::new)?;

        Ok(())
    }
}
