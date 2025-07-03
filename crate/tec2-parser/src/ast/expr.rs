use crate::ast::token::{Flag, Identifier, Operator};

#[derive(Debug)]
pub struct Expr {
    pub assignment: Option<Assignment>,
    pub flag_expr: Option<Box<FlagExpr>>,
}

#[derive(Debug)]
pub struct FlagExpr {
    pub flag: Flag,
    pub next: Option<Box<FlagExpr>>,
}

#[derive(Debug)]
pub struct Assignment {
    pub term: Term,
    pub identifier: Identifier,
}

#[derive(Debug)]
pub struct Term {
    pub left: Primary,
    pub right: Option<(Operator, Primary)>,
}

#[derive(Debug)]
pub enum Primary {
    Identifier(Identifier),
    Number(u16),
}

impl Expr {
    pub fn get_flag_vec(&self) -> Vec<&Flag> {
        let mut flag_exprs = vec![];
        let mut current = self.flag_expr.as_ref();
        while let Some(flag_expr) = current {
            flag_exprs.push(&flag_expr.flag);
            current = flag_expr.next.as_ref();
        }
        flag_exprs
    }
}
