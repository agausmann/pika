//! Intermediate language for optimization and codegen passes.
//!
//! Modeled after the intermediate language used by the QBE compiler backend.
//! https://c9x.me/compile/doc/il.html

use std::collections::HashMap;

use crate::{
    ast::Path,
    token::{Ident, IntLiteral},
};

#[derive(Debug)]
pub struct Module {
    pub functions: HashMap<Ident, Function>,
}

#[derive(Debug)]
pub struct Function {
    pub assembly: Assembly,
}

#[derive(Debug)]
pub struct Scope<'parent> {
    parent: Option<&'parent Scope<'parent>>,
    locals: HashMap<Path, Value>,
}

impl<'parent> Scope<'parent> {
    pub fn new(parent: Option<&'parent Scope<'parent>>) -> Self {
        Self {
            parent,
            locals: HashMap::new(),
        }
    }

    pub fn resolve(&self, path: &Path) -> &Value {
        self.locals
            .get(path)
            .or_else(|| self.parent.as_ref().map(|parent| parent.resolve(path)))
            .unwrap()
    }

    pub fn declare(&mut self, path: Path, value: Value) {
        assert!(!self.locals.contains_key(&path));
        self.locals.insert(path, value);
    }
}

#[derive(Debug)]
pub struct Assembly {
    instructions: Vec<Instruction>,
    labels: HashMap<Label, usize>,
    reverse_labels: HashMap<usize, Vec<Label>>,
    next_temporary: usize,
    next_label: usize,
}

impl Assembly {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            labels: HashMap::new(),
            reverse_labels: HashMap::new(),
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
        let position = self.instructions.len();
        self.labels.insert(label, position);
        self.reverse_labels
            .entry(position)
            .or_insert(Vec::new())
            .push(label);
    }

    pub fn push(&mut self, instr: Instruction) {
        self.instructions.push(instr)
    }

    pub fn instructions(&self) -> &[Instruction] {
        &self.instructions
    }

    pub fn labels_at(&self, position: usize) -> &[Label] {
        self.reverse_labels
            .get(&position)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum Operation {
    Binary(BinaryOp, Value, Value),
    Unary(UnaryOp, Value),
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum UnaryOp {
    Neg,
    Not,
    Convert,
    Cast,
}

#[derive(Debug)]
pub struct Call {
    // TODO function name type
    pub function_name: String,
    pub arguments: Vec<Value>,
}

#[derive(Debug)]
pub struct Output {
    pub dest: Temporary,
    // pub dest_type: Type,
}

#[derive(Debug)]
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
pub struct Argument(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Label(usize);

#[derive(Debug)]
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
