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
            CREATE SCHEMA IF NOT EXISTS person_master;
            ",
        )
        .await?;
        db.execute_unprepared(
            "
            CREATE TABLE IF NOT EXISTS person_master.individuals
            (
                id uuid DEFAULT uuid_generate_v7(),
                code character varying(255) NOT NULL,
                name character varying(255) NOT NULL,
                front_title character varying(255),
                last_title character varying(255),
                birth_date date NOT NULL,
                birth_place character varying(255) NOT NULL,
                gender_id uuid NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                religion_id uuid NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                occupation_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                education_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                income_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                identification_type_id uuid NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                marital_status_id uuid NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                profession_id uuid NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                age_classification_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                is_special_need boolean NOT NULL DEFAULT false,
                is_social_protection_card_recipient boolean NOT NULL DEFAULT false,
                is_deceased boolean DEFAULT false,
                created_at timestamp(0) without time zone DEFAULT now(),
                updated_at timestamp(0) without time zone DEFAULT now(),
                sync_at timestamp without time zone,
                deleted_at timestamp(0) without time zone,
                created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                CONSTRAINT pm_individuals_pkey PRIMARY KEY (id)
            )
            ",
        )
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP TABLE IF EXISTS person_master.individuals")
            .await?;

        Ok(())
    }
}
