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
            CREATE TABLE IF NOT EXISTS payment_midtrans.transaction_details
            (
                id uuid DEFAULT uuid_generate_v7(),
                order_id character varying(255) NOT NULL,
                gross_amount DOUBLE PRECISION DEFAULT 0,
                account_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                transaction_detailable_id uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                transaction_detailable_type character varying(255) NOT NULL,
                is_paid boolean DEFAULT false,
                created_at timestamp(0) without time zone DEFAULT now(),
                updated_at timestamp(0) without time zone DEFAULT now(),
                deleted_at timestamp(0) with time zone,
                sync_at timestamp without time zone,
                created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                CONSTRAINT payment_midtrans_transaction_details_pkey PRIMARY KEY (id)
            )
            ",
        )
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP TABLE IF EXISTS payment_midtrans.transaction_details")
            .await?;

        Ok(())
    }
}
