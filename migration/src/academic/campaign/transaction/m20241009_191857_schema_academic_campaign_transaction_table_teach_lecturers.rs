use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::ConnectionTrait;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let db = manager.get_connection();
        db.execute_unprepared(
            "
            CREATE SCHEMA IF NOT EXISTS academic_campaign_transaction;
            ",
        )
        .await?;
        db.execute_unprepared(
            "
            CREATE TABLE IF NOT EXISTS academic_campaign_transaction.teach_lecturers
            (
                id uuid DEFAULT uuid_generate_v7(),
                name character varying(255),
                planning integer NOT NULL DEFAULT 0,
                realization integer NOT NULL DEFAULT 0,
                credit numeric(3,1) DEFAULT 0,
                is_lecturer_home_base boolean NOT NULL DEFAULT false,
                lecturer_id uuid NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                teach_id uuid NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                evaluation_type_id uuid NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                created_at timestamp(0) without time zone DEFAULT now(),
                updated_at timestamp(0) without time zone DEFAULT now(),
                deleted_at timestamp(0) without time zone,
                sync_at timestamp(0) without time zone,
                created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                CONSTRAINT act_teach_lecturers_pkey PRIMARY KEY (id)
            )
            ",
        )
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                "DROP TABLE IF EXISTS academic_campaign_transaction.teach_lecturers",
            )
            .await?;

        Ok(())
    }
}
