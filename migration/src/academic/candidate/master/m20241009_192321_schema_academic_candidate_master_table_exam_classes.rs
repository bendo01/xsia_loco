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
            CREATE SCHEMA IF NOT EXISTS academic_candidate_master;
            ",
        )
        .await?;
        db.execute_unprepared(
            "
            CREATE TABLE IF NOT EXISTS academic_candidate_master.exam_classes
            (
                id uuid DEFAULT uuid_generate_v7(),
                code integer,
                alphabet_code character varying(255),
                name character varying(255) NOT NULL,
                phase_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                start_date timestamp(0) without time zone NOT NULL,
                end_date timestamp(0) without time zone,
                start_time time(0) without time zone,
                end_time time(0) without time zone,
                capacity integer NOT NULL DEFAULT 0,
                lms_category integer DEFAULT 0,
                is_online boolean DEFAULT false,
                created_at timestamp(0) without time zone DEFAULT now(),
                updated_at timestamp(0) without time zone DEFAULT now(),
                deleted_at timestamp(0) without time zone,
                sync_at timestamp(0) without time zone,
                created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                CONSTRAINT acm_exam_classes_pkey PRIMARY KEY (id)
            )
            ",
        )
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP TABLE IF EXISTS academic_candidate_master.exam_classes")
            .await?;

        Ok(())
    }
}
