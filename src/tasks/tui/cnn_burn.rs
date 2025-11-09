// src/tasks/tui/convolutional_neural_network_burn.rs
use loco_rs::prelude::*;
use sea_orm::entity::prelude::*;
use crate::models::feeder::master::perkuliahan_mahasiswa::_entities::perkuliahan_mahasiswa as FeederMasterPerkuliahanMahasiswa;
use std::collections::HashMap;
use uuid::Uuid;

// Burn imports
use burn::module::Module;
use burn::optim::{Adam, AdamConfig};
use burn::tensor::{backend::NdArrayBackend, Tensor};
use burn::nn::{conv::Conv1dConfig, Conv1d, Linear, LinearConfig, relu};
use burn::train::{LearnerBuilder, LearnerConfig};
use burn::record::{CompactRecorder, Recorder};

type Backend = NdArrayBackend<f32>;

#[derive(Module, Debug)]
pub struct CNNModel {
    conv1: Conv1d<Backend>,
    fc1: Linear<Backend>,
}

impl CNNModel {
    pub fn new(n_features: usize, seq_len: usize, device: &burn::tensor::Device<Backend>) -> Self {
        // conv1: in_channels = n_features, out_channels = 16, kernel_size = 3
        let conv1 = Conv1dConfig::new(n_features, 16, 3).init(device);
        // output length after conv1 = seq_len - (kernel_size - 1) = seq_len - 2
        let fc_in = (seq_len - 2) * 16;
        let fc1 = LinearConfig::new(fc_in, 1).init(device);

        Self { conv1, fc1 }
    }

    pub fn forward(&self, x: Tensor<Backend, 3>) -> Tensor<Backend, 2> {
        // Expected x shape for burn conv1d: [batch, channels(in_features), seq_len]
        let x = self.conv1.forward(x);
        let x = relu(x);
        // flatten, keep batch dim (flatten dims 1..)
        let x = x.flatten(1, 2); // result shape: [batch, fc_in]
        self.fc1.forward(x)      // shape: [batch, 1]
    }
}

pub struct ConvolutionalNeuralNetworkBurn;

