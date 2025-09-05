#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]

pub mod academic;
pub mod auth;
pub mod building;
pub mod contact;
pub mod document;
pub mod feeder;
pub mod general;
pub mod institution;
pub mod literate;
pub mod location;
pub mod payment;
pub mod person;

pub use sea_orm_migration::prelude::*;

use auth::m20241004_225447_schema_auth_table_users;
use auth::m20241102_053649_schema_auth_table_verifications;
use auth::m20241102_053701_schema_auth_table_permissions;
use auth::m20241102_053846_schema_auth_table_permission_user;
use auth::m20241102_053904_schema_auth_table_permission_position_type;
use auth::m20241102_053946_schema_auth_table_user_position_type;

use academic::campaign::reference::m20241009_182114_schema_academic_campaign_reference_table_attend_types;
use academic::campaign::reference::m20241009_182131_schema_academic_campaign_reference_table_calendar_categories;
use academic::campaign::reference::m20241009_183601_schema_academic_campaign_reference_table_encounter_categories;
use academic::campaign::reference::m20241009_183630_schema_academic_campaign_reference_table_encounter_types;
use academic::campaign::reference::m20241009_183647_schema_academic_campaign_reference_table_evaluation_types;
use academic::campaign::reference::m20241009_185853_schema_academic_campaign_reference_table_implementations;
use academic::campaign::reference::m20241009_185905_schema_academic_campaign_reference_table_scopes;
use academic::campaign::reference::m20241009_185916_schema_academic_campaign_reference_table_substances;

use academic::campaign::transaction::m20241009_190134_schema_academic_campaign_transaction_table_activities;
use academic::campaign::transaction::m20241009_190213_schema_academic_campaign_transaction_table_calendar_details;
use academic::campaign::transaction::m20241009_190223_schema_academic_campaign_transaction_table_calendars;
use academic::campaign::transaction::m20241009_190245_schema_academic_campaign_transaction_table_class_codes;
use academic::campaign::transaction::m20241009_190258_schema_academic_campaign_transaction_table_grades;
use academic::campaign::transaction::m20241009_190312_schema_academic_campaign_transaction_table_schedules;
use academic::campaign::transaction::m20241009_191841_schema_academic_campaign_transaction_table_teach_decrees;
use academic::campaign::transaction::m20241009_191857_schema_academic_campaign_transaction_table_teach_lecturers;
use academic::campaign::transaction::m20241009_191907_schema_academic_campaign_transaction_table_teaches;

use academic::candidate::master::m20241009_192301_schema_academic_candidate_master_table_candidates;
use academic::candidate::master::m20241009_192321_schema_academic_candidate_master_table_exam_classes;
use academic::candidate::reference::m20241009_192552_schema_academic_candidate_reference_table_document_types;
use academic::candidate::reference::m20241009_192711_schema_academic_candidate_reference_table_phases;
use academic::candidate::transaction::m20241009_193611_schema_academic_candidate_transaction_table_candidate_unit_choices;
use academic::candidate::transaction::m20241009_195103_schema_academic_candidate_transaction_table_documents;
use academic::candidate::transaction::m20241009_195117_schema_academic_candidate_transaction_table_exams;

use academic::course::master::m20241009_210314_schema_academic_course_master_table_concentrations;
use academic::course::master::m20241009_212823_schema_academic_course_master_table_course_evaluation_plannings;
use academic::course::master::m20241009_212844_schema_academic_course_master_table_course_learn_plannings;
use academic::course::master::m20241009_212934_schema_academic_course_master_table_courses;
use academic::course::master::m20241009_212956_schema_academic_course_master_table_curriculum_details;
use academic::course::master::m20241009_213016_schema_academic_course_master_table_curriculums;

use academic::course::reference::m20241009_213206_schema_academic_course_reference_table_competences;
use academic::course::reference::m20241009_213353_schema_academic_course_reference_table_course_evaluation_bases;
use academic::course::reference::m20241009_213500_schema_academic_course_reference_table_curriculum_types;
use academic::course::reference::m20241009_213535_schema_academic_course_reference_table_groups;
use academic::course::reference::m20241009_213546_schema_academic_course_reference_table_semesters;
use academic::course::reference::m20241009_213558_schema_academic_course_reference_table_varieties;

use academic::general::reference::m20241009_213955_schema_academic_general_reference_table_academic_years;
use academic::general::reference::m20241009_214007_schema_academic_general_reference_table_academic_year_categories;

use academic::lecturer::reference::m20241030_184758_schema_academic_lecturer_reference_contracts;
use academic::lecturer::reference::m20241030_184813_schema_academic_lecturer_reference_groups;
use academic::lecturer::reference::m20241030_184825_schema_academic_lecturer_reference_ranks;
use academic::lecturer::reference::m20241030_184837_schema_academic_lecturer_reference_statuses;

use academic::lecturer::master::m20241009_232031_schema_academic_lecturer_master_table_lecturers;

use academic::lecturer::transaction::m20241030_185557_schema_academic_lecturer_transaction_academic_groups;
use academic::lecturer::transaction::m20241030_185617_schema_academic_lecturer_transaction_academic_ranks;
use academic::lecturer::transaction::m20241030_185646_schema_academic_lecturer_transaction_homebases;

use academic::prior_learning_recognition::reference::m20250627_232706_schema_prior_learning_recognition_reference_create_professionalisms;
use academic::prior_learning_recognition::reference::m20250628_010718_schema_prior_learning_recognition_reference_create_evidence_types;
use academic::prior_learning_recognition::reference::m20250628_010719_schema_prior_learning_recognition_reference_create_evidence_categories;
use academic::prior_learning_recognition::reference::m20250718_094014_schema_prior_learning_recognition_reference_create_evaluator_types;

