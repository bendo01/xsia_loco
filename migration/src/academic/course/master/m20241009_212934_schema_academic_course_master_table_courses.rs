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
            CREATE SCHEMA IF NOT EXISTS academic_course_master;
            ",
        )
        .await?;
        db.execute_unprepared(
            "
            CREATE TABLE IF NOT EXISTS academic_course_master.courses
            (
                id uuid DEFAULT uuid_generate_v7(),
                code character varying(255) NOT NULL,
                name character varying(255) NOT NULL,
                implementation_method text,
                total_credit double precision DEFAULT '0'::double precision,
                lecture_credit double precision DEFAULT '0'::double precision,
                practice_credit double precision DEFAULT '0'::double precision,
                field_practice_credit double precision DEFAULT '0'::double precision,
                simulation_credit double precision DEFAULT '0'::double precision,
                has_unit boolean NOT NULL DEFAULT false,
                has_syllabus boolean NOT NULL DEFAULT false,
                has_material boolean NOT NULL DEFAULT false,
                has_practice boolean NOT NULL DEFAULT false,
                has_dictation boolean NOT NULL DEFAULT false,
                group_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                variety_id uuid NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                unit_id uuid NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                competence_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                feeder_course_group_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                feeder_course_type_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                feeder_course_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                start_date date,
                end_date date,
                created_at timestamp(0) without time zone DEFAULT now(),
                updated_at timestamp(0) without time zone DEFAULT now(),
                deleted_at timestamp(0) without time zone,
                sync_at timestamp(0) without time zone,
                created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                CONSTRAINT acm_courses_pkey PRIMARY KEY (id)
            )
            ",
        )
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP TABLE IF EXISTS academic_course_master.courses")
            .await?;

        Ok(())
    }
}
