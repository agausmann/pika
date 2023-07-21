use chumsky::prelude::*;

use crate::{
    il,
    token::{Ident, IntLiteral, Token},
};

pub fn module() -> impl Parser<Token, Module, Error = Simple<Token>> {
    let ident = select! {
        Token::Ident(x) => x,
    };
    let int_literal = select! {
        Token::IntLiteral(x) => x,
    };

    let path = ident
        .separated_by(just(Token::Colon2))
        .at_least(1)
        .map(|elements| Path { elements });

    let expr = recursive(|expr| {
        let field_init = ident
            .then_ignore(just(Token::Colon))
            .then(expr.clone())
            .map(|(name, value)| FieldInit { name, value });

        let struct_init = path
            .clone()
            .then_ignore(just(Token::OpenBrace))
            .then(field_init.separated_by(just(Token::Comma)).allow_trailing())
            .then_ignore(just(Token::CloseBrace))
            .map(|(name, fields)| StructInit { name, fields });

        let array_init = just(Token::OpenBracket)
            .ignore_then(choice((
                expr.clone()
                    .then_ignore(just(Token::Semicolon))
                    .then(int_literal)
                    .map(|(element, size)| ArrayInit::Fill {
                        element: Box::new(element),
                        size,
                    }),
                expr.clone()
                    .separated_by(just(Token::Comma))
                    .allow_trailing()
                    .map(ArrayInit::Elements),
            )))
            .then_ignore(just(Token::CloseBracket));

        let factor = choice((
            struct_init.map(Expr::StructInit),
            array_init.map(Expr::ArrayInit),
            path.clone().map(Expr::Path),
            int_literal.map(Expr::IntLiteral),
        ));
        let prefix_op = select! {
            Token::Exclam => PrefixOp::Not,
        };
        let suffix_op = choice((
            just(Token::Dot)
                .ignore_then(ident)
                .map(SuffixOp::FieldAccess),
            just(Token::OpenBracket)
                .ignore_then(expr.clone())
                .map(Box::new)
                .then_ignore(just(Token::CloseBracket))
                .map(SuffixOp::ArrayIndex),
        ));
        let term = prefix_op
            .repeated()
            .then(factor)
            .then(suffix_op.repeated())
            .map(|((prefixes, factor), suffixes)| {
                let mut acc = factor;
                // Suffixes take precedence over prefixes
                for suffix in suffixes {
                    acc = Expr::Suffix(Box::new(acc), suffix);
                }
                for prefix in prefixes {
                    acc = Expr::Prefix(prefix, Box::new(acc));
                }
                acc
            });

        let binary_op = select! {
            Token::Plus => BinaryOp::Plus,
            Token::Minus => BinaryOp::Minus,
            Token::And2 => BinaryOp::LogicAnd,
            Token::Eq2 => BinaryOp::CmpEq,
        };

        term.clone()
            .then((binary_op.then(term)).repeated())
            .map(|(head, tail)| {
                // Shunting yard algorithm
                let mut output: Vec<Expr> = Vec::with_capacity(tail.len());
                let mut operators: Vec<BinaryOp> = Vec::with_capacity(tail.len());
                output.push(head);

                for (op, rhs) in tail {
                    loop {
                        match operators.last() {
                            Some(&op2)
                                if op2.precedence() > op.precedence()
                                    || (op2.precedence() == op.precedence()
                                        && op.is_left_associative()) =>
                            {
                                let r = output.pop().unwrap();
                                let l = output.pop().unwrap();
                                output.push(Expr::Binary(op2, Box::new(l), Box::new(r)));
                                operators.pop();
                            }
                            _ => break,
                        }
                    }
                    operators.push(op);
                    output.push(rhs);
                }
                while let Some(op) = operators.pop() {
                    let r = output.pop().unwrap();
                    let l = output.pop().unwrap();
                    output.push(Expr::Binary(op, Box::new(l), Box::new(r)));
                }

                assert!(output.len() == 1);
                output.pop().unwrap()
            })
    });

    let type_name = recursive(|type_name| {
        let array_type = just(Token::OpenBracket)
            .ignore_then(type_name)
            .then_ignore(just(Token::Semicolon))
            .then(int_literal)
            .then_ignore(just(Token::CloseBracket))
            .map(|(element, size)| ArrayType {
                element: Box::new(element),
                size,
            });
        choice((path.map(Type::Path), array_type.map(Type::Array)))
    });

    let fn_arg = ident
        .then_ignore(just(Token::Colon))
        .then(type_name.clone())
        .map(|(arg_name, arg_type)| FnArg { arg_name, arg_type });

    let block = recursive(|block| {
        let let_statement = just(Token::Let)
            .ignore_then(just(Token::Mut).or_not())
            .then(ident)
            .then(just(Token::Colon).ignore_then(type_name.clone()).or_not())
            .then_ignore(just(Token::Eq))
            .then(expr.clone())
            .then_ignore(just(Token::Semicolon))
            .map(|(((is_mut, binding), binding_type), value)| Let {
                is_mut: is_mut.is_some(),
                binding,
                binding_type,
                value,
            });
        let assign_statement = expr
            .clone()
            .then_ignore(just(Token::Eq))
            .then(expr.clone())
            .then_ignore(just(Token::Semicolon))
            .map(|(dest, src)| Assign { dest, src });

        let if_statement = just(Token::If)
            .ignore_then(expr.clone())
            .then(block.clone())
            .map(|(condition, body)| IfCase { condition, body })
            .separated_by(just(Token::Else))
            .at_least(1)
            .then(just(Token::Else).ignore_then(block.clone()).or_not())
            .map(|(cases, else_case)| If { cases, else_case });

        let iterable = int_literal
            .then_ignore(just(Token::Dot2))
            .then(int_literal)
            .map(|(start, end)| Iterable::Range(start, end));
        let for_statement = just(Token::For)
            .ignore_then(ident)
            .then_ignore(just(Token::In))
            .then(iterable)
            .then(block.clone())
            .map(|((binding, target), body)| For {
                binding,
                iterable: target,
                body,
            });
        let return_statement = just(Token::Return)
            .ignore_then(expr.clone())
            .then_ignore(just(Token::Semicolon));
        let break_statement = just(Token::Break).then(just(Token::Semicolon));
        let statement = choice((
            block.map(Statement::Block),
            let_statement.map(Statement::Let),
            assign_statement.map(Statement::Assign),
            if_statement.map(Statement::If),
            for_statement.map(Statement::For),
            return_statement.map(Statement::Return),
            break_statement.to(Statement::Break),
        ));

        just(Token::OpenBrace)
            .ignore_then(statement.repeated())
            .then(expr.or_not())
            .then_ignore(just(Token::CloseBrace))
            .map(|(statements, expr)| Block { statements, expr })
    });

    let fn_item = just(Token::Fn)
        .ignore_then(ident)
        .then(
            just(Token::OpenParen)
                .ignore_then(fn_arg.separated_by(just(Token::Comma)).allow_trailing())
                .then_ignore(just(Token::CloseParen)),
        )
        .then_ignore(just(Token::RightArrow))
        .then(type_name.clone())
        .then(block)
        .map(|(((name, args), return_type), body)| FnItem {
            name,
            args,
            return_type,
            body,
        });

    let field =
        ident
            .then_ignore(just(Token::Colon))
            .then(type_name)
            .map(|(field_name, field_type)| Field {
                field_name,
                field_type,
            });

    let struct_item = just(Token::Struct)
        .ignore_then(ident)
        .then_ignore(just(Token::OpenBrace))
        .then(field.separated_by(just(Token::Comma)).allow_trailing())
        .then_ignore(just(Token::CloseBrace))
        .map(|(name, fields)| StructItem { name, fields });

    let variant = ident.map(|name| Variant { name });
    let enum_item = just(Token::Enum)
        .ignore_then(ident)
        .then_ignore(just(Token::OpenBrace))
        .then(variant.separated_by(just(Token::Comma)).allow_trailing())
        .then_ignore(just(Token::CloseBrace))
        .map(|(name, variants)| EnumItem { name, variants });

    let item = choice((
        fn_item.map(Item::Fn),
        struct_item.map(Item::Struct),
        enum_item.map(Item::Enum),
    ));
    item.repeated()
        .then_ignore(end())
        .map(|items| Module { items })
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Path {
    pub elements: Vec<Ident>,
}

#[derive(Debug, Clone)]
pub struct Module {
    pub items: Vec<Item>,
}

#[derive(Debug, Clone)]
pub enum Item {
    Fn(FnItem),
    Struct(StructItem),
    Enum(EnumItem),
}

#[derive(Debug, Clone)]
pub struct FnItem {
    pub name: Ident,
    pub args: Vec<FnArg>,
    pub return_type: Type,
    pub body: Block,
}

impl FnItem {
    pub fn visit_il(&self, module: &mut il::Module) {
        let mut assembly = il::Assembly::new();
        self.body.visit_il(&mut assembly);
        module
            .functions
            .insert(self.name.clone(), il::Function { assembly });
    }
}

#[derive(Debug, Clone)]
pub struct FnArg {
    pub arg_name: Ident,
    pub arg_type: Type,
}

#[derive(Debug, Clone)]
pub enum Type {
    Path(Path),
    Array(ArrayType),
}

#[derive(Debug, Clone)]
pub struct ArrayType {
    pub element: Box<Type>,
    pub size: IntLiteral,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<Statement>,
    pub expr: Option<Expr>,
}

impl Block {
    pub fn visit_il(&self, assembly: &mut il::Assembly) -> il::Value {
        for stmt in &self.statements {
            stmt.visit_il(assembly);
        }
        self.expr
            .as_ref()
            .map(|expr| expr.visit_il(assembly))
            .unwrap_or(il::Value::Literal(il::Literal::Nil))
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Path(Path),
    IntLiteral(IntLiteral),
    StructInit(StructInit),
    ArrayInit(ArrayInit),
    Prefix(PrefixOp, Box<Expr>),
    Suffix(Box<Expr>, SuffixOp),
    Binary(BinaryOp, Box<Expr>, Box<Expr>),
}

impl Expr {
    pub fn visit_il(&self, assembly: &mut il::Assembly) -> il::Value {
        match self {
            Self::Path(_) => todo!(),
            Self::IntLiteral(int) => il::Value::Literal(il::Literal::Int(int.clone())),
            Self::StructInit(_) => todo!(),
            Self::ArrayInit(_) => todo!(),
            Self::Prefix(op, expr) => {
                let expr = expr.visit_il(assembly);
                let dest = assembly.new_temporary();
                assembly.push(il::Instruction::Operation(
                    il::Output { dest },
                    il::Operation::Unary(op.into(), expr),
                ));
                il::Value::Temporary(dest)
            }
            Self::Suffix(_, _) => todo!(),
            Self::Binary(op, left, right) => {
                let left = left.visit_il(assembly);
                let right = right.visit_il(assembly);

                let dest = assembly.new_temporary();
                assembly.push(il::Instruction::Operation(
                    il::Output { dest },
                    il::Operation::Binary(op.into(), left, right),
                ));
                il::Value::Temporary(dest)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct StructInit {
    pub name: Path,
    pub fields: Vec<FieldInit>,
}

#[derive(Debug, Clone)]
pub struct FieldInit {
    pub name: Ident,
    pub value: Expr,
}

#[derive(Debug, Clone)]
pub enum ArrayInit {
    Elements(Vec<Expr>),
    Fill {
        element: Box<Expr>,
        size: IntLiteral,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum PrefixOp {
    Not,
}

impl From<&PrefixOp> for il::UnaryOp {
    fn from(value: &PrefixOp) -> Self {
        match value {
            PrefixOp::Not => Self::Not,
        }
    }
}

#[derive(Debug, Clone)]
pub enum SuffixOp {
    FieldAccess(Ident),
    ArrayIndex(Box<Expr>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinaryOp {
    Plus,
    Minus,
    CmpEq,
    LogicAnd,
}

impl From<&BinaryOp> for il::BinaryOp {
    fn from(value: &BinaryOp) -> Self {
        match value {
            BinaryOp::Plus => Self::Add,
            BinaryOp::Minus => Self::Sub,
            BinaryOp::CmpEq => Self::Eq,
            BinaryOp::LogicAnd => Self::And,
        }
    }
}

impl BinaryOp {
    fn precedence(&self) -> usize {
        match self {
            Self::Plus | Self::Minus => 2,
            Self::CmpEq => 1,
            Self::LogicAnd => 0,
        }
    }

    fn is_left_associative(&self) -> bool {
        match self {
            _ => true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct StructItem {
    pub name: Ident,
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone)]
pub struct Field {
    pub field_name: Ident,
    pub field_type: Type,
}

#[derive(Debug, Clone)]
pub struct EnumItem {
    pub name: Ident,
    pub variants: Vec<Variant>,
}

#[derive(Debug, Clone)]
pub struct Variant {
    pub name: Ident,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Block(Block),
    Let(Let),
    Assign(Assign),
    If(If),
    For(For),
    Return(Expr),
    Break,
}

impl Statement {
    pub fn visit_il(&self, assembly: &mut il::Assembly) {
        match self {
            Self::Block(block) => {
                block.visit_il(assembly);
            }
            Self::Let(lett) => lett.visit_il(assembly),
            Self::Assign(assign) => assign.visit_il(assembly),
            Self::If(iff) => iff.visit_il(assembly),
            Self::For(forr) => forr.visit_il(assembly),
            Self::Return(expr) => {
                let expr = expr.visit_il(assembly);
                assembly.push(il::Instruction::Continuation(il::Continuation::Return(
                    expr,
                )));
            }
            Self::Break => todo!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Let {
    pub is_mut: bool,
    pub binding: Ident,
    pub binding_type: Option<Type>,
    pub value: Expr,
}

impl Let {
    fn visit_il(&self, assembly: &mut il::Assembly) {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct Assign {
    pub dest: Expr,
    pub src: Expr,
}

impl Assign {
    fn visit_il(&self, assembly: &mut il::Assembly) {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct If {
    pub cases: Vec<IfCase>,
    pub else_case: Option<Block>,
}

impl If {
    fn visit_il(&self, assembly: &mut il::Assembly) {
        let end = if self.cases.len() > 1 || self.else_case.is_some() {
            Some(assembly.new_label())
        } else {
            None
        };

        for case in &self.cases {
            let condition = case.condition.visit_il(assembly);
            let skip = assembly.new_label();
            assembly.push(il::Instruction::Continuation(il::Continuation::BranchZero(
                condition, skip,
            )));
            case.body.visit_il(assembly);
            if let Some(end) = end {
                assembly.push(il::Instruction::Continuation(il::Continuation::Jump(end)));
            }
            assembly.set_label(skip);
        }

        if let Some(else_case) = &self.else_case {
            else_case.visit_il(assembly);
        }

        if let Some(end) = end {
            assembly.set_label(end);
        }
    }
}

#[derive(Debug, Clone)]
pub struct IfCase {
    pub condition: Expr,
    pub body: Block,
}

#[derive(Debug, Clone)]
pub struct For {
    pub binding: Ident,
    pub iterable: Iterable,
    pub body: Block,
}

impl For {
    fn visit_il(&self, assembly: &mut il::Assembly) {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub enum Iterable {
    Range(IntLiteral, IntLiteral),
}
