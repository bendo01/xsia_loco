use loco_rs::prelude::*;
use std::collections::HashMap;
use uuid::Uuid;
use anyhow::Context;

use burn::record::{BinFileRecorder, FullPrecisionSettings};
use burn::tensor::{backend::ndarray::NdArrayBackend, Tensor};

use crate::models::burn::convolutional_neural_network::perkuliahan_mahasiswa::{PerkuliahanMahasiswaCnnModel, B as BackendB};
use crate::models::feeder::master::perkuliahan_mahasiswa::_entities::perkuliahan_mahasiswa as FeederMasterPerkuliahanMahasiswa;

#[derive(Clone)]
struct MahasiswaInfo {
    nim: Option<String>,
    nama: Option<String>,
    records: Vec<FeederMasterPerkuliahanMahasiswa::Model>,
}

pub struct ConvolutionalNeuralNetworkBurnForecast;

#[async_trait]
impl Task for ConvolutionalNeuralNetworkBurnForecast {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "ConvolutionalNeuralNetworkBurnForecast".to_string(),
            detail: "forecast simple 1D-CNN for kelulusan tepat waktu using burn 0.19".to_string(),
        }
    }

    #[allow(clippy::too_many_lines)]
    async fn run(&self, app_context: &AppContext, _vars: &task::Vars) -> Result<(), Error> {
        // Configuration
        const SEQ_LEN: usize = 8;   // number of semesters
        const N_FEATURES: usize = 4; // ips, ipk, sks_semester, sks_total

        let saved_model_dir = std::env::var("SAVED_MODEL_DIR").unwrap_or_else(|_| "./saved_model".to_string());
        let model_file = format!("{}/model.bin", saved_model_dir);

        // Fetch records via SeaORM (using app_context.db, adjust accessor if different)
        let records = FeederMasterPerkuliahanMahasiswa::Entity::find()
            .all(&app_context.db)
            .await
            .map_err(|e| Error::Message(format!("DB query error: {e}")))?;

        if records.is_empty() {
            println!("Tidak ada data untuk diproses.");
            return Ok(());
        }

        // Group by id_registrasi_mahasiswa
        let mut groups: HashMap<Uuid, MahasiswaInfo> = HashMap::new();
        for rec in &records {
            if let Some(id_reg) = rec.id_registrasi_mahasiswa {
                groups.entry(id_reg)
                    .and_modify(|info| info.records.push(rec.clone()))
                    .or_insert(MahasiswaInfo {
                        nim: rec.nim.clone(),
                        nama: rec.nama_mahasiswa.clone(),
                        records: vec![rec.clone()],
                    });
            }
        }

        // Sort records per student by id_semester (assumes id_semester sorts lexicographically)
        for info in groups.values_mut() {
            info.records.sort_by_key(|m| m.id_semester.clone().unwrap_or_default());
        }

        let batch = groups.len();
        if batch == 0 {
            println!("Tidak ada data setelah pengelompokan.");
            return Ok(());
        }

        // Prepare flat data: [batch, SEQ_LEN, N_FEATURES]
        let mut data_flat: Vec<f32> = Vec::with_capacity(batch * SEQ_LEN * N_FEATURES);
        let mut mahasiswa_list: Vec<(Uuid, Option<String>, Option<String>)> = Vec::with_capacity(batch);

        for (student_id, info) in groups.iter() {
            mahasiswa_list.push((*student_id, info.nim.clone(), info.nama.clone()));

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

            // pad at the beginning if shorter than SEQ_LEN
            if seq.len() < SEQ_LEN {
                let pad = SEQ_LEN - seq.len();
                for _ in 0..pad {
                    seq.insert(0, [0.0; N_FEATURES]);
                }
            }

            // push into flat vector in order: for t in 0..SEQ_LEN { for f in 0..N_FEATURES { value } }
            for t in 0..SEQ_LEN {
                for f in 0..N_FEATURES {
                    data_flat.push(seq[t][f]);
                }
            }
        }

        // Build Burn Tensor: shape [batch, SEQ_LEN, N_FEATURES]
        // NOTE: TensorData::new expects data vector and shape
        let tensor_data = burn::tensor::TensorData::new(data_flat.clone(), [batch, SEQ_LEN, N_FEATURES]);
        let input_tensor = Tensor::<BackendB, 3>::from_data(tensor_data, &burn::backend::ndarray::NdArrayDevice::default());

        // Load model using BinFileRecorder
        // The recorded model must have been saved using the same module structure
        let recorder = BinFileRecorder::<FullPrecisionSettings>::new();
        let record = recorder.load(&model_file)
            .map_err(|e| Error::Message(format!("Gagal load model file '{}': {}", model_file, e)))?;

        // Create the model instance — the input_dim must be SEQ_LEN * N_FEATURES
        let input_dim = SEQ_LEN * N_FEATURES;
        // Hidden size — must match what you used during training & saving
        let hidden = 64usize;

        let base_model = PerkuliahanMahasiswaCnnModel::new(input_dim, hidden);
        // Load parameters from record into model instance
        let model = base_model.load_record(record)
            .map_err(|e| Error::Message(format!("Gagal load record into model: {:?}", e)))?;

        // Run forward
        let output = model.forward(input_tensor);
        // output shape expected: [batch, 1]
        // Convert to Vec<f32>
        let out_data = output.into_data();
        // Convert TensorData to Vec<f32> using to_vec method
        let out_values: Vec<f32> = out_data.to_vec::<f32>().unwrap_or_default();

        // Print results
        println!("\n=== HASIL PREDIKSI KELULUSAN TEPAT WAKTU ===");
        println!("{:<15} | {:<30} | {:<8} | {:<10}", "NIM", "Nama Mahasiswa", "Prob", "Tepat Waktu");
        println!("{:-<70}", "-");
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