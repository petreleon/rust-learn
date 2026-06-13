pub mod application;
pub mod bootstrap;
pub mod config;
pub mod db;
pub mod domain;
pub mod http;
pub mod infra;
pub mod middlewares;
pub mod models;
pub mod repositories;
pub mod services;
pub mod shared;
pub mod utils;

// Keep lib lightweight; main.rs remains the binary entrypoint.
