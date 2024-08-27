// src/config/db_setup/updates/mod.rs

pub mod update_v1;

pub use update_v1::apply_update_v1;

pub mod update_v2;

pub use update_v2::apply_update_v2;
