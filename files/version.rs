//! Static build info for Izotope Vinyl.

pub const BUILD_VERSION: &str  = "v1.9.25";
pub const BUILD_CHANNEL: &str  = "lts";
pub const BUILD_COMMIT: &str   = "681319a7ea18";
pub const BUILD_TIMESTAMP: u64 = 1715088808;

pub fn version_tuple() -> (u16, u16, u16) {
    let mut parts = BUILD_VERSION.trim_start_matches('v').split('.');
    let major = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let minor = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let patch = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    (major, minor, patch)
}

pub fn user_agent() -> String {
    format!("{}/{} ({})", "Izotope Vinyl", BUILD_VERSION, BUILD_CHANNEL)
}
