use loco_rs::prelude::*;
use std::collections::HashMap;
use uuid::Uuid;
use sea_orm::QueryOrder;

use burn::{
    module::Module,
    nn::{conv::Conv1dConfig, LinearConfig, Relu, activation::Sigmoid},
    optim::{AdamConfig, Optimizer, GradientsParams},
    record::CompactRecorder,
    backend::{Autodiff, ndarray::{NdArray, NdArrayDevice}},
    tensor::{Tensor, TensorData},
};
use crate::models::feeder::master::perkuliahan_mahasiswa::_entities::perkuliahan_mahasiswa as FeederMasterPerkuliahanMahasiswa;
use crate::models::feeder::master::mahasiswa_lulusan_dropout::_entities::mahasiswa_lulusan_dropout as FeederMasterMahasiswaLulusDO;

type MyBackend = Autodiff<NdArray<f32>>;

// =====================================================
//  CNN Model (same architecture as before)
// =====================================================
#[derive(Module, Debug)]
pub struct CNNModel<B: burn::tensor::backend::Backend> {
    conv1: burn::nn::conv::Conv1d<B>,
    fc1: burn::nn::Linear<B>,
    relu: Relu,
    sigmoid: Sigmoid,
}

impl<B: burn::tensor::backend::Backend> CNNModel<B> {
    pub fn new(n_features: usize, seq_len: usize, device: &B::Device) -> Self {
        let conv1 = Conv1dConfig::new(n_features, 16, 3).init(device);
        let fc_in = (seq_len - 2) * 16;
        let fc1 = LinearConfig::new(fc_in, 1).init(device);
        Self {
            conv1,
            fc1,
            relu: Relu::new(),
            sigmoid: Sigmoid::new(),
        }
    }

    pub fn forward(&self, x: burn::tensor::Tensor<B, 3>) -> burn::tensor::Tensor<B, 2> {
        let x = self.conv1.forward(x);
        let x = self.relu.forward(x);
        let x = x.flatten(1, 2);
        let x = self.fc1.forward(x);
        self.sigmoid.forward(x)
    }
}

// =====================================================
//  Task Implementation
// =====================================================
pub struct ConvolutionalNeuralNetworkBurnTraining;

