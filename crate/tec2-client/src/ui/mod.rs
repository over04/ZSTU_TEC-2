use crate::app::{AppUtil, Page, Route, Router};

mod main;
mod parser;

pub use main::Main;
pub use parser::Parser;

pub struct Tec2ClientRouter;

impl Router for Tec2ClientRouter {
    fn default_route(util: AppUtil) -> Route
    where
        Self: Sized,
    {
        Box::new(Main::new(util))
    }
}
