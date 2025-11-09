use loco_rs::prelude::*;
// use sea_orm::entity::prelude::*;
use tensorflow::{Graph, Session, SessionOptions, Tensor, SavedModelBundle};
use std::collections::HashMap;
use crate::models::feeder::master::perkuliahan_mahasiswa::_entities::perkuliahan_mahasiswa as FeederMasterPerkuliahanMahasiswa;

#[derive(Clone)]
struct MahasiswaInfo {
    nim: Option<String>,
    nama: Option<String>,
    records: Vec<FeederMasterPerkuliahanMahasiswa::Model>,
}

pub struct ConvolutionalNeuralNetwork;

#[async_trait]
impl Task for ConvolutionalNeuralNetwork {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "ConvolutionalNeuralNetwork".to_string(),
            detail: "test tensor flow using cnn algorithm".to_string(),
        }
    }

    #[allow(clippy::too_many_lines)]
    async fn run(&self, app_context: &AppContext, _vars: &task::Vars) -> Result<(), Error> {
        // Parameter konfigurasi
        const SEQ_LEN: usize = 8; // jumlah semester
        const N_FEATURES: usize = 4; // ips, ipk, sks_semester, sks_total

        let saved_model_dir = std::env::var("SAVED_MODEL_DIR").unwrap_or_else(|_| "./saved_model".to_string());
        let tf_input_op_name = std::env::var("TF_INPUT_OP_NAME").unwrap_or_else(|_| "serving_default_input".to_string());
        let tf_output_op_name = std::env::var("TF_OUTPUT_OP_NAME").unwrap_or_else(|_| "StatefulPartitionedCall".to_string());

        // Ambil koneksi database dari AppContext
        // let db: &DatabaseConnection = app_context.get::<DatabaseConnection>().ok_or_else(|| Error::Message("DB connection not found".into()))?;

        // Ambil data semua mahasiswa
        let records = FeederMasterPerkuliahanMahasiswa::Entity::find()
            .all(&app_context.db)
            .await
            .map_err(|e| Error::Message(format!("DB query error: {e}")))?;

        // Group berdasarkan id_registrasi_mahasiswa
        let mut groups: HashMap<Uuid, MahasiswaInfo> = HashMap::new();
        for rec in &records {
            if let Some(id_reg) = rec.id_registrasi_mahasiswa {
                groups
                    .entry(id_reg)
                    .and_modify(|info| info.records.push(rec.clone()))
                    .or_insert(MahasiswaInfo {
                        nim: rec.nim.clone(),
                        nama: rec.nama_mahasiswa.clone(),
                        records: vec![rec],
                    });
            }
        }

        // Sortir per mahasiswa berdasarkan id_semester
        for info in groups.values_mut() {
            info.records.sort_by_key(|m| m.id_semester.clone().unwrap_or_default());
        }

        let batch = groups.len();
        if batch == 0 {
            println!("Tidak ada data untuk diproses.");
            return Ok(());
        }

        let mut data_flat: Vec<f32> = Vec::with_capacity(batch * SEQ_LEN * N_FEATURES);

        let mut mahasiswa_list: Vec<(Uuid, Option<String>, Option<String>)> = Vec::with_capacity(batch);

        for (student_id, info) in groups.iter() {
            mahasiswa_list.push((*student_id, info.nim.clone(), info.nama.clone()));

            // Ambil data maksimal SEQ_LEN semester terakhir
            let rec_count = info.records.len();
            let start = if rec_count > SEQ_LEN { rec_count - SEQ_LEN } else { 0 };
            let slice = &info.records[start..];

            let mut seq: Vec<[f32; N_FEATURES]> = Vec::with_capacity(SEQ_LEN);
            for rec in slice.iter() {
                let ips = rec.ips.unwrap_or(0.0);
                let ipk = rec.ipk.unwrap_or(0.0);
                let sks_smt = rec.sks_semester.unwrap_or(0.0);
                let sks_tot = rec.sks_total.unwrap_or(0.0);
                seq.push([ips, ipk, sks_smt, sks_tot]);
            }

            // Padding
            if seq.len() < SEQ_LEN {
                let pad = SEQ_LEN - seq.len();
                for _ in 0..pad {
                    seq.insert(0, [0.0; N_FEATURES]);
                }
            }

            // Masukkan ke dalam flat vector
            for t in 0..SEQ_LEN {
                for f in 0..N_FEATURES {
                    data_flat.push(seq[t][f]);
                }
            }
        }

        // Buat tensor input
        let dims = [batch as u64, SEQ_LEN as u64, N_FEATURES as u64];
        let input_tensor = Tensor::new(&dims)
            .with_values(&data_flat)
            .map_err(|e| Error::Message(format!("Tensor error: {e}")))?;

        // Load model TensorFlow
        let mut graph = Graph::new();
        let bundle = SavedModelBundle::load(&SessionOptions::new(), &["serve"], &mut graph, &saved_model_dir)
            .map_err(|e| Error::Message(format!("Gagal load model TF: {e}")))?;
        let session: Session = bundle.session;

        let input_op = graph.operation_by_name_required(&tf_input_op_name)
            .map_err(|e| Error::Message(format!("Input op '{tf_input_op_name}' tidak ditemukan: {e}")))?;
        let output_op = graph.operation_by_name_required(&tf_output_op_name)
            .map_err(|e| Error::Message(format!("Output op '{tf_output_op_name}' tidak ditemukan: {e}")))?;

        // Jalankan inference
        let mut run_args = tensorflow::SessionRunArgs::new();
        run_args.add_feed(&input_op, 0, &input_tensor);
        let output_token = run_args.request_fetch(&output_op, 0);

        session.run(&mut run_args)
            .map_err(|e| Error::Message(format!("Gagal menjalankan sesi TF: {e}")))?;
        let output: Tensor<f32> = run_args.fetch(output_token)
            .map_err(|e| Error::Message(format!("Gagal ambil hasil TF: {e}")))?;

        // Interpretasi hasil
        println!("\n=== HASIL PREDIKSI KELULUSAN TEPAT WAKTU ===");
        println!("{:<15} | {:<30} | {:<8} | {:<10}", "NIM", "Nama Mahasiswa", "Prob", "Tepat Waktu");
        println!("{:-<70}", "-");
        let out_values = output.to_vec();
        for (i, (_id_reg, nim, nama)) in mahasiswa_list.iter().enumerate() {
            let prob = out_values.get(i).copied().unwrap_or(0.0);
            let predicted_on_time = prob >= 0.5;
            println!(
                "{:<15} | {:<30} | {:<8.4} | {}",
                nim.clone().unwrap_or_else(|| "-".to_string()),
                nama.clone().unwrap_or_else(|| "-".to_string()),
                prob,
                if predicted_on_time { "✅ Ya" } else { "❌ Tidak" }
            );
        }

        Ok(())
    }
}