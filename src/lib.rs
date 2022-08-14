use nalgebra::{DMatrix, SMatrix};
use num::{complex::Complex64, Zero};

#[derive(Debug)]
pub struct Memory {
    qubits: DMatrix<Complex64>,
}

impl Memory {
    pub fn new(size: usize) -> Self {
        Self { qubits: DMatrix::from_element(size * 2, 1, Complex64::zero()) }
    }

    pub fn apply_gate<G>(&mut self, start: usize, gate: &G)
    where
        G: Gate,
    {
        gate.apply(self, start)
    }
}

pub trait Gate {
    fn apply(&self, memory: &mut Memory, start: usize);
}

pub type SquareMatrix<const N: usize> = SMatrix<Complex64, N, N>;

#[derive(Debug)]
pub struct SGate<const N: usize> {
    elements: SquareMatrix<N>,
}

impl<const N: usize> SGate<N> {
    pub fn new(elements: SquareMatrix<N>) -> Self {
        Self { elements }
    }
}

impl<const N: usize> Gate for SGate<N> {
    fn apply(&self, memory: &mut Memory, start: usize) {
        let mut slice = memory.qubits.slice_mut((start, 0), (N, 2));
        let mut output = SquareMatrix::from_element(Complex64::zero());
        self.elements.mul_to(&slice, &mut output);
        slice.copy_from(&output);
    }
}

#[derive(Debug)]
pub struct DGate {
    elements: DMatrix<Complex64>,
}

impl DGate {
    pub fn new(elements: DMatrix<Complex64>) -> Self {
        if !elements.is_square() {
            panic!(
                "Gate of dimensions {}x{} is not square matrix",
                elements.shape().0,
                elements.shape().1
            );
        }
        Self { elements }
    }
}

impl Gate for DGate {
    fn apply(&self, memory: &mut Memory, start: usize) {
        let mut slice =
            memory.qubits.slice_mut((start, 0), (self.elements.shape().0, 2));
        let mut output = DMatrix::from_element(
            self.elements.shape().0,
            self.elements.shape().1,
            Complex64::zero(),
        );
        self.elements.mul_to(&slice, &mut output);
        slice.copy_from(&output);
    }
}
