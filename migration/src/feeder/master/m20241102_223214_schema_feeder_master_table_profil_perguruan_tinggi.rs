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
            CREATE SCHEMA IF NOT EXISTS feeder_master;
            ",
        )
        .await?;
        db.execute_unprepared(
            "
            CREATE TABLE IF NOT EXISTS feeder_master.profil_perguruan_tinggi
            (
                id uuid DEFAULT uuid_generate_v7(),
                id_perguruan_tinggi uuid,
                kode_perguruan_tinggi character varying(255),
                nama_perguruan_tinggi character varying(255),
                telepon character varying(255),
                faximile character varying(255),
                email character varying(255),
                website character varying(255),
                jalan character varying(255),
                dusun character varying(255),
                rt_rw integer,
                kelurahan character varying(255),
                kode_pos character varying(255),
                id_wilayah character varying(255),
                nama_wilayah character varying(255),
                lintang_bujur character varying(255),
                bank character varying(255),
                unit_cabang character varying(255),
                nomor_rekening character varying(255),
                mbs character varying(255),
                luas_tanah_milik character varying(255),
                luas_tanah_bukan_milik character varying(255),
                sk_pendirian character varying(255),
                tanggal_sk_pendirian character varying(255),
                id_status_milik character varying(255),
                nama_status_milik character varying(255),
                status_perguruan_tinggi character varying(255),
                sk_izin_operasional character varying(255),
                tanggal_izin_operasional date,
                created_at timestamp(0) without time zone DEFAULT now(),
                updated_at timestamp(0) without time zone DEFAULT now(),
                deleted_at timestamp(0) without time zone,
                sync_at timestamp(0) without time zone,
                created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                CONSTRAINT feeder_master_profil_perguruan_tinggi_pkey PRIMARY KEY (id)
            )
            ",
        )
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP TABLE IF EXISTS feeder_master.profil_perguruan_tinggi")
            .await?;

        Ok(())
    }
}
