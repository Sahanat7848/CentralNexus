use async_trait::async_trait;
use diesel::PgConnection;

#[async_trait]
#[async_trait]
pub trait TransactionProvider {
    async fn transaction<R, E>(
        &self,
        f: Box<dyn for<'a> FnOnce(&'a mut PgConnection) -> Result<R, E> + Send + 'static>,
    ) -> Result<R, E>
    where
        R: Send + 'static,
        E: From<diesel::result::Error> + Send + 'static;
}
