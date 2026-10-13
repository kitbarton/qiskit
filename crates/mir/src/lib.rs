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

//! Qiskit middle-end intermediate representation (MIR) for fault-tolerant quantum programs.
//!
//! This is the base crate that defines the IR, its data structures and built-in operations and
//! algorithms.

pub mod line_graph;
pub mod operations;
pub mod qid;

use crate::line_graph::LineGraph;
use crate::operations::MirOp;

type QidList = Vec<u64>; // temp - replace with Qid when available
type SymbolIdList = Vec<u64>; // temp - relpace with SymbolId when available

/// Instruction in Qiskit Medium IR
///
/// This represents an instruction in MIR. Each instruction consists of an
/// operation, a list of Quantum IDs, a list of SymbolIds of arguments and a
/// list of SymbolIds for returns. Any of these lists may be empty, indicating
/// that the specific field is not required for the given operation. For
/// example, a `PauliRotation` does not "return" any values and thus the
/// `returns` field should be an empty vector. Similarly, a branch operation
/// does not operate on, or effect any QuantumIDs and thus the `qids` list
/// should be empty.
pub struct MirInstruction {
    op: MirOp,
    qids: QidList,
    args: SymbolIdList,
    returns: SymbolIdList, // Or just SymbolId
}

pub struct MirInstructionList {
    list: LineGraph<MirInstruction>,
}
