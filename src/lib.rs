//! SigmaOS-env — Environment variable model.
//!
//! Models managing and displaying environment variables.
//! Deterministic and std-only.

pub mod environment;
pub mod options;

pub use environment::{Environment, EnvVar};
pub use options::{EnvOptions, UnsetMode};
