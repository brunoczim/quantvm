use crate::memory::Memory;
use nalgebra::{DMatrix, SMatrix};
use num::{complex::Complex64, Integer, One, Zero};

pub trait Gate {
    fn apply(&self, memory: &mut Memory, start: usize);
}

#[derive(Debug)]
pub struct SGate<const N: usize> {
    elements: SMatrix<Complex64, N, N>,
}

impl<const N: usize> SGate<N> {
    pub fn from_matrix(elements: SMatrix<Complex64, N, N>) -> Self {
        if N.is_odd() {
            panic!("SGate must be have an even dimension, found {}", N);
        }
        Self { elements }
    }

    pub fn identity() -> Self {
        let mut matrix = SMatrix::from_element(Complex64::zero());

        for index in 0 .. N {
            matrix[(index, index)] = Complex64::one();
        }

        Self::from_matrix(matrix)
    }
}

impl<const N: usize> Gate for SGate<N> {
    fn apply(&self, memory: &mut Memory, start: usize) {
        let mut qubits_matrix = memory.qubits_matrix_mut();
        let mut slice = qubits_matrix.slice_mut((start, 0), (N, 2));
        let mut output: SMatrix<Complex64, N, N> =
            SMatrix::from_element(Complex64::zero());
        self.elements.mul_to(&slice, &mut output);
        slice.copy_from(&output);
    }
}

#[derive(Debug)]
pub struct DGate {
    elements: DMatrix<Complex64>,
}

impl DGate {
    pub fn from_matrix(elements: DMatrix<Complex64>) -> Self {
        if !elements.is_square() {
            panic!(
                "Gate of dimensions {}x{} is not square matrix",
                elements.shape().0,
                elements.shape().1
            );
        }
        if elements.shape().0.is_odd() {
            panic!(
                "DGate must be have an even dimension, found {}",
                elements.shape().0
            );
        }
        Self { elements }
    }

    pub fn identity(size: usize) -> Self {
        let mut matrix =
            DMatrix::from_element(size * 2, size * 2, Complex64::zero());

        for index in 0 .. size {
            matrix[(index, index)] = Complex64::one();
        }

        Self::from_matrix(matrix)
    }
}

impl Gate for DGate {
    fn apply(&self, memory: &mut Memory, start: usize) {
        let mut qubits_matrix = memory.qubits_matrix_mut();
        let mut slice =
            qubits_matrix.slice_mut((start, 0), (self.elements.shape().0, 2));
        let mut output = DMatrix::from_element(
            self.elements.shape().0,
            self.elements.shape().1,
            Complex64::zero(),
        );
        self.elements.mul_to(&slice, &mut output);
        slice.copy_from(&output);
    }
}
