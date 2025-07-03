use lalrpop_util::lalrpop_mod;
pub mod ast;
mod custom_macro;
pub mod error;
pub mod parser;

pub use error::{CanNotBeAchievedReason, Error, Result};

lalrpop_mod!(pub grammar);
