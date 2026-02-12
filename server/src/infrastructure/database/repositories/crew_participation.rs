use anyhow::Result;
use async_trait::async_trait;
use diesel::PgConnection;
use diesel::RunQueryDsl;
use diesel::delete;
use diesel::prelude::*;
use std::sync::Arc;

use crate::domain::entities::crew_memberships::CrewMemberShips;
use crate::domain::repositories::crew_oparation::CrewOperationRepository;
use crate::infrastructure::database::postgresql_connection::PgPoolSquad;
use crate::infrastructure::database::schema::crew_memberships;

pub struct CrewParticipationPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl CrewParticipationPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl CrewOperationRepository for CrewParticipationPostgres {
    async fn join(&self, crew_memberships: CrewMemberShips) -> Result<()> {
        let mut connection = self
            .db_pool
            .get()
            .map_err(|e| anyhow::Error::msg(e.to_string()))?;

        diesel::insert_into(crew_memberships::table)
            .values(&crew_memberships)
            .execute(&mut connection)?;

        Ok(())
    }

    async fn leave(&self, crew_memberships: CrewMemberShips) -> Result<()> {
        let mut connection = self
            .db_pool
            .get()
            .map_err(|e| anyhow::Error::msg(e.to_string()))?;

        diesel::delete(crew_memberships::table)
            .filter(crew_memberships::brawler_id.eq(crew_memberships.brawler_id))
            .filter(crew_memberships::mission_id.eq(crew_memberships.mission_id))
            .execute(&mut connection)?;

        Ok(())
    }

    fn for_insert_transaction_test(
        &self,
        conn: &mut PgConnection,
        crew_memberships: CrewMemberShips,
    ) -> Result<()> {
        diesel::insert_into(crew_memberships::table)
            .values(&crew_memberships)
            .execute(conn)?;

        Ok(())
    }

    fn for_delete_transaction_test(
        &self,
        conn: &mut PgConnection,
        crew_memberships: CrewMemberShips,
    ) -> Result<()> {
        delete(crew_memberships::table)
            .filter(crew_memberships::brawler_id.eq(crew_memberships.brawler_id))
            .filter(crew_memberships::mission_id.eq(crew_memberships.mission_id))
            .execute(conn)?;

        Ok(())
    }
}

#[async_trait]
impl crate::domain::repositories::transaction_provider::TransactionProvider
    for CrewParticipationPostgres
{
    async fn transaction<R, E>(
        &self,
        f: Box<dyn for<'a> FnOnce(&'a mut PgConnection) -> Result<R, E> + Send + 'static>,
    ) -> Result<R, E>
    where
        R: Send + 'static,
        E: From<diesel::result::Error> + Send + 'static,
    {
        self.db_pool.transaction(f).await
    }
}
