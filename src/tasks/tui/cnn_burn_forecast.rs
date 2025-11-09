use loco_rs::prelude::*;
use std::collections::HashMap;
use uuid::Uuid;

use burn::prelude::*;
use burn::record::CompactRecorder;
use burn::tensor::Tensor;
use burn_ndarray::NdArrayDevice;

use crate::models::burn::convolutional_neural_network::perkuliahan_mahasiswa::{
    PerkuliahanMahasiswaCnnModel, B as BackendB,
};
use crate::models::feeder::master::perkuliahan_mahasiswa::_entities::perkuliahan_mahasiswa as FeederMasterPerkuliahanMahasiswa;

#[derive(Clone)]
struct MahasiswaInfo {
    nim: Option<String>,
    nama: Option<String>,
    records: Vec<FeederMasterPerkuliahanMahasiswa::Model>,
}

pub struct ConvolutionalNeuralNetworkBurnForecast;

#[async_trait::async_trait]
impl Task for ConvolutionalNeuralNetworkBurnForecast {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "ConvolutionalNeuralNetworkBurnForecast".to_string(),
            detail: "Forecast kelulusan tepat waktu using Burn 0.19 CNN".to_string(),
        }
    }

    #[allow(clippy::too_many_lines)]
    async fn run(&self, app_context: &AppContext, _vars: &task::Vars) -> Result<(), Error> {
        // Configuration constants
        const SEQ_LEN: usize = 8;
        const N_FEATURES: usize = 4;

        let saved_model_dir =
            std::env::var("SAVED_MODEL_DIR").unwrap_or_else(|_| "./public/cnn_training".to_string());

        // Query database records
        let records = FeederMasterPerkuliahanMahasiswa::Entity::find()
            .all(&app_context.db)
            .await
            .map_err(|e| Error::Message(format!("DB query error: {e}")))?;

        if records.is_empty() {
            println!("Tidak ada data untuk diproses.");
            return Ok(());
        }

        // Group by mahasiswa
        let mut groups: HashMap<Uuid, MahasiswaInfo> = HashMap::new();
        for rec in &records {
            if let Some(id_reg) = rec.id_registrasi_mahasiswa {
                groups
                    .entry(id_reg)
                    .and_modify(|info| info.records.push(rec.clone()))
                    .or_insert(MahasiswaInfo {
                        nim: rec.nim.clone(),
                        nama: rec.nama_mahasiswa.clone(),
                        records: vec![rec.clone()],
                    });
            }
        }

        // Sort semester order
        for info in groups.values_mut() {
            info.records
                .sort_by_key(|r| r.id_semester.clone().unwrap_or_default());
        }

        let batch = groups.len();
        if batch == 0 {
            println!("Tidak ada data mahasiswa setelah pengelompokan.");
            return Ok(());
        }

        let mut data_flat: Vec<f32> = Vec::with_capacity(batch * SEQ_LEN * N_FEATURES);
        let mut mahasiswa_list: Vec<(Uuid, Option<String>, Option<String>)> =
            Vec::with_capacity(batch);

        for (student_id, info) in groups.iter() {
            mahasiswa_list.push((*student_id, info.nim.clone(), info.nama.clone()));

            let rec_count = info.records.len();
            let start = if rec_count > SEQ_LEN { rec_count - SEQ_LEN } else { 0 };
            let slice = &info.records[start..];

            let mut seq: Vec<[f32; N_FEATURES]> = Vec::with_capacity(SEQ_LEN);
            for rec in slice.iter() {
                seq.push([
                    rec.ips.unwrap_or(0.0),
                    rec.ipk.unwrap_or(0.0),
                    rec.sks_semester.unwrap_or(0.0),
                    rec.sks_total.unwrap_or(0.0),
                ]);
            }

            // Pad from start
            if seq.len() < SEQ_LEN {
                let pad = SEQ_LEN - seq.len();
                for _ in 0..pad {
                    seq.insert(0, [0.0; N_FEATURES]);
                }
            }

            // Flatten
            for t in 0..SEQ_LEN {
                for f in 0..N_FEATURES {
                    data_flat.push(seq[t][f]);
                }
            }
        }

        // Build Burn tensor input: [batch, SEQ_LEN, N_FEATURES]
        let device = NdArrayDevice::default();
        let tensor_data = burn::tensor::TensorData::new(data_flat, [batch, SEQ_LEN, N_FEATURES]);
        let mut input_tensor = Tensor::<BackendB, 3>::from_data(tensor_data, &device);

        // Permute to [batch, N_FEATURES, SEQ_LEN] for Conv1D
        input_tensor = input_tensor.permute([0, 2, 1]);

        // CompactRecorder expects .mpk extension but we have .burn
        // We need to temporarily rename or use a symbolic link approach
        // For now, let's just specify the path without extension and let recorder add .burn
        let model_base_path = format!("{}/cnn_model_v19", saved_model_dir);
        
        // Check if .burn file exists, if so use it directly
        let burn_path = format!("{}.burn", model_base_path);
        let mpk_path = format!("{}.mpk", model_base_path);
        
        // If .burn exists but .mpk doesn't, create a temporary symlink
        if std::path::Path::new(&burn_path).exists() && !std::path::Path::new(&mpk_path).exists() {
            std::os::unix::fs::symlink(&burn_path, &mpk_path)
                .map_err(|e| Error::Message(format!("Failed to create symlink: {e}")))?;
        }
        
        let recorder = CompactRecorder::new();
        let model = <PerkuliahanMahasiswaCnnModel as Module<BackendB>>::load_file::<CompactRecorder, _>(
            PerkuliahanMahasiswaCnnModel::new(N_FEATURES, SEQ_LEN, &device),
            model_base_path.clone(),
            &recorder,
            &device
        )
        .map_err(|e| Error::Message(format!("Gagal load model file '{model_base_path}': {e}")))?;
        
        // Clean up symlink if we created it
        if std::path::Path::new(&mpk_path).is_symlink() {
            let _ = std::fs::remove_file(&mpk_path);
        }

        // Run inference
        let output = model.forward(input_tensor);
        let out_data = output.into_data();
        let out_values = out_data.as_slice().unwrap_or(&[]).to_vec();

        // Output results
        println!("\n=== HASIL PREDIKSI KELULUSAN TEPAT WAKTU ===");
        println!(
            "{:<15} | {:<30} | {:<8} | {:<10}",
            "NIM", "Nama Mahasiswa", "Prob", "Tepat Waktu"
        );
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
