#![deny(clippy::nursery)]
#![deny(clippy::pedantic)]
#![deny(clippy::unwrap_used)]
#![allow(clippy::missing_errors_doc)]
#![feature(io_error_more)]

pub mod args;
pub mod commands;
pub mod errors;
pub mod rules;
