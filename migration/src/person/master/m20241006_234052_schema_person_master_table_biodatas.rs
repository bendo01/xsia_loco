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
            CREATE TABLE IF NOT EXISTS person_master.biodatas
            (
                id uuid DEFAULT uuid_generate_v7(),
                height double precision DEFAULT '0'::double precision,
                weight double precision DEFAULT '0'::double precision,
                is_positive_blood_rhesus boolean NOT NULL DEFAULT false,
                blood_type_id uuid NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                hair_type_id uuid NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                hair_color_id uuid NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                eye_color_id uuid NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                individual_id uuid NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                bust double precision NOT NULL DEFAULT '0'::double precision,
                waist double precision NOT NULL DEFAULT '0'::double precision,
                hip double precision NOT NULL DEFAULT '0'::double precision,
                arm_circumference double precision NOT NULL DEFAULT '0'::double precision,
                menarche_age integer NOT NULL DEFAULT 0,
                menopause_age integer NOT NULL DEFAULT 0,
                created_at timestamp(0) without time zone DEFAULT now(),
                updated_at timestamp(0) without time zone DEFAULT now(),
                sync_at timestamp without time zone,
                deleted_at timestamp(0) without time zone,
                created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                CONSTRAINT pm_biodatas_pkey PRIMARY KEY (id)
            )
            ",
        )
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP TABLE IF EXISTS person_master.biodatas")
            .await?;

        Ok(())
    }
}
