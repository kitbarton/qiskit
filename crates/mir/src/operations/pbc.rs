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

pub struct PauliRotation {
    paulis: u64, // TODO - replace with Pauli or SymbolId when available
    angle: f64,  // TODO - create AngleType
}

impl fmt::Display for PauliRotation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "pauli_rotation({}, {}", self.paulis, self.angle)
    }
}

pub struct PauliMeasure {
    paulis: u64, // TODO - or use SymbolIdList from instruction
}
