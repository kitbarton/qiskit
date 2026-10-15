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
pub mod ops;
pub mod qid;

use std::fmt;

use crate::line_graph::LineGraph;
use crate::ops::MirOp;
use crate::qid::QidList;

#[derive(Debug)]
pub struct SymbolIdList(Vec<u64>); // temp - replace with SymbolId when available

impl SymbolIdList {
    pub fn new(syms: Vec<u64>) -> Self {
        SymbolIdList(syms)
    }
    pub fn inner(&self) -> &Vec<u64> {
        &self.0
    }
}

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
#[derive(Debug)]
pub struct MirInstruction {
    op: MirOp,
    qids: QidList,
    args: SymbolIdList,
    returns: SymbolIdList, // Or just SymbolId
}

impl fmt::Display for MirInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "op: {:?}, qids: {:?}, args: {:?}, returns: {:?}",
            self.op, self.qids, self.args, self.returns
        )
    }
}

pub struct MirInstructionList {
    list: LineGraph<MirInstruction>,
}

impl fmt::Display for MirInstructionList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.list)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ops::pbc::{AngleType, Pauli, PauliList, PauliRotation};
    use crate::qid::Qid;

    #[test]
    fn test_create_instruction() {
        let instr = MirInstruction {
            op: MirOp::PauliRotation(PauliRotation::new(
                PauliList::new(vec![Pauli::I; 8]),
                AngleType::new(0.5),
            )),
            qids: QidList::new(vec![Qid::virt(0); 8]),
            args: SymbolIdList(vec![]),
            returns: SymbolIdList(vec![]),
        };
        println!("KIT: instr: {:?}", instr);
    }
}
