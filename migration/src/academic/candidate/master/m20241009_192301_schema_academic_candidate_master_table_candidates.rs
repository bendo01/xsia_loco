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
            CREATE TABLE IF NOT EXISTS academic_candidate_master.candidates
            (
                id uuid DEFAULT uuid_generate_v7(),
                thread integer DEFAULT 0,
                code character varying(255),
                name character varying(255) NOT NULL,
                student_national_number character varying(255),
                school_name character varying(255),
                school_regency_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                state_smart_card_number character varying(255),
                individual_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                academic_year_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                student_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                user_id uuid NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                registration_type_id uuid NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                institution_id uuid NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                guidence_name character varying(255),
                guidence_phone_number character varying(255),
                created_at timestamp(0) without time zone DEFAULT now(),
                updated_at timestamp(0) without time zone DEFAULT now(),
                deleted_at timestamp(0) without time zone,
                sync_at timestamp(0) without time zone,
                created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                CONSTRAINT acm_candidates_pkey PRIMARY KEY (id)
            )
            ",
        )
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP TABLE IF EXISTS academic_candidate_master.candidates")
            .await?;

        Ok(())
    }
}
