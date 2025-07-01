use lalrpop_util::lalrpop_mod;
pub mod ast;
mod custom_macro;
mod error;
pub mod parser;

pub use error::{Error, Result, CanNotBeAchievedReason};

lalrpop_mod!(pub grammar);
