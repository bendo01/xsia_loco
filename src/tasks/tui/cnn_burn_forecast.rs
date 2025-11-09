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

pub struct ConvolutionalNeuralNetworkBurnForecast;

#[async_trait]
impl Task for ConvolutionalNeuralNetworkBurnForecast {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "ConvolutionalNeuralNetworkBurnForecast".to_string(),
            detail: "forecast simple 1D-CNN for kelulusan tepat waktu using burn 0.19".to_string(),
        }
    }

    async fn run(&self, app_context: &AppContext, _vars: &task::Vars) -> Result<(), Error> {

    }
}