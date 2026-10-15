// This code is part of Qiskit.
//
// (C) Copyright IBM 2026
//
// This code is licensed under the Apache License, Version 2.0. You may
// obtain a copy of this license in the LICENSE.txt file in the root directory
// of this source tree or at https://www.apache.org/licenses/LICENSE-2.0.
//
// Any modifications or derivative works of this code must retain this
// copyright notice, and modified files need to carry a notice indicating
// that they have been altered from the originals.

//! This module contains operations that are used in Pauli-Based Computations.

use std::fmt;

///  Basic Pauli enum
///
/// TODO: Decide whether we want to use existing from Qiskit or define one specifically for MIR.
#[derive(Debug, Copy, Clone, Default)]
pub enum Pauli {
    #[default]
    I,
    X,
    Z,
    Y,
}

impl fmt::Display for Pauli {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Debug, Clone)]
pub struct PauliList(Vec<Pauli>);

/// Initial type for Angles
///
/// TODO: Decide whether we want to use existing from Qiskit or define one here
/// This is likely overkill for what we need (in terms of precision), but it is
/// just a placeholder for now.
pub type AnglePrecision = f64;

#[derive(Debug)]
pub struct PauliRotation {
    paulis: PauliList,
    angle: f64, // TODO - create AngleType
}

impl fmt::Display for PauliRotation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "pauli_rotation({:?}, {}", self.paulis, self.angle)
    }
}

pub struct PauliMeasure {
    paulis: u64, // TODO - or use SymbolIdList from instruction
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_pauli_rotation() {
        let pauli_rotation = PauliRotation {
            paulis: PauliList(vec![Pauli::I; 8]),
            angle: 0.5,
        };

        println!("KIT: Pauli Rotation Op: {:?}", pauli_rotation);
    }
}