#[async_trait]
impl Task for ConvolutionalNeuralNetworkBurn {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "ConvolutionalNeuralNetworkBurn".to_string(),
            detail: "train simple 1D-CNN for kelulusan tepat waktu using burn".to_string(),
        }
    }

    #[allow(clippy::too_many_lines)]
    async fn run(&self, app_context: &AppContext, _vars: &task::Vars) -> Result<(), Error> {
        // Hyperparameters
        const SEQ_LEN: usize = 8; // number of semester timesteps
        const N_FEATURES: usize = 4; // ips, ipk, sks_semester, sks_total
        let batch_size_limit: usize = 256; // batch ketika membuat dataset di memori

        // 1) Query semua record mahasiswa
        let db = &app_context.db;
        let records: Vec<FeederMasterPerkuliahanMahasiswa::Model> = FeederMasterPerkuliahanMahasiswa::Entity::find()
            .all(db)
            .await
            .map_err(|e| Error::Message(format!("DB query error: {e}")))?;

        if records.is_empty() {
            println!("No records found, abort training.");
            return Ok(());
        }

        // 2) Group by id_registrasi_mahasiswa, simpan nim & nama
        #[derive(Clone)]
        struct MahasiswaInfo {
            nim: Option<String>,
            nama: Option<String>,
            records: Vec<FeederMasterPerkuliahanMahasiswa::Model>,
            label: Option<f32>, // 1.0 for on-time, 0.0 otherwise (you must compute real label)
        }

        let mut groups: HashMap<Uuid, MahasiswaInfo> = HashMap::new();
        for rec in &records {
            if let Some(id_reg) = rec.id_registrasi_mahasiswa {
                groups.entry(id_reg).and_modify(|info| {
                    info.records.push(rec.clone());
                }).or_insert_with(|| MahasiswaInfo {
                    nim: rec.nim.clone(),
                    nama: rec.nama_mahasiswa.clone(),
                    records: vec![rec.clone()],
                    label: None,
                });
            }
        }

        // 3) Sort records per student by semester id (assuming id_semester sorts lexicographically)
        for info in groups.values_mut() {
            info.records.sort_by_key(|m| m.id_semester.clone().unwrap_or_default());
        }

        // 4) BUILD DATASET: convert each student -> (input tensor, label)
        // You need a rule to create label: apakah lulus tepat waktu? -> here we assume
        // there's a column or you must compute from data. For example purpose we'll
        // set label = 1.0 when id_status_mahasiswa == "L" at expected semester OR placeholder.
        //
        // Replace label extraction with actual logic in your dataset.
        let mut xs: Vec<Tensor<Backend, 3>> = Vec::new(); // each Tensor shape [1, channels, seq_len]
        let mut ys: Vec<Tensor<Backend, 2>> = Vec::new(); // each Tensor shape [1, 1]

        for (_id_reg, info) in groups.iter_mut() {
            // Build sequence: take last SEQ_LEN records; pad left with zeros if needed
            let rec_count = info.records.len();
            let start = if rec_count > SEQ_LEN { rec_count - SEQ_LEN } else { 0 };
            let slice = &info.records[start..];

            // Prepare ndarray for shape (n_features, seq_len)
            // We'll create Array2 shape (n_features, SEQ_LEN) and then convert to Tensor [1, n_features, SEQ_LEN]
            let mut arr = ndarray::Array2::<f32>::zeros((N_FEATURES, SEQ_LEN));
            let mut col = SEQ_LEN; // fill from right to left
            for rec in slice.iter().rev() {
                col -= 1;
                // features order: channels-first: (features, timestep)
                arr[[0, col]] = rec.ips.unwrap_or(0.0);
                arr[[1, col]] = rec.ipk.unwrap_or(0.0);
                arr[[2, col]] = rec.sks_semester.unwrap_or(0.0);
                arr[[3, col]] = rec.sks_total.unwrap_or(0.0);
            }

            // If fewer records, left columns remain zero (padding done)
            // Convert Array2 -> Tensor shape [1, N_FEATURES, SEQ_LEN]
            let arr3 = arr.into_shape((1, N_FEATURES, SEQ_LEN)).map_err(|e| Error::Message(format!("ndarray reshape error: {:?}", e)))?;
            let tensor_x = Tensor::<Backend, 3>::from_ndarray(arr3);

            // Determine label: REPLACE dengan logika sebenarnya
            // Example: if any record has id_status_mahasiswa == Some("L") (lulus) -> label 1.0, else 0.0
            let mut label_val = 0.0_f32;
            if info.records.iter().any(|r| r.id_status_mahasiswa.as_deref() == Some("L")) {
                label_val = 1.0;
            }
            info.label = Some(label_val);
            let tensor_y = Tensor::<Backend, 2>::from_ndarray(ndarray::Array2::from_shape_vec((1, 1), vec![label_val]).unwrap());

            xs.push(tensor_x);
            ys.push(tensor_y);
        }

        // 5) Stack per-batch => create dataset tensors X and Y
        let n_samples = xs.len();
        println!("Prepared {} samples for training.", n_samples);
        if n_samples == 0 {
            println!("No training samples -> exit.");
            return Ok(());
        }

        // Option: make smaller dataset for demo
        let max_samples: usize = std::env::var("BURN_MAX_SAMPLES")
            .ok()
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(n_samples.min(batch_size_limit));

        // stack tensors into single tensor with shape [batch, channels, seq_len]
        // Burn's Tensor supports stacking via concatenation from ndarray behind the scenes.
        // We'll collect ndarrays and then create final ndarray and convert once.
        let mut arr_list: Vec<ndarray::Array3<f32>> = Vec::with_capacity(max_samples);
        let mut label_list: Vec<f32> = Vec::with_capacity(max_samples);

        for i in 0..max_samples {
            // extract ndarray back from Tensor using .into_ndarray() (API may vary)
            // Here, we convert via `.into_data()` alternative if needed.
            // To keep compatibility, we rebuild ndarray using to_vec and reshape
            let x_vec = xs[i].to_data().to_vec();
            // xs[i] shape: [1, N_FEATURES, SEQ_LEN]
            let ar = ndarray::Array::from_shape_vec((1, N_FEATURES, SEQ_LEN), x_vec)
                .map_err(|e| Error::Message(format!("Error build ndarray: {:?}", e)))?;
            arr_list.push(ar);
            let y_val = ys[i].to_data().to_vec()[0];
            label_list.push(y_val);
        }

        // concatenate along axis 0 (batch)
        let mut stacked_x = ndarray::concatenate(
            ndarray::Axis(0),
            &arr_list.iter().map(|a| a.view()).collect::<Vec<_>>(),
        ).map_err(|e| Error::Message(format!("ndarray concat error: {:?}", e)))?;

        // reshape stacked_x to (batch, channels, seq_len) already that shape
        // Build labels ndarray (batch, 1)
        let labels_arr = ndarray::Array2::from_shape_vec((max_samples, 1), label_list)
            .map_err(|e| Error::Message(format!("labels ndarray error: {:?}", e)))?;

        // Create Tensor for X and Y
        let x_tensor: Tensor<Backend, 3> = Tensor::from_ndarray(stacked_x);
        let y_tensor: Tensor<Backend, 2> = Tensor::from_ndarray(labels_arr);

        // 6) Define model, optimizer, learner
        let device = burn::tensor::Device::<Backend>::default();
        let model = CNNModel::new(N_FEATURES, SEQ_LEN, &device);
        let optim = AdamConfig::default().init();

        // Build learner — API: LearnerBuilder::new(name).build(model, optim) (may differ by burn version)
        let learner = LearnerBuilder::new("cnn-burn".to_string())
            .with_device("cpu".to_string())
            .build::<_, _, _>(model, optim);

        // Configure training: number of epochs, batch size
        let num_epochs: usize = std::env::var("BURN_EPOCHS").ok().and_then(|s| s.parse().ok()).unwrap_or(10);
        let train_batch_size: usize = std::env::var("BURN_BATCH_SIZE").ok().and_then(|s| s.parse().ok()).unwrap_or(16);

        println!("Starting training: epochs={}, batch_size={}", num_epochs, train_batch_size);

        // Simple fit: (note: actual Learner.fit API may accept dataset iterators)
        // We'll call a simple training routine using learner.fit (high-level), adjust if API differs.
        // If the high-level .fit(x, y) exists:
        learner.fit(x_tensor, y_tensor);

        // 7) Save model
        let recorder = CompactRecorder::default();
        // Save file name
        let model_path = std::path::Path::new("./models/cnn_model.burn");
        // Some burn versions use `save_file` on model; adjust if signature differs.
        // Example:
        if let Err(e) = learner.save_file(model_path, &recorder) {
            println!("Warning: failed to save model: {:?}", e);
        } else {
            println!("Model saved to {:?}", model_path);
        }

        println!("Training finished.");
        Ok(())
    }
}
