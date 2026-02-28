//! Token-based authentication middleware.
//!
//! Supports both Bearer token and Basic Auth (with token as password).

mod middleware;

pub use middleware::{TokenAuthLayer, TokenAuthService};