use academic::prior_learning_recognition::transaction::m20250628_013351_schema_prior_learning_recognition_transaction_create_decrees;
use academic::prior_learning_recognition::transaction::m20250712_232734_schema_prior_learning_recognition_transaction_create_evaluations;
use academic::prior_learning_recognition::transaction::m20250712_233151_schema_prior_learning_recognition_transaction_create_evaluation_details;
use academic::prior_learning_recognition::transaction::m20250716_053017_schema_prior_learning_recognition_transaction_create_recognitions;
use academic::prior_learning_recognition::transaction::m20250718_092644_schema_prior_learning_recognition_transaction_create_evaluators;

use academic::student::adviser::m20241030_192659_schema_academic_student_adviser_counsellors;
use academic::student::adviser::m20241030_192712_schema_academic_student_adviser_decrees;

use academic::student::campaign::m20241030_193210_schema_academic_student_campaign_activities;
use academic::student::campaign::m20241030_193215_schema_academic_student_campaign_convertions;
use academic::student::campaign::m20241030_193223_schema_academic_student_campaign_detail_activities;
use academic::student::campaign::m20241030_193229_schema_academic_student_campaign_detail_activity_evaluation_components;

use academic::student::final_assignment::reference::m20241030_194123_schema_academic_student_final_assignment_reference_adviser_categories;
use academic::student::final_assignment::reference::m20241030_194150_schema_academic_student_final_assignment_reference_approval_types;
use academic::student::final_assignment::reference::m20241030_194212_schema_academic_student_final_assignment_reference_categories;
use academic::student::final_assignment::reference::m20241030_194330_schema_academic_student_final_assignment_reference_requirements;
use academic::student::final_assignment::reference::m20241030_194410_schema_academic_student_final_assignment_reference_stages;
use academic::student::final_assignment::reference::m20241030_194433_schema_academic_student_final_assignment_reference_varieties;

use academic::student::final_assignment::transaction::m20241030_194624_schema_academic_student_final_assignment_transaction_advisers;
use academic::student::final_assignment::transaction::m20241030_194720_schema_academic_student_final_assignment_transaction_evaluation_details;
use academic::student::final_assignment::transaction::m20241030_194746_schema_academic_student_final_assignment_transaction_evaluation_summaries;
use academic::student::final_assignment::transaction::m20241030_194838_schema_academic_student_final_assignment_transaction_final_assignment_decrees;
use academic::student::final_assignment::transaction::m20241030_194906_schema_academic_student_final_assignment_transaction_prerequisites;
use academic::student::final_assignment::transaction::m20241030_194931_schema_academic_student_final_assignment_transaction_schedules;
use academic::student::final_assignment::transaction::m20241030_194946_schema_academic_student_final_assignment_transaction_submissions;

use academic::student::master::m20241030_195454_schema_academic_student_master_images;
use academic::student::master::m20241030_195505_schema_academic_student_master_students;

use academic::student::reference::m20241030_195643_schema_academic_student_reference_finances;
use academic::student::reference::m20241030_195658_schema_academic_student_reference_registrations;
use academic::student::reference::m20241030_195719_schema_academic_student_reference_resign_statuses;
use academic::student::reference::m20241030_195740_schema_academic_student_reference_selection_types;
use academic::student::reference::m20241030_195754_schema_academic_student_reference_statuses;

use academic::survey::master::m20241030_200500_schema_academic_survey_master_answers;
use academic::survey::master::m20241030_200528_schema_academic_survey_master_bundle_question;
use academic::survey::master::m20241030_200559_schema_academic_survey_master_bundles;
use academic::survey::master::m20241030_200614_schema_academic_survey_master_questions;

use academic::survey::reference::m20241030_200649_schema_academic_survey_reference_bundle_categories;
use academic::survey::reference::m20241030_200711_schema_academic_survey_reference_question_varieties;

use academic::survey::transaction::m20241030_200838_schema_academic_survey_transaction_conducts;
use academic::survey::transaction::m20241030_200851_schema_academic_survey_transaction_responds;

use building::reference::m20241102_054329_schema_bulding_reference_table_categories;
use building::reference::m20241102_054343_schema_bulding_reference_table_conditions;
use building::reference::m20241102_054356_schema_bulding_reference_table_room_types;
use building::reference::m20241102_054412_schema_bulding_reference_table_varieties;

use building::master::m20241102_054200_schema_bulding_master_table_buildings;
use building::master::m20241102_054230_schema_bulding_master_table_rooms;

use contact::reference::m20241102_064407_schema_contact_reference_table_electronic_mail_types;
use contact::reference::m20241102_065934_schema_contact_reference_table_phone_types;
use contact::reference::m20241102_065944_schema_contact_reference_table_residence_types;
use contact::reference::m20241102_065956_schema_contact_reference_table_website_types;

use contact::master::m20241102_064053_schema_contact_master_table_electronic_mails;
use contact::master::m20241102_064106_schema_contact_master_table_phones;
use contact::master::m20241102_064249_schema_contact_master_table_residences;
use contact::master::m20241102_064301_schema_contact_master_table_websites;

use document::reference::m20250628_012704_schema_document_reference_create_archive_types;
use document::transaction::m20250628_013155_schema_document_transaction_create_archives;

use feeder::akumulasi::m20241102_071050_schema_feeder_akumulasi_table_estimasi;
use feeder::akumulasi::m20241102_071102_schema_feeder_akumulasi_table_jumlah_data;
use feeder::akun::m20241102_071248_schema_feeder_akun_table_kredensial;

