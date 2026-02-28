//! SSE transport for MCP HTTP mode.
//!
//! Provides a custom SSE server implementation that can be wrapped with
//! authentication middleware.

mod sse;

pub use sse::{AuthSseServer, SseTransport};
