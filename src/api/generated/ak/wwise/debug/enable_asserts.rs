// Generated bindings for ak.wwise.debug.enableAsserts

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.debug.enableAsserts`.
///
/// Enables debug assertions. Every call to enableAsserts with 'false' increments the ref count. Calling with true decrements the ref count. This is only available with Debug builds.
#[derive(Debug, Clone, serde::Serialize)]
pub struct DebugEnableAssertsArgs {
    /// Indicates whether assertions should be enabled or disabled.
    pub enable: bool,
}

impl DebugEnableAssertsArgs {
    /// Creates a new [`DebugEnableAssertsArgs`] with the required fields.
    pub fn new(enable: bool) -> Self {
        Self { enable }
    }
}

impl From<bool> for DebugEnableAssertsArgs {
    fn from(value: bool) -> Self {
        Self::new(value)
    }
}

/// Module-scoped alias for [`DebugEnableAssertsArgs`].
pub type EnableAssertsArgs = DebugEnableAssertsArgs;