use feeder::master::m20241102_221502_schema_feeder_master_table_aktifitas_kuliah_mahasiswa;
use feeder::master::m20241102_221513_schema_feeder_master_table_aktifitas_mahasiswa;
use feeder::master::m20241102_221730_schema_feeder_master_table_aktifitas_mengajar_dosen;
use feeder::master::m20241102_221753_schema_feeder_master_table_anggota_aktifitas_mahasiswa;
use feeder::master::m20241102_221833_schema_feeder_master_table_bidang_minat_perguruan_tinggi;
use feeder::master::m20241102_221853_schema_feeder_master_table_bimbing_mahasiswa;
use feeder::master::m20241102_222014_schema_feeder_master_table_biodata_dosen;
use feeder::master::m20241102_222024_schema_feeder_master_table_biodata_mahasiswa;
use feeder::master::m20241102_222036_schema_feeder_master_table_dosen;
use feeder::master::m20241102_222052_schema_feeder_master_table_dosen_pembimbing;
use feeder::master::m20241102_222109_schema_feeder_master_table_dosen_pengajar_kelas_kuliah;
use feeder::master::m20241102_222124_schema_feeder_master_table_fakultas;
use feeder::master::m20241102_222158_schema_feeder_master_table_hitung_transkrip_angkatan_mahasiswa;
use feeder::master::m20241102_222232_schema_feeder_master_table_kartu_rencana_studi_mahasiswa;
use feeder::master::m20241102_222250_schema_feeder_master_table_kelas_kuliah;
use feeder::master::m20241102_222305_schema_feeder_master_table_konsistensi_data;
use feeder::master::m20241102_222400_schema_feeder_master_table_konversi_kampus_merdeka;
use feeder::master::m20241102_222415_schema_feeder_master_table_kurikulum;
use feeder::master::m20241102_222431_schema_feeder_master_table_mahasiswa;
use feeder::master::m20241102_222557_schema_feeder_master_table_mahasiswa_bimbingan_dosen;
use feeder::master::m20241102_222634_schema_feeder_master_table_mahasiswa_lulus_dropout;
use feeder::master::m20241102_222715_schema_feeder_master_table_matakuliah;
use feeder::master::m20241102_222726_schema_feeder_master_table_matakuliah_kurikulum;
use feeder::master::m20241102_222758_schema_feeder_master_table_nilai_perkuliahan_kelas;
use feeder::master::m20241102_222825_schema_feeder_master_table_nilai_transfer_pendidikan_mahasiswa;
use feeder::master::m20241102_222918_schema_feeder_master_table_penugasan_dosen;
use feeder::master::m20241102_222933_schema_feeder_master_table_perguruan_tinggi;
use feeder::master::m20241102_222950_schema_feeder_master_table_periode_aktif;
use feeder::master::m20241102_223007_schema_feeder_master_table_periode_perkuliahan;
use feeder::master::m20241102_223023_schema_feeder_master_table_perkuliahan_mahasiswa;
use feeder::master::m20241102_223047_schema_feeder_master_table_peserta_kelas_kuliah;
use feeder::master::m20241102_223157_schema_feeder_master_table_prestasi_mahasiswa;
use feeder::master::m20241102_223214_schema_feeder_master_table_profil_perguruan_tinggi;
use feeder::master::m20241103_090128_schema_feeder_master_table_profil_program_studi;
use feeder::master::m20241103_090145_schema_feeder_master_table_profil_rencana_evaluasi;
use feeder::master::m20241103_090202_schema_feeder_master_table_riwayat_fungsional_dosen;
use feeder::master::m20241103_090216_schema_feeder_master_table_riwayat_nilai_mahasiswa;
use feeder::master::m20241103_090230_schema_feeder_master_table_riwayat_pangkat_dosen;
use feeder::master::m20241103_090243_schema_feeder_master_table_riwayat_pendidikan_dosen;
use feeder::master::m20241103_090256_schema_feeder_master_table_riwayat_penelitian_dosen;
use feeder::master::m20241103_090307_schema_feeder_master_table_riwayat_sertifikasi_dosen;
use feeder::master::m20241103_090327_schema_feeder_master_table_skala_nilai_program_studi;
use feeder::master::m20241103_090342_schema_feeder_master_table_substansi_matakuliah;
use feeder::master::m20241103_090359_schema_feeder_master_table_transkrip_mahasiswa;
use feeder::master::m20241103_090410_schema_feeder_master_table_uji_mahasiswa;
use feeder::master::m20250904_131434_schema_feeder_master_table_komponen_evaluasi_kelas;

use feeder::referensi::m20241103_090507_schema_feeder_referensi_table_agama;
use feeder::referensi::m20241104_211659_schema_feeder_referensi_table_alat_transportasi;
use feeder::referensi::m20241104_211727_schema_feeder_referensi_table_bentuk_pendidikan;
use feeder::referensi::m20241104_211748_schema_feeder_referensi_table_ikatan_sumber_daya_manusia;
use feeder::referensi::m20241104_211913_schema_feeder_referensi_table_jabatan_fungsional;
use feeder::referensi::m20241104_211925_schema_feeder_referensi_table_jalur_masuk;
use feeder::referensi::m20241104_211955_schema_feeder_referensi_table_jenis_aktifitas_mahasiswa;
use feeder::referensi::m20241104_212021_schema_feeder_referensi_table_jenis_evaluasi;
use feeder::referensi::m20241104_212041_schema_feeder_referensi_table_jenis_keluar;
use feeder::referensi::m20241104_212058_schema_feeder_referensi_table_jenis_pendaftaran;
use feeder::referensi::m20241104_212202_schema_feeder_referensi_table_jenis_prestasi;
use feeder::referensi::m20241104_212254_schema_feeder_referensi_table_jenis_satuan_manajemen_sumberdaya;
use feeder::referensi::m20241104_212311_schema_feeder_referensi_table_jenis_sertifikasi;
use feeder::referensi::m20241104_212327_schema_feeder_referensi_table_jenis_substansi;
use feeder::referensi::m20241104_212340_schema_feeder_referensi_table_jenis_tinggal;
use feeder::referensi::m20241104_212409_schema_feeder_referensi_table_jenjang_pendidikan;
use feeder::referensi::m20241104_212428_schema_feeder_referensi_table_kategori_kegiatan;
use feeder::referensi::m20241104_212447_schema_feeder_referensi_table_kebutuhan_khusus;
use feeder::referensi::m20241104_212504_schema_feeder_referensi_table_lembaga_pengangkat;
use feeder::referensi::m20241104_212528_schema_feeder_referensi_table_level_wilayah;
use feeder::referensi::m20241104_212550_schema_feeder_referensi_table_negara;
use feeder::referensi::m20241104_212725_schema_feeder_referensi_table_pangkat_golongan;
use feeder::referensi::m20241104_212807_schema_feeder_referensi_table_pekerjaan;
use feeder::referensi::m20241104_212929_schema_feeder_referensi_table_pembiayaan;
use feeder::referensi::m20241104_212942_schema_feeder_referensi_table_penghasilan;
use feeder::referensi::m20241104_213001_schema_feeder_referensi_table_periode_lampau;
use feeder::referensi::m20241104_213059_schema_feeder_referensi_table_semester;
use feeder::referensi::m20241104_213126_schema_feeder_referensi_table_status_keaktifan_pegawai;
use feeder::referensi::m20241104_213145_schema_feeder_referensi_table_status_kepegawaian;
use feeder::referensi::m20241104_213240_schema_feeder_referensi_table_status_mahasiswa;
use feeder::referensi::m20241104_213312_schema_feeder_referensi_table_tahun_ajaran;
use feeder::referensi::m20241104_213340_schema_feeder_referensi_table_tingkat_prestasi;
use feeder::referensi::m20241104_213355_schema_feeder_referensi_table_wilayah;

