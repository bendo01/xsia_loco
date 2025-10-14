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
            CREATE TABLE IF NOT EXISTS academic_campaign_transaction.teaches
            (
                id uuid DEFAULT uuid_generate_v7(),
                name text,
                class_code_id uuid NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                course_id uuid NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                activity_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                description text,
                start_date date,
                end_date date,
                practice_start_date date,
                practice_end_date date,
                curriculum_detail_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                teach_decree_id uuid NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                is_lecturer_credit_sum_problem boolean DEFAULT false,
                is_lock boolean DEFAULT false,
                max_member integer DEFAULT 0,
                encounter_category_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                scope_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                feeder_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                created_at timestamp(0) without time zone DEFAULT now(),
                updated_at timestamp(0) without time zone DEFAULT now(),
                deleted_at timestamp(0) without time zone,
                sync_at timestamp(0) without time zone,
                created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                CONSTRAINT act_teaches_pkey PRIMARY KEY (id)
            )
            ",
        )
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP TABLE IF EXISTS academic_campaign_transaction.teaches")
            .await?;

        Ok(())
    }
}
