// Generated bindings for ak.wwise.core.profiler.registerMeter

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::api::generated::primitives::*;

/// Arguments for `ak.wwise.core.profiler.registerMeter`.
///
/// Registers a bus, an aux bus or device to receive meter data. Only the Main Audio Bus is registered by default. Use `ak.wwise.core.profiler.getMeters` to retrieve the meter data after registering. Every call to ak.wwise.core.profiler.registerMeter must have a matching call to `ak.wwise.core.profiler.unregisterMeter`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoreProfilerRegisterMeterArgs {
    /// The ID (GUID), name, or path of the object to receive meter data. This object must be a bus, an aux bus or a device.
    pub object: ObjectRef,
}

impl CoreProfilerRegisterMeterArgs {
    /// Creates a new [`CoreProfilerRegisterMeterArgs`] with the required fields.
    pub fn new(object: impl Into<ObjectRef>) -> Self {
        Self {
            object: object.into(),
        }
    }
}

impl From<ObjectRef> for CoreProfilerRegisterMeterArgs {
    fn from(value: ObjectRef) -> Self {
        Self::new(value)
    }
}

impl From<&str> for CoreProfilerRegisterMeterArgs {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for CoreProfilerRegisterMeterArgs {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&String> for CoreProfilerRegisterMeterArgs {
    fn from(value: &String) -> Self {
        Self::new(value)
    }
}

/// Module-scoped alias for [`CoreProfilerRegisterMeterArgs`].
pub type RegisterMeterArgs = CoreProfilerRegisterMeterArgs;
