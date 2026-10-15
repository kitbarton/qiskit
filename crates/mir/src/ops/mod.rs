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

//! Definition of operations in Qiskit MIR.
//!
//! Each file represents a different logical category of operations.

pub mod pbc;

use crate::ops::pbc::PauliRotation;

pub enum MirOp {
    // Quantum operations
    PauliRotation(PauliRotation),
    // PauliMeasure(PauliMeasure),
    // Control flow operations
    ConditionalBranch,
    UnconditionalBranch,
}
