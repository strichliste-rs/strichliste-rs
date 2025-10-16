use std::str::FromStr;

use sqlx::{pool::PoolConnection, sqlite::SqliteConnectOptions, Sqlite, SqlitePool, Transaction};
use tracing::info;

use crate::backend::database::DBError;

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

        db.setup().await?;

        Ok(db)
    }

    pub async fn close(self) {
        self.pool.close().await;
    }

    pub async fn get_conn(&self) -> Result<PoolConnection<Sqlite>, DBError> {
        self.pool
            .acquire()
            .await
            .map_err(|e| DBError::new(e.to_string()))
    }

    pub async fn get_conn_transaction(&'_ self) -> Result<Transaction<'_, Sqlite>, DBError> {
        self.pool.begin().await.map_err(DBError::new)
    }

    async fn setup(&self) -> Result<(), DBError> {
        let mut transaction = self.get_conn_transaction().await.unwrap();

        sqlx::migrate!("./migrations")
            .run(&mut *transaction)
            .await
            .map_err(DBError::new)?;

        info!("Applied database migrations (if necessary)");

        transaction.commit().await.map_err(From::from)
    }
}
