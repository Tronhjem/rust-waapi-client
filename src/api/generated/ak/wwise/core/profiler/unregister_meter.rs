// Generated bindings for ak.wwise.core.profiler.unregisterMeter

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.profiler.unregisterMeter`.
///
/// Unregisters a bus or device that was registered with `ak.wwise.core.profiler.registerMeter`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreProfilerUnregisterMeterArgs {
    /// The ID (GUID), name, or path of the object to receive meter data. This object must be a bus, an aux bus or a device.
    pub object: ObjectRef,
}

impl CoreProfilerUnregisterMeterArgs {
    /// Creates a new [`CoreProfilerUnregisterMeterArgs`] with the required fields.
    pub fn new(object: ObjectRef) -> Self {
        Self { object }
    }
}
