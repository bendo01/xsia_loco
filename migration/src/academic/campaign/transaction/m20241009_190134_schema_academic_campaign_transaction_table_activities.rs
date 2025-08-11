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
            CREATE TABLE IF NOT EXISTS academic_campaign_transaction.activities
            (
                id uuid NOT NULL DEFAULT gen_random_uuid(),
                name character varying(255) NOT NULL,
                week_quantity integer DEFAULT 0,
                student_target integer NOT NULL DEFAULT 0,
                candidate_number integer NOT NULL DEFAULT 0,
                candidate_pass integer NOT NULL DEFAULT 0,
                became_student integer NOT NULL DEFAULT 0,
                transfer_student integer NOT NULL DEFAULT 0,
                total_class_member integer DEFAULT 0,
                start_date date,
                end_date date,
                start_transaction date,
                end_transaction date,
                unit_id uuid NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                academic_year_id uuid NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                is_active boolean DEFAULT false,
                feeder_id uuid,
                created_at timestamp(0) without time zone DEFAULT now(),
                updated_at timestamp(0) without time zone DEFAULT now(),
                deleted_at timestamp(0) without time zone,
                sync_at timestamp(0) without time zone,
                created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                CONSTRAINT act_activities_pkey PRIMARY KEY (id)
            )
            ",
        )
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP TABLE IF EXISTS academic_campaign_transaction.activities")
            .await?;

        Ok(())
    }
}
