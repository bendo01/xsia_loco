use burn::{
    prelude::*,
    nn::{conv::Conv1d, conv::Conv1dConfig, Linear, LinearConfig, Relu, activation::Sigmoid},
    tensor::Tensor,
};
use burn_ndarray::NdArray;

pub type B = NdArray<f32>;

#[derive(Module, Debug, Clone)]
pub struct PerkuliahanMahasiswaCnnModel {
    conv1: Conv1d<B>,
    fc1: Linear<B>,
    relu: Relu,
    sigmoid: Sigmoid,
}

impl PerkuliahanMahasiswaCnnModel {
    /// Matches exactly the training CNNModel definition
    pub fn new(n_features: usize, seq_len: usize, device: &<B as Backend>::Device) -> Self {
        let conv1 = Conv1dConfig::new(n_features, 16, 3).init(device);
        let fc_in = (seq_len - 2) * 16; // same as training (kernel_size = 3)
        let fc1 = LinearConfig::new(fc_in, 1).init(device);

        Self {
            conv1,
            fc1,
            relu: Relu::new(),
            sigmoid: Sigmoid::new(),
        }
    }

    pub fn forward(&self, x: Tensor<B, 3>) -> Tensor<B, 2> {
        let x = self.conv1.forward(x);
        let x = self.relu.forward(x);
        let x = x.flatten(1, 2);
        let x = self.fc1.forward(x);
        self.sigmoid.forward(x)
    }
}
