use crate::gate::Gate;
use nalgebra::{DMatrix, DMatrixSlice, DMatrixSliceMut};
use num::{
    complex::{Complex64, ComplexFloat},
    One,
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

        let this = Self { qubits: matrix };

        for index in 0 .. this.qubits.shape().0 {
            let axiom_term = this.qubits[(index * 2, 0)].abs().powi(2)
                + this.qubits[(index * 2 + 1, 0)].abs().powi(2);
            if (axiom_term - 1.0).abs() >= 1e-30 {
                panic!(
                    "Qubit {} does not respect probability axiom, alpha: {}, \
                     beta: {}, sum of probabilities: {}",
                    index,
                    this.qubits[(index * 2, 0)],
                    this.qubits[(index * 2 + 1, 0)],
                    axiom_term
                );
            }
        }

        this
    }

    pub fn zeroed(size: usize) -> Self {
        let mut this = Self {
            qubits: DMatrix::from_element(size * 2, 1, Complex64::zero()),
        };
        for index in 0 .. size {
            this.qubits[(index * 2, 0)].set_one();
            this.qubits[(index * 2 + 1, 0)].set_zero();
        }
        this
    }

    pub fn qubits_matrix(&self) -> DMatrixSlice<Complex64> {
        self.qubits.slice((0, 0), self.qubits.shape())
    }

    pub fn qubits_matrix_mut(&mut self) -> DMatrixSliceMut<Complex64> {
        self.qubits.slice_mut((0, 0), self.qubits.shape())
    }

    pub fn predict_measure(&self, index: usize) -> bool {
        self.qubits[(index * 2 + 1, 0)].abs().powi(2) >= 0.5
    }

    pub fn measure(&mut self, index: usize) -> bool {
        let measured = self.predict_measure(index);
        if measured {
            self.qubits[(index * 2, 0)].set_zero();
            self.qubits[(index * 2 + 1, 0)].set_one();
        } else {
            self.qubits[(index * 2, 0)].set_one();
            self.qubits[(index * 2 + 1, 0)].set_zero();
        }
        measured
    }

    pub fn apply_gate<G>(&mut self, start: usize, gate: &G)
    where
        G: Gate,
    {
        gate.apply(self, start)
    }
}
