//! Connection URL parsing (postgres://, mysql://, jdbc:..., sqlite:...).

use dbayam_core::config::ConnectionConfig;
use dbayam_core::{DbResult, Secret};

/// Parse a connection URL into a configuration and an optional password.
pub fn parse(url: &str) -> DbResult<(ConnectionConfig, Option<Secret>)> {
    dbayam_config::parse_connection_url(url)
}

/// Format a configuration as a URL (without the password).
pub fn format(cfg: &ConnectionConfig) -> String {
    dbayam_config::to_url(cfg)
}
