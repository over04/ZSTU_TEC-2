use crate::app::{AppUtil, Route, Router};
use std::any::type_name_of_val;
use std::fmt::Debug;

pub struct RouterManager {
    routes: Vec<Route>,
}

impl RouterManager {
    pub fn new<T: Router>(util: AppUtil) -> Self {
        Self {
            routes: vec![T::default_route(util)],
        }
    }

    pub fn push(&mut self, route: Route) {
        self.routes.push(route);
    }

    pub fn pop(&mut self) -> Option<Route> {
        self.routes.pop()
    }
}

impl Debug for RouterManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.routes
            .iter()
            .map(|val| type_name_of_val(val.as_ref()))
            .collect::<Vec<_>>()
            .fmt(f)
    }
}
