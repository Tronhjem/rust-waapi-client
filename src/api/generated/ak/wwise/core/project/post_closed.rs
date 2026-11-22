// Generated bindings for topic ak.wwise.core.project.postClosed

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Payload for `ak.wwise.core.project.postClosed`.
///
/// Sent when the after the project is completely closed.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct CoreProjectPostClosedPayload {}

/// Module-scoped alias for [`CoreProjectPostClosedPayload`].
pub type PostClosedPayload = CoreProjectPostClosedPayload;
