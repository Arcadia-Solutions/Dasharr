use std::{ops::Deref, sync::Arc};

use crate::{connection_pool::ConnectionPool, env::Env};

pub mod api_doc;
pub mod connection_pool;
pub mod env;
pub mod error;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod repositories;
pub mod routes;
pub mod services;

pub struct Dasharr {
    pub pool: Arc<ConnectionPool>,
    env: Env,
}

impl Deref for Dasharr {
    type Target = Env;

    fn deref(&self) -> &Self::Target {
        &self.env
    }
}

impl Dasharr {
    pub fn new(pool: Arc<ConnectionPool>, env: Env) -> Self {
        Self { pool, env }
    }
}
