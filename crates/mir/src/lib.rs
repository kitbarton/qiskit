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

use crate::line_graph::{LineGraph, iter::IterMain};
use crate::ops::MirOp;
use crate::qid::QidList;

#[derive(Clone, Debug, PartialEq)]
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
#[derive(Clone, Debug, PartialEq)]
pub struct MirInstruction {
    op: MirOp,
    qids: QidList,
    args: SymbolIdList,
    returns: SymbolIdList, // Or just SymbolId
}

impl MirInstruction {
    pub fn new(op: MirOp, qids: QidList, args: SymbolIdList, returns: SymbolIdList) -> Self {
        Self {
            op,
            qids,
            args,
            returns,
        }
    }

    pub fn op(&self) -> &MirOp {
        &self.op
    }

    pub fn qids(&self) -> &QidList {
        &self.qids
    }

    pub fn args(&self) -> &SymbolIdList {
        &self.args
    }
    pub fn returns(&self) -> &SymbolIdList {
        &self.returns
    }
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

pub struct MirInstructionIndex(line_graph::Index);

impl MirInstructionIndex {
    pub fn inner(&self) -> line_graph::Index {
        self.0
    }
}

#[derive(Debug)]
pub struct MirInstructionList {
    instr_list: LineGraph<MirInstruction>,
}

impl MirInstructionList {
    pub fn new() -> Self {
        Self {
            instr_list: LineGraph::<MirInstruction>::new(),
        }
    }

    pub fn push_back(&mut self, instr: MirInstruction) -> MirInstructionIndex {
        MirInstructionIndex(self.instr_list.push_back(instr))
    }

    pub fn push_front(&mut self, instr: MirInstruction) -> MirInstructionIndex {
        MirInstructionIndex(self.instr_list.push_front(instr))
    }

    pub fn iter(&self) -> MirInstructionListIter<'_> {
        MirInstructionListIter {
            inner: self.instr_list.iter_main(),
            //inner: self.instr_list.iter(),
        }
    }
}

impl<'a> IntoIterator for &'a MirInstructionList {
    type Item = &'a MirInstruction;
    type IntoIter = MirInstructionListIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl fmt::Display for MirInstructionList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.instr_list)
    }
}

pub struct MirInstructionListIter<'a> {
    //inner: std::slice::Iter<'a, MirInstruction>,
    inner: IterMain<'a, MirInstruction>,
}

impl<'a> Iterator for MirInstructionListIter<'a> {
    type Item = &'a MirInstruction;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|s| s.weight())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ops::pbc::{AngleType, Pauli, PauliList, PauliRotation};
    use crate::qid::Qid;

    fn create_pauli_rotation_op(pauli: Pauli, size: usize, angle: f64) -> MirOp {
        MirOp::PauliRotation(PauliRotation::new(
            PauliList::new(vec![pauli; size]),
            AngleType::new(angle),
        ))
    }

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
    }

    #[test]
    fn test_instruction_list() {
        let mut instr_list = MirInstructionList::new();

        let instr1 = MirInstruction {
            op: create_pauli_rotation_op(Pauli::X, 8, 0.5),
            qids: QidList::new(vec![Qid::virt(0); 8]),
            args: SymbolIdList(vec![]),
            returns: SymbolIdList(vec![]),
        };

        let instr2 = MirInstruction {
            op: create_pauli_rotation_op(Pauli::Y, 8, 0.5),
            qids: QidList::new(vec![Qid::virt(0); 8]),
            args: SymbolIdList(vec![]),
            returns: SymbolIdList(vec![]),
        };

        let instr3 = MirInstruction {
            op: create_pauli_rotation_op(Pauli::Z, 8, 0.5),
            qids: QidList::new(vec![Qid::virt(0); 8]),
            args: SymbolIdList(vec![]),
            returns: SymbolIdList(vec![]),
        };

        let check_list = vec![instr3.clone(), instr1.clone(), instr2.clone()];

        instr_list.push_back(instr1);
        instr_list.push_back(instr2);
        instr_list.push_front(instr3);

        // Iterate through the instruction list.
        // Expect to find instr3, instr1, instr2
        for (index, instr) in instr_list.iter().enumerate() {
            assert_eq!(*instr, check_list[index])
        }
    }
}
