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
            CREATE SCHEMA IF NOT EXISTS payment_midtrans;
            ",
        )
        .await?;
        db.execute_unprepared(
            "
            CREATE TABLE IF NOT EXISTS payment_midtrans.accounts
            (
                id uuid DEFAULT uuid_generate_v7(),
                name character varying(255) NOT NULL,
                institution_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                merchant_identification character varying(255) NOT NULL,
                client_key character varying(255) NOT NULL,
                server_key character varying(255) NOT NULL,
                sandbox_url character varying(255) NOT NULL,
                production_url character varying(255) NOT NULL,
                is_production boolean NOT NULL DEFAULT false,
                is_sanitized boolean NOT NULL DEFAULT true,
                is_3ds boolean NOT NULL DEFAULT true,
                created_at timestamp(0) without time zone DEFAULT now(),
                updated_at timestamp(0) without time zone DEFAULT now(),
                deleted_at timestamp(0) with time zone,
                sync_at timestamp without time zone,
                created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                CONSTRAINT payment_midtrans_accounts_pkey PRIMARY KEY (id)
            )
            ",
        )
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP TABLE IF EXISTS payment_midtrans.accounts")
            .await?;

        Ok(())
    }
}
