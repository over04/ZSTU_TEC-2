use ratatui::Frame;
use ratatui::crossterm::event::KeyEvent;
use std::sync::Arc;

mod app;
mod router;

pub use app::App;
pub use router::RouterManager;

pub trait Page: Send + Sync {
    fn new(util: AppUtil) -> Self
    where
        Self: Sized;
    fn render(&mut self, frame: &mut Frame);
    fn handle_key_event(&mut self, event: KeyEvent);
}

pub trait Router {
    fn default_route(util: AppUtil) -> Route
    where
        Self: Sized;
}

pub type Route = Box<dyn Page>;

pub type AppUtil = Arc<app::AppUtil>;
