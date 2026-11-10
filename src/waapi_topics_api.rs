/// Wwise Authoring API endpoints
#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
pub mod ak {
    pub mod wwise {
        pub mod core {
            pub mod audio {
                pub const imported: &str = "ak.wwise.core.audio.imported";
            }
            pub mod log {
                pub const itemAdded: &str = "ak.wwise.core.log.itemAdded";
            }
            pub mod object {
                pub const attenuationCurveChanged: &str =
                    "ak.wwise.core.object.attenuationCurveChanged";
                pub const attenuationCurveLinkChanged: &str =
                    "ak.wwise.core.object.attenuationCurveLinkChanged";
                pub const childAdded: &str = "ak.wwise.core.object.childAdded";
                pub const childRemoved: &str = "ak.wwise.core.object.childRemoved";
                pub const created: &str = "ak.wwise.core.object.created";
                pub const curveChanged: &str = "ak.wwise.core.object.curveChanged";
                pub const nameChanged: &str = "ak.wwise.core.object.nameChanged";
                pub const notesChanged: &str = "ak.wwise.core.object.notesChanged";
                pub const postDeleted: &str = "ak.wwise.core.object.postDeleted";
                pub const preDeleted: &str = "ak.wwise.core.object.preDeleted";
                pub const propertyChanged: &str = "ak.wwise.core.object.propertyChanged";
                pub const referenceChanged: &str = "ak.wwise.core.object.referenceChanged";
            }
            pub mod profiler {
                pub const gameObjectRegistered: &str =
                    "ak.wwise.core.profiler.gameObjectRegistered";
                pub const gameObjectReset: &str = "ak.wwise.core.profiler.gameObjectReset";
                pub const gameObjectUnregistered: &str =
                    "ak.wwise.core.profiler.gameObjectUnregistered";
                pub const stateChanged: &str = "ak.wwise.core.profiler.stateChanged";
                pub const switchChanged: &str = "ak.wwise.core.profiler.switchChanged";

                pub mod captureLog {
                    pub const itemAdded: &str = "ak.wwise.core.profiler.captureLog.itemAdded";
                }
            }
            pub mod project {
                pub const loaded: &str = "ak.wwise.core.project.loaded";
                pub const postClosed: &str = "ak.wwise.core.project.postClosed";
                pub const preClosed: &str = "ak.wwise.core.project.preClosed";
                pub const saved: &str = "ak.wwise.core.project.saved";
            }
            pub mod soundbank {
                pub const generated: &str = "ak.wwise.core.soundbank.generated";
                pub const generationDone: &str = "ak.wwise.core.soundbank.generationDone";
            }
            pub mod switchContainer {
                pub const assignmentAdded: &str = "ak.wwise.core.switchContainer.assignmentAdded";
                pub const assignmentRemoved: &str =
                    "ak.wwise.core.switchContainer.assignmentRemoved";
            }
            pub mod transport {
                pub const stateChanged: &str = "ak.wwise.core.transport.stateChanged";
            }
        }
        pub mod debug {
            pub const assertFailed: &str = "ak.wwise.debug.assertFailed";
        }
        pub mod ui {
            pub const selectionChanged: &str = "ak.wwise.ui.selectionChanged";

            pub mod commands {
                pub const executed: &str = "ak.wwise.ui.commands.executed";
            }

            pub mod signal {
                pub const click: &str = "ak.wwise.ui.signal.click";
                pub const toggle: &str = "ak.wwise.ui.signal.toggle";
            }
        }
    }
}

/// Typed URI enum for `WaapiTopic` endpoints.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WaapiTopic {
    /// `ak.wwise.core.audio.imported`
    ak_wwise_core_audio_imported,
    /// `ak.wwise.core.log.itemAdded`
    ak_wwise_core_log_itemAdded,
    /// `ak.wwise.core.object.attenuationCurveChanged`
    ak_wwise_core_object_attenuationCurveChanged,
    /// `ak.wwise.core.object.attenuationCurveLinkChanged`
    ak_wwise_core_object_attenuationCurveLinkChanged,
    /// `ak.wwise.core.object.childAdded`
    ak_wwise_core_object_childAdded,
    /// `ak.wwise.core.object.childRemoved`
    ak_wwise_core_object_childRemoved,
    /// `ak.wwise.core.object.created`
    ak_wwise_core_object_created,
    /// `ak.wwise.core.object.curveChanged`
    ak_wwise_core_object_curveChanged,
    /// `ak.wwise.core.object.nameChanged`
    ak_wwise_core_object_nameChanged,
    /// `ak.wwise.core.object.notesChanged`
    ak_wwise_core_object_notesChanged,
    /// `ak.wwise.core.object.postDeleted`
    ak_wwise_core_object_postDeleted,
    /// `ak.wwise.core.object.preDeleted`
    ak_wwise_core_object_preDeleted,
    /// `ak.wwise.core.object.propertyChanged`
    ak_wwise_core_object_propertyChanged,
    /// `ak.wwise.core.object.referenceChanged`
    ak_wwise_core_object_referenceChanged,
    /// `ak.wwise.core.profiler.captureLog.itemAdded`
    ak_wwise_core_profiler_captureLog_itemAdded,
    /// `ak.wwise.core.profiler.gameObjectRegistered`
    ak_wwise_core_profiler_gameObjectRegistered,
    /// `ak.wwise.core.profiler.gameObjectReset`
    ak_wwise_core_profiler_gameObjectReset,
    /// `ak.wwise.core.profiler.gameObjectUnregistered`
    ak_wwise_core_profiler_gameObjectUnregistered,
    /// `ak.wwise.core.profiler.stateChanged`
    ak_wwise_core_profiler_stateChanged,
    /// `ak.wwise.core.profiler.switchChanged`
    ak_wwise_core_profiler_switchChanged,
    /// `ak.wwise.core.project.loaded`
    ak_wwise_core_project_loaded,
    /// `ak.wwise.core.project.postClosed`
    ak_wwise_core_project_postClosed,
    /// `ak.wwise.core.project.preClosed`
    ak_wwise_core_project_preClosed,
    /// `ak.wwise.core.project.saved`
    ak_wwise_core_project_saved,
    /// `ak.wwise.core.soundbank.generated`
    ak_wwise_core_soundbank_generated,
    /// `ak.wwise.core.soundbank.generationDone`
    ak_wwise_core_soundbank_generationDone,
    /// `ak.wwise.core.switchContainer.assignmentAdded`
    ak_wwise_core_switchContainer_assignmentAdded,
    /// `ak.wwise.core.switchContainer.assignmentRemoved`
    ak_wwise_core_switchContainer_assignmentRemoved,
    /// `ak.wwise.core.transport.stateChanged`
    ak_wwise_core_transport_stateChanged,
    /// `ak.wwise.debug.assertFailed`
    ak_wwise_debug_assertFailed,
    /// `ak.wwise.ui.commands.executed`
    ak_wwise_ui_commands_executed,
    /// `ak.wwise.ui.selectionChanged`
    ak_wwise_ui_selectionChanged,
    /// `ak.wwise.ui.signal.click`
    ak_wwise_ui_signal_click,
    /// `ak.wwise.ui.signal.toggle`
    ak_wwise_ui_signal_toggle,
}

