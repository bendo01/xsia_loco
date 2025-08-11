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
            CREATE SCHEMA IF NOT EXISTS feeder_consistency;
            ",
        )
        .await?;
        db.execute_unprepared(
            "
            CREATE TABLE IF NOT EXISTS feeder_consistency.comparisons
            (
                id uuid DEFAULT uuid_generate_v7(),
                code character varying(255) NOT NULL,
                name character varying(255) NOT NULL,
                kind character varying(255) NOT NULL,
                feeder_table character varying(255) NOT NULL,
                app_table character varying(255) NOT NULL,
                reference_key_feeder_table character varying(255) NOT NULL,
                reference_key_app_table character varying(255) NOT NULL,
                total_feeder_data integer NOT NULL DEFAULT 0,
                total_app_data integer NOT NULL DEFAULT 0,
                difference integer NOT NULL DEFAULT 0,
                created_at timestamp(0) without time zone DEFAULT now(),
                updated_at timestamp(0) without time zone DEFAULT now(),
                deleted_at timestamp(0) without time zone,
                sync_at timestamp(0) without time zone,
                created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                CONSTRAINT feeder_consistency_comparisons_pkey PRIMARY KEY (id)
            )
            ",
        )
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP TABLE IF EXISTS feeder_consistency.comparisons")
            .await?;

        Ok(())
    }
}
