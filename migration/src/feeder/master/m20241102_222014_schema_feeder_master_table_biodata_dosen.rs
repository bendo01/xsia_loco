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
            CREATE TABLE IF NOT EXISTS feeder_master.biodata_dosen
            (
                id uuid DEFAULT uuid_generate_v7(),
                id_dosen character varying(255),
                nama_dosen character varying(255),
                tempat_lahir character varying(255),
                tanggal_lahir date,
                jenis_kelamin character varying(255),
                id_agama character varying(255),
                nama_agama character varying(255) ,
                id_status_aktif character varying(255),
                nama_status_aktif character varying(255),
                nidn character varying(255),
                nama_ibu_kandung character varying(255),
                nik character varying(255),
                nip character varying(255),
                npwp character varying(255),
                id_jenis_sdm character varying(255),
                nama_jenis_sdm character varying(255),
                no_sk_cpns character varying(255),
                tanggal_sk_cpns date,
                no_sk_pengangkatan character varying(255),
                mulai_sk_pengangkatan character varying(255),
                id_lembaga_pengangkatan character varying(255),
                nama_lembaga_pengangkatan character varying(255),
                id_pangkat_golongan character varying(255),
                nama_pangkat_golongan character varying(255) ,
                id_sumber_gaji character varying(255),
                nama_sumber_gaji character varying(255),
                jalan character varying(255),
                dusun character varying(255),
                rt character varying(255),
                rw character varying(255),
                ds_kel character varying(255),
                kode_pos character varying(255) ,
                id_wilayah character varying(255),
                nama_wilayah character varying(255) ,
                telepon character varying(255),
                handphone character varying(255),
                email character varying(255),
                status_pernikahan character varying(255),
                nama_suami_istri character varying(255),
                nip_suami_istri character varying(255) ,
                tanggal_mulai_pns date,
                id_pekerjaan_suami_istri character varying(255),
                nama_pekerjaan_suami_istri character varying(255) ,
                created_at timestamp(0) without time zone DEFAULT now(),
                updated_at timestamp(0) without time zone DEFAULT now(),
                deleted_at timestamp(0) without time zone,
                sync_at timestamp(0) without time zone,
                created_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                updated_by uuid DEFAULT '00000000-0000-0000-0000-000000000000'::uuid,
                CONSTRAINT feeder_master_biodata_dosen_pkey PRIMARY KEY (id)
            )
            ",
        )
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP TABLE IF EXISTS feeder_master.biodata_dosen")
            .await?;

        Ok(())
    }
}
