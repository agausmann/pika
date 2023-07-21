//! Intermediate language for optimization and codegen passes.
//!
//! Modeled after the intermediate language used by the QBE compiler backend.
//! https://c9x.me/compile/doc/il.html

use std::collections::HashMap;

use crate::token::{Ident, IntLiteral};

pub struct Module {
    pub functions: HashMap<Ident, Function>,
}

pub struct Function {
    pub assembly: Assembly,
}

pub struct Assembly {
    instructions: Vec<Instruction>,
    labels: HashMap<Label, usize>,
    next_temporary: usize,
    next_label: usize,
}

impl Assembly {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            labels: HashMap::new(),
            next_temporary: 0,
            next_label: 0,
        }
    }

    pub fn new_temporary(&mut self) -> Temporary {
        let result = Temporary(self.next_temporary);
        self.next_temporary += 1;
        result
    }

    pub fn new_label(&mut self) -> Label {
        let result = Label(self.next_label);
        self.next_label += 1;
        result
    }

    pub fn set_label(&mut self, label: Label) {
        assert!(
            !self.labels.contains_key(&label),
            "attempt to set label more than once"
        );
        self.labels.insert(label, self.instructions.len());
    }

    pub fn push(&mut self, instr: Instruction) {
        self.instructions.push(instr)
    }
}

pub enum Instruction {
    Operation(Output, Operation),
    Call(Option<Output>, Call),
    Load {
        output: Output,
        addr: Value,
    },
    Store {
        addr: Value,
        value: Value,
    },
    Alloc {
        addr_output: Output,
        size: u64,
        alignment: u64,
    },
    Continuation(Continuation),
}

pub enum Operation {
    Binary(BinaryOp, Value, Value),
    Unary(UnaryOp, Value),
}

pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Or,
    Xor,
    And,
    Shr,
    Shl,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

pub enum UnaryOp {
    Neg,
    Not,
    Convert,
    Cast,
}

pub struct Call {
    // TODO function name type
    pub function_name: String,
    pub arguments: Vec<Value>,
}

pub struct Output {
    pub dest: Temporary,
    // pub dest_type: Type,
}

pub enum Continuation {
    Jump(Label),
    BranchZero(Value, Label),
    BranchNonZero(Value, Label),
    Return(Value),
    Halt,
}

#[derive(Debug, Clone)]
pub enum Value {
    Temporary(Temporary),
    Literal(Literal),
    Argument(Argument),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Temporary(usize);

#[derive(Debug, Clone)]
pub enum Literal {
    Nil,
    Int(IntLiteral),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Argument(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Label(usize);

pub enum Type {
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    Isize,
    Usize,
    F32,
    F64,
}