use feeder::rekapitulasi::m20241104_214551_schema_feeder_rekapitulasi_table_indeks_prestasi_sementara_mahasiswa;
use feeder::rekapitulasi::m20241104_214610_schema_feeder_rekapitulasi_table_jumlah_dosen;
use feeder::rekapitulasi::m20241104_214622_schema_feeder_rekapitulasi_table_jumlah_mahasiswa;
use feeder::rekapitulasi::m20241104_214640_schema_feeder_rekapitulasi_table_kartu_hasil_studi_mahasiswa;
use feeder::rekapitulasi::m20241104_214655_schema_feeder_rekapitulasi_table_kartu_rencana_studi_mahasiswa;
use feeder::rekapitulasi::m20241104_214717_schema_feeder_rekapitulasi_table_laporan;

use institution::reference::m20241007_002103_schema_institution_reference_table_categories;
use institution::reference::m20241007_002116_schema_institution_reference_table_position_types;
use institution::reference::m20241007_002241_schema_institution_reference_table_unit_types;
use institution::reference::m20241007_002256_schema_institution_reference_table_varieties;

use institution::master::m20241007_002412_schema_institution_master_table_employees;
use institution::master::m20241007_002423_schema_institution_master_table_institutions;
use institution::master::m20241007_002437_schema_institution_master_table_staffes;
use institution::master::m20241007_002447_schema_institution_master_table_units;

use literate::m20241007_000346_schema_literate_table_categories;
use literate::m20241007_000358_schema_literate_table_educations;
use literate::m20241007_000408_schema_literate_table_groups;
use literate::m20241007_000417_schema_literate_table_levels;
use literate::m20241007_000434_schema_literate_table_varieties;

use location::m20241006_234356_schema_location_table_continents;
use location::m20241006_234412_schema_location_table_countries;
use location::m20241006_234423_schema_location_table_provinces;
use location::m20241006_234434_schema_location_table_regencies;
use location::m20241006_234445_schema_location_table_regency_types;
use location::m20241006_234456_schema_location_table_regions;
use location::m20241006_234506_schema_location_table_sub_districts;
use location::m20241006_234516_schema_location_table_villages;

use person::reference::m20241004_232942_schema_person_reference_table_age_classifications;
use person::reference::m20241004_233159_schema_person_reference_table_blood_types;
use person::reference::m20241004_233211_schema_person_reference_table_eye_colors;
use person::reference::m20241004_233230_schema_person_reference_table_genders;
use person::reference::m20241004_233302_schema_person_reference_table_hair_colors;
use person::reference::m20241004_233308_schema_person_reference_table_hair_types;
use person::reference::m20241004_233320_schema_person_reference_table_identification_types;
use person::reference::m20241004_233506_schema_person_reference_table_incomes;
use person::reference::m20241004_233516_schema_person_reference_table_marital_statuses;
use person::reference::m20241004_233530_schema_person_reference_table_occupations;
use person::reference::m20241004_233539_schema_person_reference_table_professions;
use person::reference::m20241004_233547_schema_person_reference_table_relative_types;
use person::reference::m20241004_233555_schema_person_reference_table_religions;

use person::master::m20241006_234052_schema_person_master_table_biodatas;
use person::master::m20241006_234137_schema_person_master_table_family_card_members;
use person::master::m20241006_234146_schema_person_master_table_family_cards;
use person::master::m20241006_234205_schema_person_master_table_individuals;

// use feeder::akun::m20241102_071248_schema_feeder_akun_table_kredensial;

