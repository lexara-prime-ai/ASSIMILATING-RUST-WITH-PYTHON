// Notes
// Tensor - a mathematical object analogous to but more general than a vector,
// represented by an array of components that are functions of the coordinates of a space.

use burn::tensor::Tensor;
use burn::backend::WgpuBackend;

pub struct TensorBuilder;

impl TensorBuilder {
    pub fn init_builder(&self) {
        let tensor = Tensor::<WgpuBackend, 2>::from_data([[3., 4.], [5., 8.]]);
        let tensor_clone = Tensor::<WgpuBackend, 2>::ones_like(&tensor);

        println!("{}", tensor + tensor_clone);
    }
}