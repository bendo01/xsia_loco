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
            CREATE SCHEMA IF NOT EXISTS auth;
            ",
        )
        .await?;
        db.execute_unprepared(
            "
            CREATE TABLE IF NOT EXISTS auth.users
            (
                id uuid DEFAULT uuid_generate_v7(),
                name character varying(255) NOT NULL,
                email character varying(255) NOT NULL,
                email_verified_at timestamp(0) without time zone,
                password character varying(255) NOT NULL,
                remember_token character varying(100),
                individual_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                is_active boolean DEFAULT true,
                current_role_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                pid uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                api_key character varying,
                reset_token character varying,
                email_verification_token character varying,
                reset_sent_at timestamp(6) without time zone,
                email_verification_sent_at timestamp(6) without time zone,
                magic_link_token character varying,
                magic_link_expiration timestamp(6) without time zone,
                created_at timestamp(0) without time zone DEFAULT now(),
                updated_at timestamp(0) without time zone DEFAULT now(),
                deleted_at timestamp(0) without time zone,
                sync_at timestamp(0) without time zone,
                created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                CONSTRAINT auth_users_pkey PRIMARY KEY (id)
            )
            ",
        )
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP TABLE IF EXISTS auth.users")
            .await?;

        Ok(())
    }
}
