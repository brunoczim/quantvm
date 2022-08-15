use crate::gate::Gate;
use nalgebra::{DMatrix, DMatrixSlice, DMatrixSliceMut};
use num::{
    complex::{Complex64, ComplexFloat},
    Zero,
};

#[derive(Debug)]
pub struct Memory {
    qubits: DMatrix<Complex64>,
}

impl Memory {
    pub fn from_matrix(matrix: DMatrix<Complex64>) -> Self {
        if matrix.shape().1 != 1 {
            panic!(
                "Shape of qubit matrix must be a column vector, shape {}x{} \
                 found",
                matrix.shape().0,
                matrix.shape().1
            )
        }

        Self { qubits: matrix }
    }

    pub fn zeroed(size: usize) -> Self {
        Self { qubits: DMatrix::from_element(size * 2, 1, Complex64::zero()) }
    }

    pub fn qubits_matrix(&self) -> DMatrixSlice<Complex64> {
        self.qubits.slice((0, 0), self.qubits.shape())
    }

    pub fn qubits_matrix_mut(&mut self) -> DMatrixSliceMut<Complex64> {
        self.qubits.slice_mut((0, 0), self.qubits.shape())
    }

    pub fn measure(&self, index: usize) -> bool {
        self.qubits[index * 2 + 1].abs().powi(2) >= 0.5
    }

    pub fn apply_gate<G>(&mut self, start: usize, gate: &G)
    where
        G: Gate,
    {
        gate.apply(self, start)
    }
}
