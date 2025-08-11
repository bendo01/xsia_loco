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
            CREATE SCHEMA IF NOT EXISTS institution_master
            ",
        )
        .await?;
        db.execute_unprepared(
            "
            CREATE TABLE IF NOT EXISTS institution_master.institutions
            (
                id uuid NOT NULL DEFAULT gen_random_uuid(),
                code character varying(255),
                name character varying(255),
                alphabet_code character varying(255),
                is_active boolean NOT NULL DEFAULT false,
                variety_id uuid NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                category_id uuid NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                country_id uuid NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                parent_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                feeder_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                academic_year_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                created_at timestamp(0) without time zone DEFAULT now(),
                updated_at timestamp(0) without time zone DEFAULT now(),
                deleted_at timestamp(0) without time zone,
                sync_at timestamp(0) without time zone,
                created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                CONSTRAINT im_institutions_pkey PRIMARY KEY (id)
            )
            ",
        )
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP TABLE IF EXISTS institution_master.institutions")
            .await?;

        Ok(())
    }
}