#[async_trait::async_trait]
impl Task for ConvolutionalNeuralNetworkBurnTraining {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "ConvolutionalNeuralNetworkBurnTraining".to_string(),
            detail: "Train 1D CNN using pure Burn (Learner API)".to_string(),
        }
    }

    async fn run(&self, app_context: &AppContext, _vars: &task::Vars) -> Result<(), Error> {
        const SEQ_LEN: usize = 8;
        const N_FEATURES: usize = 4;

        // === Fetch records from DB ===
        let db = &app_context.db;
        let perkuliahan_records: Vec<FeederMasterPerkuliahanMahasiswa::Model> =
            FeederMasterPerkuliahanMahasiswa::Entity::find()
                .order_by_asc(FeederMasterPerkuliahanMahasiswa::Column::Nim)
                .all(db)
                .await
                .map_err(|e| Error::Message(format!("DB query error (perkuliahan): {e}")))?;

        let lulus_records: Vec<FeederMasterMahasiswaLulusDO::Model> =
            FeederMasterMahasiswaLulusDO::Entity::find()
                .all(db)
                .await
                .map_err(|e| Error::Message(format!("DB query error (lulus/do): {e}")))?;

        // Build label map
        let mut label_map: HashMap<Uuid, f32> = HashMap::new();
        for mhs in &lulus_records {
            if let Some(id_reg) = mhs.id_registrasi_mahasiswa {
                let status = mhs.nama_jenis_keluar.to_lowercase();
                let label = if status.contains("lulus") { 1.0 } else { 0.0 };
                label_map.insert(id_reg, label);
            }
        }
        
        println!("Found {} students with graduation/dropout status", label_map.len());

        // Group perkuliahan by mahasiswa
        #[derive(Clone)]
        struct MahasiswaInfo {
            records: Vec<FeederMasterPerkuliahanMahasiswa::Model>,
        }

        let mut groups: HashMap<Uuid, MahasiswaInfo> = HashMap::new();
        for rec in &perkuliahan_records {
            if let Some(id_reg) = rec.id_registrasi_mahasiswa {
                if label_map.contains_key(&id_reg) {
                    groups
                        .entry(id_reg)
                        .and_modify(|info| info.records.push(rec.clone()))
                        .or_insert(MahasiswaInfo { records: vec![rec.clone()] });
                }
            }
        }

        let mut x_vec = Vec::new();
        let mut y_vec = Vec::new();

        for (id_reg, info) in groups.iter_mut() {
            info.records.sort_by_key(|r| r.id_semester.clone().unwrap_or_default());
            let rec_count = info.records.len();
            let start = if rec_count > SEQ_LEN { rec_count - SEQ_LEN } else { 0 };
            let slice = &info.records[start..];

            let mut seq = Vec::with_capacity(SEQ_LEN * N_FEATURES);
            let pad = SEQ_LEN - slice.len();

            // Pad
            for _ in 0..pad {
                seq.extend_from_slice(&[0.0, 0.0, 0.0, 0.0]);
            }

            for rec in slice {
                seq.push(rec.ips.unwrap_or(0.0) / 4.0);
                seq.push(rec.ipk.unwrap_or(0.0) / 4.0);
                seq.push(rec.sks_semester.unwrap_or(0.0) / 24.0);
                seq.push(rec.sks_total.unwrap_or(0.0) / 160.0);
            }

            x_vec.push(seq);
            y_vec.push(*label_map.get(id_reg).unwrap_or(&0.0));
        }

        let n_samples = x_vec.len();
        let positives = y_vec.iter().filter(|&&v| v == 1.0).count();
        println!("Dataset: {} samples | Positives: {}", n_samples, positives);
        
        if n_samples == 0 {
            return Err(Error::Message(
                "No training data available. Make sure students exist in both perkuliahan_mahasiswa and mahasiswa_lulusan_dropout tables with matching id_registrasi_mahasiswa.".to_string()
            ));
        }

        // === Convert to Burn tensors ===
        let device = NdArrayDevice::default();
        
        // Flatten x_vec for tensor creation
        let x_flat: Vec<f32> = x_vec.iter().flat_map(|v| v.iter().copied()).collect();
        let x_data = TensorData::new(x_flat, [n_samples, N_FEATURES, SEQ_LEN]);
        let y_data = TensorData::new(y_vec.clone(), [n_samples, 1]);
        
        let x_tensor = Tensor::<MyBackend, 3>::from_data(x_data, &device);
        let y_tensor = Tensor::<MyBackend, 2>::from_data(y_data, &device);

        // === Model + Optimizer ===
        let mut model = CNNModel::<MyBackend>::new(N_FEATURES, SEQ_LEN, &device);
        let mut optim = AdamConfig::new().init();

        // === Manual training loop ===
        let epochs = 30;
        let lr = 0.001;

        println!("Training for {} epochs with learning rate {}...", epochs, lr);

        for epoch in 0..epochs {
            let output = model.forward(x_tensor.clone());
            let loss = output.clone().sub(y_tensor.clone()).powf_scalar(2.0).mean();

            let grads = loss.backward();
            let grads = GradientsParams::from_grads(grads, &model);
            model = optim.step(lr, model, grads);
            
            let loss_val = loss.into_data().to_vec::<f32>().unwrap()[0];
            
            if epoch % 5 == 0 || epoch == epochs - 1 {
                println!("Epoch {} | Loss = {:.6}", epoch, loss_val);
            }
        }

        // === Save model ===
        let recorder = CompactRecorder::new();
        let base = "./public/cnn_training/cnn_model_v19";
        model.save_file(base, &recorder)
            .map_err(|e| Error::Message(format!("Save model error: {e:?}")))?;
        
        std::fs::rename(format!("{base}.mpk"), format!("{base}.burn"))
            .map_err(|e| Error::Message(format!("Rename error: {e}")))?;

        println!("✅ Training complete. Model saved to {}.burn", base);

        Ok(())
    }
}
