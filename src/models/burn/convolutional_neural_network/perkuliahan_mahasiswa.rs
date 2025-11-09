// src/models/student_cnn_model.rs
use burn::{
    prelude::*,
    nn::{
        Linear, LinearConfig,
        activation::Sigmoid,
        // For demonstration: use simple linear layers instead of 2D conv,
        // because your input shape is [batch, seq_len, n_features].
    },
    tensor::Tensor,
};
use burn_ndarray::NdArray;

pub type B = NdArray<f32>;

/// A compact MLP model suitable for sequence features (SEQ_LEN x N_FEATURES).
/// If you trained a CNN, replace this with the matching conv layers and shapes.
///
/// NOTE: Make sure this architecture exactly matches how you trained/saved the model.
#[derive(Module, Debug, Clone)]
pub struct PerkuliahanMahasiswaCnnModel {
    fc1: Linear<B>,
    fc2: Linear<B>,
    sigmoid: Sigmoid,
}

impl PerkuliahanMahasiswaCnnModel {
    /// Create model instance with same input size used in training
    pub fn new(input_dim: usize, hidden: usize, device: &<B as Backend>::Device) -> Self {
        Self {
            fc1: LinearConfig::new(input_dim, hidden).init(device),
            fc2: LinearConfig::new(hidden, 1).init(device),
            sigmoid: Sigmoid::new(),
        }
    }

    /// Forward pass
    pub fn forward(&self, input: Tensor<B, 3>) -> Tensor<B, 2> {
        let batch_size = input.shape().dims()[0];
        let seq_len = input.shape().dims()[1];
        let n_features = input.shape().dims()[2];
        let x = input.reshape([batch_size, seq_len * n_features]);
        let x = self.fc1.forward(x);
        let x = self.fc2.forward(x);
        self.sigmoid.forward(x)
    }
}