impl WaapiTopic {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ak_wwise_core_audio_imported => ak::wwise::core::audio::imported,
            Self::ak_wwise_core_log_itemAdded => ak::wwise::core::log::itemAdded,
            Self::ak_wwise_core_object_attenuationCurveChanged => {
                ak::wwise::core::object::attenuationCurveChanged
            }
            Self::ak_wwise_core_object_attenuationCurveLinkChanged => {
                ak::wwise::core::object::attenuationCurveLinkChanged
            }
            Self::ak_wwise_core_object_childAdded => ak::wwise::core::object::childAdded,
            Self::ak_wwise_core_object_childRemoved => ak::wwise::core::object::childRemoved,
            Self::ak_wwise_core_object_created => ak::wwise::core::object::created,
            Self::ak_wwise_core_object_curveChanged => ak::wwise::core::object::curveChanged,
            Self::ak_wwise_core_object_nameChanged => ak::wwise::core::object::nameChanged,
            Self::ak_wwise_core_object_notesChanged => ak::wwise::core::object::notesChanged,
            Self::ak_wwise_core_object_postDeleted => ak::wwise::core::object::postDeleted,
            Self::ak_wwise_core_object_preDeleted => ak::wwise::core::object::preDeleted,
            Self::ak_wwise_core_object_propertyChanged => ak::wwise::core::object::propertyChanged,
            Self::ak_wwise_core_object_referenceChanged => {
                ak::wwise::core::object::referenceChanged
            }
            Self::ak_wwise_core_profiler_captureLog_itemAdded => {
                ak::wwise::core::profiler::captureLog::itemAdded
            }
            Self::ak_wwise_core_profiler_gameObjectRegistered => {
                ak::wwise::core::profiler::gameObjectRegistered
            }
            Self::ak_wwise_core_profiler_gameObjectReset => {
                ak::wwise::core::profiler::gameObjectReset
            }
            Self::ak_wwise_core_profiler_gameObjectUnregistered => {
                ak::wwise::core::profiler::gameObjectUnregistered
            }
            Self::ak_wwise_core_profiler_stateChanged => ak::wwise::core::profiler::stateChanged,
            Self::ak_wwise_core_profiler_switchChanged => ak::wwise::core::profiler::switchChanged,
            Self::ak_wwise_core_project_loaded => ak::wwise::core::project::loaded,
            Self::ak_wwise_core_project_postClosed => ak::wwise::core::project::postClosed,
            Self::ak_wwise_core_project_preClosed => ak::wwise::core::project::preClosed,
            Self::ak_wwise_core_project_saved => ak::wwise::core::project::saved,
            Self::ak_wwise_core_soundbank_generated => ak::wwise::core::soundbank::generated,
            Self::ak_wwise_core_soundbank_generationDone => {
                ak::wwise::core::soundbank::generationDone
            }
            Self::ak_wwise_core_switchContainer_assignmentAdded => {
                ak::wwise::core::switchContainer::assignmentAdded
            }
            Self::ak_wwise_core_switchContainer_assignmentRemoved => {
                ak::wwise::core::switchContainer::assignmentRemoved
            }
            Self::ak_wwise_core_transport_stateChanged => ak::wwise::core::transport::stateChanged,
            Self::ak_wwise_debug_assertFailed => ak::wwise::debug::assertFailed,
            Self::ak_wwise_ui_commands_executed => ak::wwise::ui::commands::executed,
            Self::ak_wwise_ui_selectionChanged => ak::wwise::ui::selectionChanged,
            Self::ak_wwise_ui_signal_click => ak::wwise::ui::signal::click,
            Self::ak_wwise_ui_signal_toggle => ak::wwise::ui::signal::toggle,
        }
    }
}

impl std::fmt::Display for WaapiTopic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl AsRef<str> for WaapiTopic {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
