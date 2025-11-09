use loco_rs::prelude::*;
use std::collections::HashMap;
use uuid::Uuid;
use sea_orm::QueryOrder;

use burn::prelude::*;
use burn::record::CompactRecorder;
use burn::tensor::Tensor;
use burn_ndarray::NdArrayDevice;

use crate::models::burn::convolutional_neural_network::perkuliahan_mahasiswa::{
    PerkuliahanMahasiswaCnnModel, B as BackendB,
};
use crate::models::feeder::master::perkuliahan_mahasiswa::_entities::perkuliahan_mahasiswa as FeederMasterPerkuliahanMahasiswa;
use crate::models::feeder::master::mahasiswa_lulusan_dropout::_entities::mahasiswa_lulusan_dropout as FeederMasterMahasiswaLulusDO;

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
            detail: "Forecast kelulusan tepat waktu using Burn CNN + DB labels".to_string(),
        }
    }

    async fn run(&self, app_context: &AppContext, _vars: &task::Vars) -> Result<(), Error> {
        const SEQ_LEN: usize = 8;
        const N_FEATURES: usize = 4;

        let db = &app_context.db;
        let saved_model_dir =
            std::env::var("SAVED_MODEL_DIR").unwrap_or_else(|_| "./public/cnn_training".to_string());
        let model_base_path = format!("{}/cnn_model_v19", saved_model_dir);

        // === Fetch data from both tables ===
        let perkuliahan_records: Vec<FeederMasterPerkuliahanMahasiswa::Model> =
            FeederMasterPerkuliahanMahasiswa::Entity::find()
                .order_by_asc(FeederMasterPerkuliahanMahasiswa::Column::Nim)
                .all(db)
                .await
                .map_err(|e| Error::Message(format!("DB query error (perkuliahan): {e}")))?;

        let lulus_records: Vec<FeederMasterMahasiswaLulusDO::Model> =
            FeederMasterMahasiswaLulusDO::Entity::find()
                .order_by_asc(FeederMasterMahasiswaLulusDO::Column::Nim)
                .all(db)
                .await
                .map_err(|e| Error::Message(format!("DB query error (lulus/do): {e}")))?;

        // === Build label map like in training ===
        let mut label_map: HashMap<Uuid, f32> = HashMap::new();
        for mhs in &lulus_records {
            if let Some(id_reg) = mhs.id_registrasi_mahasiswa {
                let status = mhs.nama_jenis_keluar.to_lowercase();
                let label = if status.contains("lulus") { 1.0 } else { 0.0 };
                label_map.insert(id_reg, label);
            }
        }

        if label_map.is_empty() {
            println!("❌ Tidak ditemukan label di tabel mahasiswa_lulusan_dropout.");
            return Ok(());
        }

        // === Group perkuliahan by mahasiswa (only those in label_map) ===
        let mut groups: HashMap<Uuid, MahasiswaInfo> = HashMap::new();
        for rec in &perkuliahan_records {
            if let Some(id_reg) = rec.id_registrasi_mahasiswa {
                if label_map.contains_key(&id_reg) {
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
        }

        let batch = groups.len();
        if batch == 0 {
            println!("❌ Tidak ada mahasiswa yang cocok antara kedua tabel.");
            return Ok(());
        }

        // === Prepare tensors ===
        let mut data_flat: Vec<f32> = Vec::with_capacity(batch * SEQ_LEN * N_FEATURES);
        let mut mahasiswa_list: Vec<(Uuid, Option<String>, Option<String>, f32)> =
            Vec::with_capacity(batch);

        for (student_id, info) in groups.iter() {
            mahasiswa_list.push((
                *student_id,
                info.nim.clone(),
                info.nama.clone(),
                *label_map.get(student_id).unwrap_or(&0.0),
            ));

            let rec_count = info.records.len();
            let start = if rec_count > SEQ_LEN { rec_count - SEQ_LEN } else { 0 };
            let slice = &info.records[start..];

            let mut seq: Vec<[f32; N_FEATURES]> = Vec::with_capacity(SEQ_LEN);
            for rec in slice {
                let ips = rec.ips.unwrap_or(0.0) / 4.0;
                let ipk = rec.ipk.unwrap_or(0.0) / 4.0;
                let sks_smt = rec.sks_semester.unwrap_or(0.0) / 24.0;
                let sks_tot = rec.sks_total.unwrap_or(0.0) / 160.0;
                seq.push([ips, ipk, sks_smt, sks_tot]);
            }

            if seq.len() < SEQ_LEN {
                let pad = SEQ_LEN - seq.len();
                for _ in 0..pad {
                    seq.insert(0, [0.0; N_FEATURES]);
                }
            }

            for t in 0..SEQ_LEN {
                for f in 0..N_FEATURES {
                    data_flat.push(seq[t][f]);
                }
            }
        }

        // === Build input tensor ===
        let device = NdArrayDevice::default();
        let tensor_data = burn::tensor::TensorData::new(data_flat, [batch, SEQ_LEN, N_FEATURES]);
        let mut input_tensor = Tensor::<BackendB, 3>::from_data(tensor_data, &device);
        input_tensor = input_tensor.permute([0, 2, 1]); // [batch, N_FEATURES, SEQ_LEN]

        // === Load model ===
        // CompactRecorder expects .mpk extension but we have .burn
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
            &device,
        )
        .map_err(|e| Error::Message(format!("Gagal load model file '{model_base_path}': {e}")))?;
        
        // Clean up symlink if we created it
        if std::path::Path::new(&mpk_path).is_symlink() {
            let _ = std::fs::remove_file(&mpk_path);
        }

        // === Run inference ===
        let output = model.forward(input_tensor);
        let out_values = output.into_data().as_slice().unwrap_or(&[]).to_vec();

        // === Evaluate ===
        let mut tp = 0;
        let mut tn = 0;
        let mut fp = 0;
        let mut fn_ = 0;

        let mut results: Vec<(Option<String>, Option<String>, f32, f32, f32)> = mahasiswa_list
            .iter()
            .enumerate()
            .map(|(i, (_id, nim, nama, label))| {
                let prob = out_values.get(i).copied().unwrap_or(0.0);
                let pred_label = if prob >= 0.5 { 1.0 } else { 0.0 };
                match (pred_label as i32, *label as i32) {
                    (1, 1) => tp += 1,
                    (0, 0) => tn += 1,
                    (1, 0) => fp += 1,
                    (0, 1) => fn_ += 1,
                    _ => {}
                }
                (nim.clone(), nama.clone(), prob, pred_label, *label)
            })
            .collect();

        results.sort_by(|a, b| a.0.as_deref().unwrap_or("").cmp(b.0.as_deref().unwrap_or("")));

        println!("\n=== HASIL PREDIKSI KELULUSAN ===");
        println!("{:<15} | {:<30} | {:<8} | {:<10} | {:<6}",
            "NIM", "Nama Mahasiswa", "Prob", "Prediksi", "Asli");
        println!("{:-<80}", "-");
        for (nim, nama, prob, pred, label) in results {
            println!("{:<15} | {:<30} | {:<8.4} | {:<10} | {:<6}",
                nim.unwrap_or_else(|| "-".to_string()),
                nama.unwrap_or_else(|| "-".to_string()),
                prob,
                if pred == 1.0 { "✅ Lulus" } else { "❌ Tidak" },
                if label == 1.0 { "1" } else { "0" });
        }

        // === Metrics ===
        let total = tp + tn + fp + fn_;
        let accuracy = (tp + tn) as f32 / total as f32;
        let precision = if tp + fp > 0 { tp as f32 / (tp + fp) as f32 } else { 0.0 };
        let recall = if tp + fn_ > 0 { tp as f32 / (tp + fn_) as f32 } else { 0.0 };
        let f1 = if precision + recall > 0.0 {
            2.0 * (precision * recall) / (precision + recall)
        } else {
            0.0
        };

        println!("\n=== METRIK EVALUASI ===");
        println!("Total data: {}", total);
        println!("True Positive = {tp}, True Negative = {tn}, False Positive = {fp}, False Negative = {fn_}");
        println!("Accuracy : {:.4}", accuracy);
        println!("Precision: {:.4}", precision);
        println!("Recall   : {:.4}", recall);
        println!("F1 Score : {:.4}", f1);

        Ok(())
    }
}
