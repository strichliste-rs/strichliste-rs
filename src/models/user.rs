#[cfg(feature = "ssr")]
use crate::{backend::core::User, model::Money};

#[cfg(feature = "ssr")]
use {
    crate::backend::database::{DatabaseResponse, DatabaseType},
    sqlx::Executor,
};

#[cfg(feature = "ssr")]
impl User {
    pub async fn add_money<T>(&mut self, conn: &mut T, money: Money) -> DatabaseResponse<()>
    where
        for<'a> &'a mut T: Executor<'a, Database = DatabaseType>,
    {
        let new_money = self.money.value + money.value;

        self.set_money(conn, new_money).await
    }
}
