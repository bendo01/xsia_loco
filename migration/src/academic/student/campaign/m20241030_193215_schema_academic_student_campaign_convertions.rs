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
            CREATE SCHEMA IF NOT EXISTS academic_student_campaign;
            ",
        )
        .await?;
        db.execute_unprepared(
            "
            CREATE TABLE IF NOT EXISTS academic_student_campaign.convertions
            (
                id uuid DEFAULT uuid_generate_v7(),
                student_id uuid NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                course_id uuid NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                grade_id uuid NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                transfer_code character varying(255) NOT NULL,
                transfer_name character varying(255) NOT NULL,
                transfer_credit real DEFAULT '0'::real,
                transfer_grade character varying(255) NOT NULL,
                is_lock timestamp(0) without time zone,
                feeder_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                created_at timestamp(0) without time zone DEFAULT now(),
                updated_at timestamp(0) without time zone DEFAULT now(),
                deleted_at timestamp(0) without time zone,
                sync_at timestamp(0) without time zone,
                created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                CONSTRAINT academic_student_campaign_convertions_pkey PRIMARY KEY (id)
            )
            ",
        )
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP TABLE IF EXISTS academic_student_campaign.convertions")
            .await?;

        Ok(())
    }
}