use payment::midtrans::m20250501_124920_schema_payment_midtrans_payment_create_accounts;
use payment::midtrans::m20250521_163052_schema_payment_midtrans_payment_create_transaction_details;
use payment::midtrans::m20250521_164437_schema_payment_midtrans_payment_create_customer_details;
use payment::midtrans::m20250521_164556_schema_payment_midtrans_payment_create_item_details;
use payment::midtrans::m20250521_170855_schema_payment_midtrans_payment_create_billing_addresses;
use payment::midtrans::m20250521_170952_schema_payment_midtrans_payment_create_shipping_addresses;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    #[allow(clippy::too_many_lines)]
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241004_225447_schema_auth_table_users::Migration),
            Box::new(m20241102_053649_schema_auth_table_verifications::Migration),
            Box::new(m20241102_053701_schema_auth_table_permissions::Migration),
            Box::new(m20241102_053846_schema_auth_table_permission_user::Migration),
            Box::new(m20241102_053904_schema_auth_table_permission_position_type::Migration),
            Box::new(m20241102_053946_schema_auth_table_user_position_type::Migration),

            Box::new(m20241009_182114_schema_academic_campaign_reference_table_attend_types::Migration),
            Box::new(m20241009_182131_schema_academic_campaign_reference_table_calendar_categories::Migration),
            Box::new(m20241009_183601_schema_academic_campaign_reference_table_encounter_categories::Migration),
            Box::new(m20241009_183630_schema_academic_campaign_reference_table_encounter_types::Migration),
            Box::new(m20241009_183647_schema_academic_campaign_reference_table_evaluation_types::Migration),
            Box::new(m20241009_185853_schema_academic_campaign_reference_table_implementations::Migration),
            Box::new(m20241009_185905_schema_academic_campaign_reference_table_scopes::Migration),
            Box::new(m20241009_185916_schema_academic_campaign_reference_table_substances::Migration),

            Box::new(m20241009_190134_schema_academic_campaign_transaction_table_activities::Migration),
            Box::new(m20241009_190213_schema_academic_campaign_transaction_table_calendar_details::Migration),
            Box::new(m20241009_190223_schema_academic_campaign_transaction_table_calendars::Migration),
            Box::new(m20241009_190245_schema_academic_campaign_transaction_table_class_codes::Migration),
            Box::new(m20241009_190258_schema_academic_campaign_transaction_table_grades::Migration),
            Box::new(m20241009_190312_schema_academic_campaign_transaction_table_schedules::Migration),
            Box::new(m20241009_191841_schema_academic_campaign_transaction_table_teach_decrees::Migration),
            Box::new(m20241009_191857_schema_academic_campaign_transaction_table_teach_lecturers::Migration),
            Box::new(m20241009_191907_schema_academic_campaign_transaction_table_teaches::Migration),

            Box::new(m20241009_192301_schema_academic_candidate_master_table_candidates::Migration),
            Box::new(m20241009_192321_schema_academic_candidate_master_table_exam_classes::Migration),
            Box::new(m20241009_192552_schema_academic_candidate_reference_table_document_types::Migration),
            Box::new(m20241009_192711_schema_academic_candidate_reference_table_phases::Migration),
            Box::new(m20241009_193611_schema_academic_candidate_transaction_table_candidate_unit_choices::Migration),
            Box::new(m20241009_195103_schema_academic_candidate_transaction_table_documents::Migration),
            Box::new(m20241009_195117_schema_academic_candidate_transaction_table_exams::Migration),

            Box::new(m20241009_210314_schema_academic_course_master_table_concentrations::Migration),
            Box::new(m20241009_212823_schema_academic_course_master_table_course_evaluation_plannings::Migration),
            Box::new(m20241009_212844_schema_academic_course_master_table_course_learn_plannings::Migration),
            Box::new(m20241009_212934_schema_academic_course_master_table_courses::Migration),
            Box::new(m20241009_212956_schema_academic_course_master_table_curriculum_details::Migration),
            Box::new(m20241009_213016_schema_academic_course_master_table_curriculums::Migration),

            Box::new(m20241009_213206_schema_academic_course_reference_table_competences::Migration),
            Box::new(m20241009_213353_schema_academic_course_reference_table_course_evaluation_bases::Migration),
            Box::new(m20241009_213500_schema_academic_course_reference_table_curriculum_types::Migration),
            Box::new(m20241009_213535_schema_academic_course_reference_table_groups::Migration),
            Box::new(m20241009_213546_schema_academic_course_reference_table_semesters::Migration),
            Box::new(m20241009_213558_schema_academic_course_reference_table_varieties::Migration),

            Box::new(m20241009_213955_schema_academic_general_reference_table_academic_years::Migration),
            Box::new(m20241009_214007_schema_academic_general_reference_table_academic_year_categories::Migration),

            Box::new(m20241030_184758_schema_academic_lecturer_reference_contracts::Migration),
            Box::new(m20241030_184813_schema_academic_lecturer_reference_groups::Migration),
            Box::new(m20241030_184825_schema_academic_lecturer_reference_ranks::Migration),
            Box::new(m20241030_184837_schema_academic_lecturer_reference_statuses::Migration),

            Box::new(m20241009_232031_schema_academic_lecturer_master_table_lecturers::Migration),

            Box::new(m20241030_185557_schema_academic_lecturer_transaction_academic_groups::Migration),
            Box::new(m20241030_185617_schema_academic_lecturer_transaction_academic_ranks::Migration),
            Box::new(m20241030_185646_schema_academic_lecturer_transaction_homebases::Migration),

            Box::new(m20250627_232706_schema_prior_learning_recognition_reference_create_professionalisms::Migration),
            Box::new(m20250628_010718_schema_prior_learning_recognition_reference_create_evidence_types::Migration),
            Box::new(m20250628_010719_schema_prior_learning_recognition_reference_create_evidence_categories::Migration),
            Box::new(m20250718_094014_schema_prior_learning_recognition_reference_create_evaluator_types::Migration),
            Box::new(m20250628_013351_schema_prior_learning_recognition_transaction_create_decrees::Migration),
            Box::new(m20250712_232734_schema_prior_learning_recognition_transaction_create_evaluations::Migration),
            Box::new(m20250712_233151_schema_prior_learning_recognition_transaction_create_evaluation_details::Migration),
            Box::new(m20250716_053017_schema_prior_learning_recognition_transaction_create_recognitions::Migration),
            Box::new(m20250718_092644_schema_prior_learning_recognition_transaction_create_evaluators::Migration),

            Box::new(m20241030_192659_schema_academic_student_adviser_counsellors::Migration),
            Box::new(m20241030_192712_schema_academic_student_adviser_decrees::Migration),

            Box::new(m20241030_193210_schema_academic_student_campaign_activities::Migration),
            Box::new(m20241030_193215_schema_academic_student_campaign_convertions::Migration),
            Box::new(m20241030_193223_schema_academic_student_campaign_detail_activities::Migration),
            Box::new(m20241030_193229_schema_academic_student_campaign_detail_activity_evaluation_components::Migration),

            Box::new(m20241030_194123_schema_academic_student_final_assignment_reference_adviser_categories::Migration),
            Box::new(m20241030_194150_schema_academic_student_final_assignment_reference_approval_types::Migration),
            Box::new(m20241030_194212_schema_academic_student_final_assignment_reference_categories::Migration),
            Box::new(m20241030_194330_schema_academic_student_final_assignment_reference_requirements::Migration),
            Box::new(m20241030_194410_schema_academic_student_final_assignment_reference_stages::Migration),
            Box::new(m20241030_194433_schema_academic_student_final_assignment_reference_varieties::Migration),

            Box::new(m20241030_194624_schema_academic_student_final_assignment_transaction_advisers::Migration),
            Box::new(m20241030_194720_schema_academic_student_final_assignment_transaction_evaluation_details::Migration),
            Box::new(m20241030_194746_schema_academic_student_final_assignment_transaction_evaluation_summaries::Migration),
            Box::new(m20241030_194838_schema_academic_student_final_assignment_transaction_final_assignment_decrees::Migration),
            Box::new(m20241030_194906_schema_academic_student_final_assignment_transaction_prerequisites::Migration),
            Box::new(m20241030_194931_schema_academic_student_final_assignment_transaction_schedules::Migration),
            Box::new(m20241030_194946_schema_academic_student_final_assignment_transaction_submissions::Migration),

            Box::new(m20241030_195454_schema_academic_student_master_images::Migration),
            Box::new(m20241030_195505_schema_academic_student_master_students::Migration),

            Box::new(m20241030_195643_schema_academic_student_reference_finances::Migration),
            Box::new(m20241030_195658_schema_academic_student_reference_registrations::Migration),
            Box::new(m20241030_195719_schema_academic_student_reference_resign_statuses::Migration),
            Box::new(m20241030_195740_schema_academic_student_reference_selection_types::Migration),
            Box::new(m20241030_195754_schema_academic_student_reference_statuses::Migration),

            Box::new(m20241030_200500_schema_academic_survey_master_answers::Migration),
            Box::new(m20241030_200528_schema_academic_survey_master_bundle_question::Migration),
            Box::new(m20241030_200559_schema_academic_survey_master_bundles::Migration),
            Box::new(m20241030_200614_schema_academic_survey_master_questions::Migration),

            Box::new(m20241030_200649_schema_academic_survey_reference_bundle_categories::Migration),
            Box::new(m20241030_200711_schema_academic_survey_reference_question_varieties::Migration),

            Box::new(m20241030_200838_schema_academic_survey_transaction_conducts::Migration),
            Box::new(m20241030_200851_schema_academic_survey_transaction_responds::Migration),

            Box::new(m20241102_054329_schema_bulding_reference_table_categories::Migration),
            Box::new(m20241102_054343_schema_bulding_reference_table_conditions::Migration),
            Box::new(m20241102_054356_schema_bulding_reference_table_room_types::Migration),
            Box::new(m20241102_054412_schema_bulding_reference_table_varieties::Migration),

            Box::new(m20241102_054200_schema_bulding_master_table_buildings::Migration),
            Box::new(m20241102_054230_schema_bulding_master_table_rooms::Migration),

            Box::new(m20241102_064407_schema_contact_reference_table_electronic_mail_types::Migration),
            Box::new(m20241102_065934_schema_contact_reference_table_phone_types::Migration),
            Box::new(m20241102_065944_schema_contact_reference_table_residence_types::Migration),
            Box::new(m20241102_065956_schema_contact_reference_table_website_types::Migration),

            Box::new(m20241102_064053_schema_contact_master_table_electronic_mails::Migration),
            Box::new(m20241102_064106_schema_contact_master_table_phones::Migration),
            Box::new(m20241102_064249_schema_contact_master_table_residences::Migration),
            Box::new(m20241102_064301_schema_contact_master_table_websites::Migration),

            Box::new(m20250628_012704_schema_document_reference_create_archive_types::Migration),
            Box::new(m20250628_013155_schema_document_transaction_create_archives::Migration),

            Box::new(m20241102_071050_schema_feeder_akumulasi_table_estimasi::Migration),
            Box::new(m20241102_071102_schema_feeder_akumulasi_table_jumlah_data::Migration),
            Box::new(m20241102_071248_schema_feeder_akun_table_kredensial::Migration),

            Box::new(m20241102_221502_schema_feeder_master_table_aktifitas_kuliah_mahasiswa::Migration),
            Box::new(m20241102_221513_schema_feeder_master_table_aktifitas_mahasiswa::Migration),
            Box::new(m20241102_221730_schema_feeder_master_table_aktifitas_mengajar_dosen::Migration),
            Box::new(m20241102_221753_schema_feeder_master_table_anggota_aktifitas_mahasiswa::Migration),
            Box::new(m20241102_221833_schema_feeder_master_table_bidang_minat_perguruan_tinggi::Migration),
            Box::new(m20241102_221853_schema_feeder_master_table_bimbing_mahasiswa::Migration),
            Box::new(m20241102_222014_schema_feeder_master_table_biodata_dosen::Migration),
            Box::new(m20241102_222024_schema_feeder_master_table_biodata_mahasiswa::Migration),
            Box::new(m20241102_222036_schema_feeder_master_table_dosen::Migration),
            Box::new(m20241102_222052_schema_feeder_master_table_dosen_pembimbing::Migration),
            Box::new(m20241102_222109_schema_feeder_master_table_dosen_pengajar_kelas_kuliah::Migration),
            Box::new(m20241102_222124_schema_feeder_master_table_fakultas::Migration),
            Box::new(m20241102_222158_schema_feeder_master_table_hitung_transkrip_angkatan_mahasiswa::Migration),
            Box::new(m20241102_222232_schema_feeder_master_table_kartu_rencana_studi_mahasiswa::Migration),
            Box::new(m20241102_222250_schema_feeder_master_table_kelas_kuliah::Migration),
            Box::new(m20241102_222305_schema_feeder_master_table_konsistensi_data::Migration),
            Box::new(m20241102_222400_schema_feeder_master_table_konversi_kampus_merdeka::Migration),
            Box::new(m20241102_222415_schema_feeder_master_table_kurikulum::Migration),
            Box::new(m20241102_222431_schema_feeder_master_table_mahasiswa::Migration),
            Box::new(m20241102_222557_schema_feeder_master_table_mahasiswa_bimbingan_dosen::Migration),
            Box::new(m20241102_222634_schema_feeder_master_table_mahasiswa_lulus_dropout::Migration),
            Box::new(m20241102_222715_schema_feeder_master_table_matakuliah::Migration),
            Box::new(m20241102_222726_schema_feeder_master_table_matakuliah_kurikulum::Migration),
            Box::new(m20241102_222758_schema_feeder_master_table_nilai_perkuliahan_kelas::Migration),
            Box::new(m20241102_222825_schema_feeder_master_table_nilai_transfer_pendidikan_mahasiswa::Migration),
            Box::new(m20241102_222918_schema_feeder_master_table_penugasan_dosen::Migration),
            Box::new(m20241102_222933_schema_feeder_master_table_perguruan_tinggi::Migration),
            Box::new(m20241102_222950_schema_feeder_master_table_periode_aktif::Migration),
            Box::new(m20241102_223007_schema_feeder_master_table_periode_perkuliahan::Migration),
            Box::new(m20241102_223023_schema_feeder_master_table_perkuliahan_mahasiswa::Migration),
            Box::new(m20241102_223047_schema_feeder_master_table_peserta_kelas_kuliah::Migration),
            Box::new(m20241102_223157_schema_feeder_master_table_prestasi_mahasiswa::Migration),
            Box::new(m20241102_223214_schema_feeder_master_table_profil_perguruan_tinggi::Migration),
            Box::new(m20241103_090128_schema_feeder_master_table_profil_program_studi::Migration),
            Box::new(m20241103_090145_schema_feeder_master_table_profil_rencana_evaluasi::Migration),
            Box::new(m20241103_090202_schema_feeder_master_table_riwayat_fungsional_dosen::Migration),
            Box::new(m20241103_090216_schema_feeder_master_table_riwayat_nilai_mahasiswa::Migration),
            Box::new(m20241103_090230_schema_feeder_master_table_riwayat_pangkat_dosen::Migration),
            Box::new(m20241103_090243_schema_feeder_master_table_riwayat_pendidikan_dosen::Migration),
            Box::new(m20241103_090256_schema_feeder_master_table_riwayat_penelitian_dosen::Migration),
            Box::new(m20241103_090307_schema_feeder_master_table_riwayat_sertifikasi_dosen::Migration),
            Box::new(m20241103_090327_schema_feeder_master_table_skala_nilai_program_studi::Migration),
            Box::new(m20241103_090342_schema_feeder_master_table_substansi_matakuliah::Migration),
            Box::new(m20241103_090359_schema_feeder_master_table_transkrip_mahasiswa::Migration),
            Box::new(m20241103_090410_schema_feeder_master_table_uji_mahasiswa::Migration),
            Box::new(m20250904_131434_schema_feeder_master_table_komponen_evaluasi_kelas::Migration),

            Box::new(m20241103_090507_schema_feeder_referensi_table_agama::Migration),
            Box::new(m20241104_211659_schema_feeder_referensi_table_alat_transportasi::Migration),
            Box::new(m20241104_211727_schema_feeder_referensi_table_bentuk_pendidikan::Migration),
            Box::new(m20241104_211748_schema_feeder_referensi_table_ikatan_sumber_daya_manusia::Migration),
            Box::new(m20241104_211913_schema_feeder_referensi_table_jabatan_fungsional::Migration),
            Box::new(m20241104_211925_schema_feeder_referensi_table_jalur_masuk::Migration),
            Box::new(m20241104_211955_schema_feeder_referensi_table_jenis_aktifitas_mahasiswa::Migration),
            Box::new(m20241104_212021_schema_feeder_referensi_table_jenis_evaluasi::Migration),
            Box::new(m20241104_212041_schema_feeder_referensi_table_jenis_keluar::Migration),
            Box::new(m20241104_212058_schema_feeder_referensi_table_jenis_pendaftaran::Migration),
            Box::new(m20241104_212202_schema_feeder_referensi_table_jenis_prestasi::Migration),
            Box::new(m20241104_212254_schema_feeder_referensi_table_jenis_satuan_manajemen_sumberdaya::Migration),
            Box::new(m20241104_212311_schema_feeder_referensi_table_jenis_sertifikasi::Migration),
            Box::new(m20241104_212327_schema_feeder_referensi_table_jenis_substansi::Migration),
            Box::new(m20241104_212340_schema_feeder_referensi_table_jenis_tinggal::Migration),
            Box::new(m20241104_212409_schema_feeder_referensi_table_jenjang_pendidikan::Migration),
            Box::new(m20241104_212428_schema_feeder_referensi_table_kategori_kegiatan::Migration),
            Box::new(m20241104_212447_schema_feeder_referensi_table_kebutuhan_khusus::Migration),
            Box::new(m20241104_212504_schema_feeder_referensi_table_lembaga_pengangkat::Migration),
            Box::new(m20241104_212528_schema_feeder_referensi_table_level_wilayah::Migration),
            Box::new(m20241104_212550_schema_feeder_referensi_table_negara::Migration),
            Box::new(m20241104_212725_schema_feeder_referensi_table_pangkat_golongan::Migration),
            Box::new(m20241104_212807_schema_feeder_referensi_table_pekerjaan::Migration),
            Box::new(m20241104_212929_schema_feeder_referensi_table_pembiayaan::Migration),
            Box::new(m20241104_212942_schema_feeder_referensi_table_penghasilan::Migration),
            Box::new(m20241104_213001_schema_feeder_referensi_table_periode_lampau::Migration),
            Box::new(m20241104_213059_schema_feeder_referensi_table_semester::Migration),
            Box::new(m20241104_213126_schema_feeder_referensi_table_status_keaktifan_pegawai::Migration),
            Box::new(m20241104_213145_schema_feeder_referensi_table_status_kepegawaian::Migration),
            Box::new(m20241104_213240_schema_feeder_referensi_table_status_mahasiswa::Migration),
            Box::new(m20241104_213312_schema_feeder_referensi_table_tahun_ajaran::Migration),
            Box::new(m20241104_213340_schema_feeder_referensi_table_tingkat_prestasi::Migration),
            Box::new(m20241104_213355_schema_feeder_referensi_table_wilayah::Migration),
            Box::new(m20241104_214551_schema_feeder_rekapitulasi_table_indeks_prestasi_sementara_mahasiswa::Migration),
            Box::new(m20241104_214610_schema_feeder_rekapitulasi_table_jumlah_dosen::Migration),
            Box::new(m20241104_214622_schema_feeder_rekapitulasi_table_jumlah_mahasiswa::Migration),
            Box::new(m20241104_214640_schema_feeder_rekapitulasi_table_kartu_hasil_studi_mahasiswa::Migration),
            Box::new(m20241104_214655_schema_feeder_rekapitulasi_table_kartu_rencana_studi_mahasiswa::Migration),
            Box::new(m20241104_214717_schema_feeder_rekapitulasi_table_laporan::Migration),

            Box::new(m20241007_002103_schema_institution_reference_table_categories::Migration),
            Box::new(m20241007_002116_schema_institution_reference_table_position_types::Migration),
            Box::new(m20241007_002241_schema_institution_reference_table_unit_types::Migration),
            Box::new(m20241007_002256_schema_institution_reference_table_varieties::Migration),

            Box::new(m20241007_002412_schema_institution_master_table_employees::Migration),
            Box::new(m20241007_002423_schema_institution_master_table_institutions::Migration),
            Box::new(m20241007_002437_schema_institution_master_table_staffes::Migration),
            Box::new(m20241007_002447_schema_institution_master_table_units::Migration),

            Box::new(m20241007_000346_schema_literate_table_categories::Migration),
            Box::new(m20241007_000358_schema_literate_table_educations::Migration),
            Box::new(m20241007_000408_schema_literate_table_groups::Migration),
            Box::new(m20241007_000417_schema_literate_table_levels::Migration),
            Box::new(m20241007_000434_schema_literate_table_varieties::Migration),

            Box::new(m20241006_234356_schema_location_table_continents::Migration),
            Box::new(m20241006_234412_schema_location_table_countries::Migration),
            Box::new(m20241006_234423_schema_location_table_provinces::Migration),
            Box::new(m20241006_234434_schema_location_table_regencies::Migration),
            Box::new(m20241006_234445_schema_location_table_regency_types::Migration),
            Box::new(m20241006_234456_schema_location_table_regions::Migration),
            Box::new(m20241006_234506_schema_location_table_sub_districts::Migration),
            Box::new(m20241006_234516_schema_location_table_villages::Migration),

            Box::new(m20241004_232942_schema_person_reference_table_age_classifications::Migration),
            Box::new(m20241004_233159_schema_person_reference_table_blood_types::Migration),
            Box::new(m20241004_233211_schema_person_reference_table_eye_colors::Migration),
            Box::new(m20241004_233230_schema_person_reference_table_genders::Migration),
            Box::new(m20241004_233302_schema_person_reference_table_hair_colors::Migration),
            Box::new(m20241004_233308_schema_person_reference_table_hair_types::Migration),
            Box::new(m20241004_233320_schema_person_reference_table_identification_types::Migration),
            Box::new(m20241004_233506_schema_person_reference_table_incomes::Migration),
            Box::new(m20241004_233516_schema_person_reference_table_marital_statuses::Migration),
            Box::new(m20241004_233530_schema_person_reference_table_occupations::Migration),
            Box::new(m20241004_233539_schema_person_reference_table_professions::Migration),
            Box::new(m20241004_233547_schema_person_reference_table_relative_types::Migration),
            Box::new(m20241004_233555_schema_person_reference_table_religions::Migration),

            Box::new(m20241006_234052_schema_person_master_table_biodatas::Migration),
            Box::new(m20241006_234137_schema_person_master_table_family_card_members::Migration),
            Box::new(m20241006_234146_schema_person_master_table_family_cards::Migration),
            Box::new(m20241006_234205_schema_person_master_table_individuals::Migration),

            Box::new(m20250501_124920_schema_payment_midtrans_payment_create_accounts::Migration),
            Box::new(m20250521_163052_schema_payment_midtrans_payment_create_transaction_details::Migration),
            Box::new(m20250521_164437_schema_payment_midtrans_payment_create_customer_details::Migration),
            Box::new(m20250521_164556_schema_payment_midtrans_payment_create_item_details::Migration),
            Box::new(m20250521_170855_schema_payment_midtrans_payment_create_billing_addresses::Migration),
            Box::new(m20250521_170952_schema_payment_midtrans_payment_create_shipping_addresses::Migration),
        ]
    }
}
