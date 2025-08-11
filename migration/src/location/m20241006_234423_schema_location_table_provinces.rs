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
            CREATE SCHEMA IF NOT EXISTS location;
            ",
        )
        .await?;
        db.execute_unprepared(
            "
            CREATE TABLE IF NOT EXISTS location.provinces
            (
                id uuid DEFAULT uuid_generate_v7(),
                code character varying(255)  DEFAULT NULL::character varying,
                name character varying(255) ,
                dikti_code character varying(255)  DEFAULT NULL::character varying,
                epsbed_code character varying(255)  DEFAULT NULL::character varying,
                slug character varying(255)  DEFAULT NULL::character varying,
                description text ,
                alt_slug character varying(255)  DEFAULT NULL::character varying,
                state_ministry_code character varying(255)  DEFAULT NULL::character varying,
                state_ministry_full_code character varying(255)  DEFAULT NULL::character varying,
                state_post_department_code character varying(255)  DEFAULT NULL::character varying,
                state_ministry_name character varying(255)  DEFAULT NULL::character varying,
                dikti_name character varying(255)  DEFAULT NULL::character varying,
                validation_code character varying(255)  DEFAULT NULL::character varying,
                latitude double precision DEFAULT 0,
                longitude double precision DEFAULT 0,
                zoom integer DEFAULT 0,
                country_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                created_at timestamp(0) without time zone DEFAULT now(),
                updated_at timestamp(0) without time zone DEFAULT now(),
                sync_at timestamp without time zone,
                deleted_at timestamp(0) without time zone,
                created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                CONSTRAINT location_provinces_pkey PRIMARY KEY (id)
            )
            ",
        )
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP TABLE IF EXISTS location.provinces")
            .await?;

        Ok(())
    }
}
