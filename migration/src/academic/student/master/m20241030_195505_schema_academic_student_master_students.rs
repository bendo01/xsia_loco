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
            CREATE SCHEMA IF NOT EXISTS academic_student_master;
            ",
        )
        .await?;
        db.execute_unprepared(
            "
            CREATE TABLE IF NOT EXISTS academic_student_master.students
            (
                id uuid DEFAULT uuid_generate_v7(),
                code character varying(255) NOT NULL,
                name character varying(255) NOT NULL,
                selection_type_id uuid NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                registered date NOT NULL,
                individual_id uuid NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                status_id uuid NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                unit_id uuid NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                academic_year_id uuid NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                registration_id uuid NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                nisn character varying(255),
                resign_status_id uuid NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                concentration_id uuid NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                curriculum_id uuid NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                class_code_id uuid NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                transfer_code character varying(255),
                transfer_unit_id uuid NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                feeder_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                feeder_registration_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                finance_fee double precision DEFAULT 0,
                finance_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                created_at timestamp(0) without time zone DEFAULT now(),
                updated_at timestamp(0) without time zone DEFAULT now(),
                deleted_at timestamp(0) without time zone,
                sync_at timestamp(0) without time zone,
                created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                CONSTRAINT asm_students_pkey PRIMARY KEY (id)
            )
            ",
        )
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP TABLE IF EXISTS academic_student_master.students")
            .await?;

        Ok(())
    }
}
