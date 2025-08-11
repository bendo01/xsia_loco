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
            CREATE SCHEMA IF NOT EXISTS academic_student_final_assignment_transaction;
            ",
        )
        .await?;
        db.execute_unprepared(
            "
            CREATE TABLE IF NOT EXISTS academic_student_final_assignment_transaction.submissions
            (
                id uuid DEFAULT uuid_generate_v7(),
                title text,
                student_id uuid NOT NULL,
                approval_type_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                category_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                stage_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                final_assignment_decree_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                detail_activity_id uuid NOT NULL,
                is_taken timestamp(0) without time zone,
                is_lock timestamp(0) without time zone,
                filename character varying(255),
                dir character varying(255),
                type character varying(255),
                filesize integer,
                created_at timestamp(0) without time zone DEFAULT now(),
                updated_at timestamp(0) without time zone DEFAULT now(),
                deleted_at timestamp(0) without time zone,
                sync_at timestamp(0) without time zone,
                created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                CONSTRAINT academic_student_final_assignment_transaction_submissions_pkey PRIMARY KEY (id)
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
                "DROP TABLE IF EXISTS academic_student_final_assignment_transaction.submissions",
            )
            .await?;

        Ok(())
    }
}
