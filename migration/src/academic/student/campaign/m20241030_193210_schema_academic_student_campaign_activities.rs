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
            CREATE TABLE IF NOT EXISTS academic_student_campaign.student_activities
            (
                id uuid DEFAULT uuid_generate_v7(),
                name character varying(255),
                cumulative_index real DEFAULT '0'::real,
                grand_cumulative_index real DEFAULT '0'::real,
                total_credit real DEFAULT '0'::real,
                grand_total_credit real DEFAULT '0'::real,
                student_id uuid NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                unit_activity_id uuid NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                status_id uuid NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                resign_status_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                unit_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                is_lock boolean DEFAULT false,
                finance_fee double precision DEFAULT 0,
                finance_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                feeder_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                created_at timestamp(0) without time zone DEFAULT now(),
                updated_at timestamp(0) without time zone DEFAULT now(),
                deleted_at timestamp(0) without time zone,
                sync_at timestamp(0) without time zone,
                created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                CONSTRAINT academic_student_campaign_activities_pkey PRIMARY KEY (id)
            )
            ",
        )
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP TABLE IF EXISTS academic_student_campaign.student_activities")
            .await?;

        Ok(())
    }
}
