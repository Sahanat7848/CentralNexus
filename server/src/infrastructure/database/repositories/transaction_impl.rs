use crate::domain::repositories::transaction_provider::TransactionProvider;
use crate::infrastructure::database::postgresql_connection::PgPoolSquad;
use async_trait::async_trait;
use diesel::Connection;
use diesel::PgConnection;
use tokio::task;

#[async_trait]
impl TransactionProvider for PgPoolSquad {
    async fn transaction<R, E>(
        &self,
        f: Box<dyn for<'a> FnOnce(&'a mut PgConnection) -> Result<R, E> + Send + 'static>,
    ) -> Result<R, E>
    where
        R: Send + 'static,
        E: From<diesel::result::Error> + Send + 'static,
    {
        let pool = self.clone();
        task::spawn_blocking(move || {
            let mut conn = pool.get().expect("Failed to get connection from pool");
            conn.transaction(|conn| f(conn))
        })
        .await
        .expect("Blocking task failed")
    }
}
