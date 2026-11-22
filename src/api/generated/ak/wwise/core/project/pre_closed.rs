// Generated bindings for topic ak.wwise.core.project.preClosed

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Payload for `ak.wwise.core.project.preClosed`.
///
/// Sent when the project begins closing.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct CoreProjectPreClosedPayload {}

/// Module-scoped alias for [`CoreProjectPreClosedPayload`].
pub type PreClosedPayload = CoreProjectPreClosedPayload;
