use loco_rs::prelude::*;
use crate::models::feeder::master::perkuliahan_mahasiswa::_entities::perkuliahan_mahasiswa as FeederMasterPerkuliahanMahasiswa;
use std::collections::HashMap;
use uuid::Uuid;

use burn::{
    module::Module,
    nn::{conv::Conv1dConfig, conv::Conv1d, Linear, LinearConfig, Relu},
    optim::{AdamConfig, Optimizer, GradientsParams},
    record::CompactRecorder,
    tensor::{backend::Backend, Tensor},
};
use burn::backend::{ndarray::{NdArray, NdArrayDevice}, Autodiff};
use burn::tensor::TensorData;

type MyBackend = Autodiff<NdArray<f32>>;

#[derive(Module, Debug)]
pub struct CNNModel<B: Backend> {
    conv1: Conv1d<B>,
    fc1: Linear<B>,
}

impl<B: Backend> CNNModel<B> {
    pub fn new(n_features: usize, seq_len: usize, device: &B::Device) -> Self {
        let conv1 = Conv1dConfig::new(n_features, 16, 3).init(device);
        let fc_in = (seq_len - 2) * 16;
        let fc1 = LinearConfig::new(fc_in, 1).init(device);
        Self { conv1, fc1 }
    }

    pub fn forward(&self, x: Tensor<B, 3>) -> Tensor<B, 2> {
        let x = self.conv1.forward(x);
        let x = Relu::new().forward(x);
        let x = x.flatten(1, 2);
        self.fc1.forward(x)
    }
}

pub struct ConvolutionalNeuralNetworkBurnTraining;

#[async_trait]
impl Task for ConvolutionalNeuralNetworkBurnTraining {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "ConvolutionalNeuralNetworkBurnTraining".to_string(),
            detail: "train simple 1D-CNN for kelulusan tepat waktu using burn 0.19".to_string(),
        }
    }

    async fn run(&self, app_context: &AppContext, _vars: &task::Vars) -> Result<(), Error> {
        const SEQ_LEN: usize = 8;
        const N_FEATURES: usize = 4;

        let db = &app_context.db;
        let records: Vec<FeederMasterPerkuliahanMahasiswa::Model> = FeederMasterPerkuliahanMahasiswa::Entity::find()
            .all(db)
            .await
            .map_err(|e| Error::Message(format!("DB query error: {e}")))?;

        if records.is_empty() {
            println!("No records found.");
            return Ok(());
        }

        #[derive(Clone)]
        struct MahasiswaInfo {
            records: Vec<FeederMasterPerkuliahanMahasiswa::Model>,
        }

        let mut groups: HashMap<Uuid, MahasiswaInfo> = HashMap::new();
        for rec in &records {
            if let Some(id_reg) = rec.id_registrasi_mahasiswa {
                groups
                    .entry(id_reg)
                    .and_modify(|info| info.records.push(rec.clone()))
                    .or_insert_with(|| MahasiswaInfo {
                        records: vec![rec.clone()],
                    });
            }
        }

        // Sort & build tensors
        let mut x_data = Vec::new();
        let mut y_data = Vec::new();

        for (_, info) in groups.iter_mut() {
            info.records.sort_by_key(|r| r.id_semester.clone().unwrap_or_default());
            let rec_count = info.records.len();
            let start = if rec_count > SEQ_LEN { rec_count - SEQ_LEN } else { 0 };
            let slice = &info.records[start..];

            let mut arr = ndarray::Array2::<f32>::zeros((N_FEATURES, SEQ_LEN));
            let mut col = SEQ_LEN;
            for rec in slice.iter().rev() {
                col -= 1;
                arr[[0, col]] = rec.ips.unwrap_or(0.0);
                arr[[1, col]] = rec.ipk.unwrap_or(0.0);
                arr[[2, col]] = rec.sks_semester.unwrap_or(0.0);
                arr[[3, col]] = rec.sks_total.unwrap_or(0.0);
            }

            x_data.push(arr);

            // Label dummy: lulus = 1.0 jika status "L", else 0.0
            let label = if info.records.iter().any(|r| r.id_status_mahasiswa.as_deref() == Some("L")) {
                1.0
            } else {
                0.0
            };
            y_data.push(label);
        }

        let n_samples = x_data.len();
        println!("Loaded {} samples.", n_samples);

        let x_stack = ndarray::Array3::from_shape_vec(
            (n_samples, N_FEATURES, SEQ_LEN),
            x_data.iter().flat_map(|a| a.iter().copied()).collect(),
        )
        .unwrap();

        let _y_stack = ndarray::Array2::from_shape_vec(
            (n_samples, 1),
            y_data.clone(),
        )
        .unwrap();

        let device = NdArrayDevice::default();
        let mut model = CNNModel::<MyBackend>::new(N_FEATURES, SEQ_LEN, &device);
        let mut optim = AdamConfig::new().init();

        // Convert ndarray to burn tensor data
        let x_vec: Vec<f32> = x_stack.iter().copied().collect();
        let x_data = TensorData::new(x_vec, [n_samples, N_FEATURES, SEQ_LEN]);
        let y_data = TensorData::new(y_data.clone(), [n_samples, 1]);

        let x_tensor = Tensor::<MyBackend, 3>::from_data(x_data, &device);
        let y_tensor = Tensor::<MyBackend, 2>::from_data(y_data, &device);

        // Simple manual training loop
        let epochs = 5;
        let lr = 0.001;

        println!("Training for {epochs} epochs ...");

        for epoch in 0..epochs {
            let output = model.forward(x_tensor.clone());
            let loss = output.clone().sub(y_tensor.clone()).powf_scalar(2.0).mean();

            let grads = loss.backward();
            let grads = GradientsParams::from_grads(grads, &model);
            model = optim.step(lr, model, grads);
            
            let loss_val = loss.into_data().to_vec::<f32>().unwrap()[0];
            println!("Epoch {epoch} | Loss = {:.6}", loss_val);
        }

        // Save model
        let recorder = CompactRecorder::new();
        model
            .save_file("./public/cnn_training/cnn_model_v19.burn", &recorder)
            .map_err(|e| Error::Message(format!("Save model error: {e:?}")))?;

        println!("✅ Training complete. Model saved to ./public/cnn_training/cnn_model_v19.burn");

        Ok(())
    }
}
