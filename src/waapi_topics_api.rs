/// Wwise Authoring API URI constants organized by namespace.
#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
pub mod ak {
    /// The `ak.wwise` WAAPI namespace.
    pub mod wwise {
        /// The `ak.wwise.core` WAAPI namespace.
        pub mod core {
            /// The `ak.wwise.core.audio` WAAPI namespace.
            pub mod audio {
                /// Sent at the end of an import operation.
                ///
                /// WAAPI URI: `ak.wwise.core.audio.imported`.
                pub const imported: &str = "ak.wwise.core.audio.imported";
            }
            /// The `ak.wwise.core.log` WAAPI namespace.
            pub mod log {
                /// Sent when an item is added to the log. This could be used to retrieve items added to the SoundBank generation log. To retrieve the complete log, see `ak.wwise.core.log.get`.
                ///
                /// WAAPI URI: `ak.wwise.core.log.itemAdded`.
                pub const itemAdded: &str = "ak.wwise.core.log.itemAdded";
            }
            /// The `ak.wwise.core.object` WAAPI namespace.
            pub mod object {
                /// Sent when an attenuation curve is changed.
                ///
                /// WAAPI URI: `ak.wwise.core.object.attenuationCurveChanged`.
                pub const attenuationCurveChanged: &str =
                    "ak.wwise.core.object.attenuationCurveChanged";
                /// Sent when an attenuation curve's link/unlink is changed.
                ///
                /// WAAPI URI: `ak.wwise.core.object.attenuationCurveLinkChanged`.
                pub const attenuationCurveLinkChanged: &str =
                    "ak.wwise.core.object.attenuationCurveLinkChanged";
                /// Sent when an object is added as a child to another object. See `ak.wwise.core.object.structureChanged` to receive changes in batch.
                ///
                /// WAAPI URI: `ak.wwise.core.object.childAdded`.
                pub const childAdded: &str = "ak.wwise.core.object.childAdded";
                /// Sent when an object is removed from the children of another object. See `ak.wwise.core.object.structureChanged` to receive changes in batch.
                ///
                /// WAAPI URI: `ak.wwise.core.object.childRemoved`.
                pub const childRemoved: &str = "ak.wwise.core.object.childRemoved";
                /// Sent when an object is created. See `ak.wwise.core.object.structureChanged` to receive changes in batch.
                ///
                /// WAAPI URI: `ak.wwise.core.object.created`.
                pub const created: &str = "ak.wwise.core.object.created";
                /// Sent when one or many curves are changed.
                ///
                /// WAAPI URI: `ak.wwise.core.object.curveChanged`.
                pub const curveChanged: &str = "ak.wwise.core.object.curveChanged";
                /// Sent when an object is renamed. Publishes the renamed object. See `ak.wwise.core.object.structureChanged` to receive changes in batch.
                ///
                /// WAAPI URI: `ak.wwise.core.object.nameChanged`.
                pub const nameChanged: &str = "ak.wwise.core.object.nameChanged";
                /// Sent when the object's notes are changed.
                ///
                /// WAAPI URI: `ak.wwise.core.object.notesChanged`.
                pub const notesChanged: &str = "ak.wwise.core.object.notesChanged";
                /// Sent following an object's deletion. See `ak.wwise.core.object.structureChanged` to receive changes in batch.
                ///
                /// WAAPI URI: `ak.wwise.core.object.postDeleted`.
                pub const postDeleted: &str = "ak.wwise.core.object.postDeleted";
                /// Sent prior to an object's deletion. See `ak.wwise.core.object.structureChanged` to receive changes in batch.
                ///
                /// WAAPI URI: `ak.wwise.core.object.preDeleted`.
                pub const preDeleted: &str = "ak.wwise.core.object.preDeleted";
                /// Sent when the watched property of an object changes.
                ///
                /// WAAPI URI: `ak.wwise.core.object.propertyChanged`.
                pub const propertyChanged: &str = "ak.wwise.core.object.propertyChanged";
                /// Sent when an object reference is changed.
                ///
                /// WAAPI URI: `ak.wwise.core.object.referenceChanged`.
                pub const referenceChanged: &str = "ak.wwise.core.object.referenceChanged";
                /// Sent when the project structure changes. Publishes a summary of the changes. The publication is sent after the undo stack is modified. The changes are grouped in batch and redundant changes are collapsed.
                ///
                /// WAAPI URI: `ak.wwise.core.object.structureChanged`.
                pub const structureChanged: &str = "ak.wwise.core.object.structureChanged";
            }
            /// The `ak.wwise.core.profiler` WAAPI namespace.
            pub mod profiler {
                /// Sent when a game object has been registered.
                ///
                /// WAAPI URI: `ak.wwise.core.profiler.gameObjectRegistered`.
                pub const gameObjectRegistered: &str =
                    "ak.wwise.core.profiler.gameObjectRegistered";
                /// Sent when the game objects have been reset, such as closing a connection to a game while profiling.
                ///
                /// WAAPI URI: `ak.wwise.core.profiler.gameObjectReset`.
                pub const gameObjectReset: &str = "ak.wwise.core.profiler.gameObjectReset";
                /// Sent when a game object has been unregistered.
                ///
                /// WAAPI URI: `ak.wwise.core.profiler.gameObjectUnregistered`.
                pub const gameObjectUnregistered: &str =
                    "ak.wwise.core.profiler.gameObjectUnregistered";
                /// Sent when a state group state has been changed. This subscription does not require the profiler capture log to be started.
                ///
                /// WAAPI URI: `ak.wwise.core.profiler.stateChanged`.
                pub const stateChanged: &str = "ak.wwise.core.profiler.stateChanged";
                /// Sent when a switch group state has been changed. This function does not require the profiler capture log to be started.
                ///
                /// WAAPI URI: `ak.wwise.core.profiler.switchChanged`.
                pub const switchChanged: &str = "ak.wwise.core.profiler.switchChanged";

                /// The `ak.wwise.core.profiler.captureLog` WAAPI namespace.
                pub mod captureLog {
                    /// Sent when a new entry is added to the capture log. Note that all entries are being sent. No filtering is applied as opposed to the Capture Log view.
                    ///
                    /// WAAPI URI: `ak.wwise.core.profiler.captureLog.itemAdded`.
                    pub const itemAdded: &str = "ak.wwise.core.profiler.captureLog.itemAdded";
                }
            }
            /// The `ak.wwise.core.project` WAAPI namespace.
            pub mod project {
                /// Sent when the project has been successfully loaded.
                ///
                /// WAAPI URI: `ak.wwise.core.project.loaded`.
                pub const loaded: &str = "ak.wwise.core.project.loaded";
                /// Sent when the after the project is completely closed.
                ///
                /// WAAPI URI: `ak.wwise.core.project.postClosed`.
                pub const postClosed: &str = "ak.wwise.core.project.postClosed";
                /// Sent when the project begins closing.
                ///
                /// WAAPI URI: `ak.wwise.core.project.preClosed`.
                pub const preClosed: &str = "ak.wwise.core.project.preClosed";
                /// Sent when the project has been saved.
                ///
                /// WAAPI URI: `ak.wwise.core.project.saved`.
                pub const saved: &str = "ak.wwise.core.project.saved";
            }
            /// The `ak.wwise.core.soundbank` WAAPI namespace.
            pub mod soundbank {
                /// Sent when a single SoundBank is generated. This could be sent multiple times during SoundBank generation, for every SoundBank generated and for every platform. To generate SoundBanks, see `ak.wwise.core.soundbank.generate` or `ak.wwise.ui.commands.execute` with one of the SoundBank generation commands. See the Wwise global command IDs reference for the list of commands.
                ///
                /// WAAPI URI: `ak.wwise.core.soundbank.generated`.
                pub const generated: &str = "ak.wwise.core.soundbank.generated";
                /// Sent when all SoundBanks are generated. Note: This notification is only sent when SoundBanks have been generated, it is not a reliable way to determine when `ak.wwise.core.soundbank.generate` has completed.
                ///
                /// WAAPI URI: `ak.wwise.core.soundbank.generationDone`.
                pub const generationDone: &str = "ak.wwise.core.soundbank.generationDone";
            }
            /// The `ak.wwise.core.switchContainer` WAAPI namespace.
            pub mod switchContainer {
                /// Sent when an assignment is added to a Switch Container.
                ///
                /// WAAPI URI: `ak.wwise.core.switchContainer.assignmentAdded`.
                pub const assignmentAdded: &str = "ak.wwise.core.switchContainer.assignmentAdded";
                /// Sent when an assignment is removed from a Switch Container.
                ///
                /// WAAPI URI: `ak.wwise.core.switchContainer.assignmentRemoved`.
                pub const assignmentRemoved: &str =
                    "ak.wwise.core.switchContainer.assignmentRemoved";
            }
            /// The `ak.wwise.core.transport` WAAPI namespace.
            pub mod transport {
                /// Sent when the transport's state has changed.
                ///
                /// WAAPI URI: `ak.wwise.core.transport.stateChanged`.
                pub const stateChanged: &str = "ak.wwise.core.transport.stateChanged";
            }
        }
        /// The `ak.wwise.debug` WAAPI namespace.
        pub mod debug {
            /// Sent when an assert has failed. This is only available in Debug builds.
            ///
            /// WAAPI URI: `ak.wwise.debug.assertFailed`.
            pub const assertFailed: &str = "ak.wwise.debug.assertFailed";
        }
        /// The `ak.wwise.ui` WAAPI namespace.
        pub mod ui {
            /// Sent when the selection changes in the project.
            ///
            /// WAAPI URI: `ak.wwise.ui.selectionChanged`.
            pub const selectionChanged: &str = "ak.wwise.ui.selectionChanged";

            /// The `ak.wwise.ui.commands` WAAPI namespace.
            pub mod commands {
                /// Sent when a command is executed. The objects for which the command is executed are sent in the publication.
                ///
                /// WAAPI URI: `ak.wwise.ui.commands.executed`.
                pub const executed: &str = "ak.wwise.ui.commands.executed";
            }

            /// The `ak.wwise.ui.signal` WAAPI namespace.
            pub mod signal {
                /// Emitted when a push button or a radio button gets clicked.
                ///
                /// WAAPI URI: `ak.wwise.ui.signal.click`.
                pub const click: &str = "ak.wwise.ui.signal.click";
                /// Emitted when a toggle button or a checkbox gets toggled.
                ///
                /// WAAPI URI: `ak.wwise.ui.signal.toggle`.
                pub const toggle: &str = "ak.wwise.ui.signal.toggle";
            }
        }
    }
}

/// Typed URI enum for `WaapiTopic` endpoints.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WaapiTopic {
    /// Sent at the end of an import operation.
    ///
    /// WAAPI URI: `ak.wwise.core.audio.imported`.
    ak_wwise_core_audio_imported,
    /// Sent when an item is added to the log. This could be used to retrieve items added to the SoundBank generation log. To retrieve the complete log, see `ak.wwise.core.log.get`.
    ///
    /// WAAPI URI: `ak.wwise.core.log.itemAdded`.
    ak_wwise_core_log_itemAdded,
    /// Sent when an attenuation curve is changed.
    ///
    /// WAAPI URI: `ak.wwise.core.object.attenuationCurveChanged`.
    ak_wwise_core_object_attenuationCurveChanged,
    /// Sent when an attenuation curve's link/unlink is changed.
    ///
    /// WAAPI URI: `ak.wwise.core.object.attenuationCurveLinkChanged`.
    ak_wwise_core_object_attenuationCurveLinkChanged,
    /// Sent when an object is added as a child to another object. See `ak.wwise.core.object.structureChanged` to receive changes in batch.
    ///
    /// WAAPI URI: `ak.wwise.core.object.childAdded`.
    ak_wwise_core_object_childAdded,
    /// Sent when an object is removed from the children of another object. See `ak.wwise.core.object.structureChanged` to receive changes in batch.
    ///
    /// WAAPI URI: `ak.wwise.core.object.childRemoved`.
    ak_wwise_core_object_childRemoved,
    /// Sent when an object is created. See `ak.wwise.core.object.structureChanged` to receive changes in batch.
    ///
    /// WAAPI URI: `ak.wwise.core.object.created`.
    ak_wwise_core_object_created,
    /// Sent when one or many curves are changed.
    ///
    /// WAAPI URI: `ak.wwise.core.object.curveChanged`.
    ak_wwise_core_object_curveChanged,
    /// Sent when an object is renamed. Publishes the renamed object. See `ak.wwise.core.object.structureChanged` to receive changes in batch.
    ///
    /// WAAPI URI: `ak.wwise.core.object.nameChanged`.
    ak_wwise_core_object_nameChanged,
    /// Sent when the object's notes are changed.
    ///
    /// WAAPI URI: `ak.wwise.core.object.notesChanged`.
    ak_wwise_core_object_notesChanged,
    /// Sent following an object's deletion. See `ak.wwise.core.object.structureChanged` to receive changes in batch.
    ///
    /// WAAPI URI: `ak.wwise.core.object.postDeleted`.
    ak_wwise_core_object_postDeleted,
    /// Sent prior to an object's deletion. See `ak.wwise.core.object.structureChanged` to receive changes in batch.
    ///
    /// WAAPI URI: `ak.wwise.core.object.preDeleted`.
    ak_wwise_core_object_preDeleted,
    /// Sent when the watched property of an object changes.
    ///
    /// WAAPI URI: `ak.wwise.core.object.propertyChanged`.
    ak_wwise_core_object_propertyChanged,
    /// Sent when an object reference is changed.
    ///
    /// WAAPI URI: `ak.wwise.core.object.referenceChanged`.
    ak_wwise_core_object_referenceChanged,
    /// Sent when the project structure changes. Publishes a summary of the changes. The publication is sent after the undo stack is modified. The changes are grouped in batch and redundant changes are collapsed.
    ///
    /// WAAPI URI: `ak.wwise.core.object.structureChanged`.
    ak_wwise_core_object_structureChanged,
    /// Sent when a new entry is added to the capture log. Note that all entries are being sent. No filtering is applied as opposed to the Capture Log view.
    ///
    /// WAAPI URI: `ak.wwise.core.profiler.captureLog.itemAdded`.
    ak_wwise_core_profiler_captureLog_itemAdded,
    /// Sent when a game object has been registered.
    ///
    /// WAAPI URI: `ak.wwise.core.profiler.gameObjectRegistered`.
    ak_wwise_core_profiler_gameObjectRegistered,
    /// Sent when the game objects have been reset, such as closing a connection to a game while profiling.
    ///
    /// WAAPI URI: `ak.wwise.core.profiler.gameObjectReset`.
    ak_wwise_core_profiler_gameObjectReset,
    /// Sent when a game object has been unregistered.
    ///
    /// WAAPI URI: `ak.wwise.core.profiler.gameObjectUnregistered`.
    ak_wwise_core_profiler_gameObjectUnregistered,
    /// Sent when a state group state has been changed. This subscription does not require the profiler capture log to be started.
    ///
    /// WAAPI URI: `ak.wwise.core.profiler.stateChanged`.
    ak_wwise_core_profiler_stateChanged,
    /// Sent when a switch group state has been changed. This function does not require the profiler capture log to be started.
    ///
    /// WAAPI URI: `ak.wwise.core.profiler.switchChanged`.
    ak_wwise_core_profiler_switchChanged,
    /// Sent when the project has been successfully loaded.
    ///
    /// WAAPI URI: `ak.wwise.core.project.loaded`.
    ak_wwise_core_project_loaded,
    /// Sent when the after the project is completely closed.
    ///
    /// WAAPI URI: `ak.wwise.core.project.postClosed`.
    ak_wwise_core_project_postClosed,
    /// Sent when the project begins closing.
    ///
    /// WAAPI URI: `ak.wwise.core.project.preClosed`.
    ak_wwise_core_project_preClosed,
    /// Sent when the project has been saved.
    ///
    /// WAAPI URI: `ak.wwise.core.project.saved`.
    ak_wwise_core_project_saved,
    /// Sent when a single SoundBank is generated. This could be sent multiple times during SoundBank generation, for every SoundBank generated and for every platform. To generate SoundBanks, see `ak.wwise.core.soundbank.generate` or `ak.wwise.ui.commands.execute` with one of the SoundBank generation commands. See the Wwise global command IDs reference for the list of commands.
    ///
    /// WAAPI URI: `ak.wwise.core.soundbank.generated`.
    ak_wwise_core_soundbank_generated,
    /// Sent when all SoundBanks are generated. Note: This notification is only sent when SoundBanks have been generated, it is not a reliable way to determine when `ak.wwise.core.soundbank.generate` has completed.
    ///
    /// WAAPI URI: `ak.wwise.core.soundbank.generationDone`.
    ak_wwise_core_soundbank_generationDone,
    /// Sent when an assignment is added to a Switch Container.
    ///
    /// WAAPI URI: `ak.wwise.core.switchContainer.assignmentAdded`.
    ak_wwise_core_switchContainer_assignmentAdded,
    /// Sent when an assignment is removed from a Switch Container.
    ///
    /// WAAPI URI: `ak.wwise.core.switchContainer.assignmentRemoved`.
    ak_wwise_core_switchContainer_assignmentRemoved,
    /// Sent when the transport's state has changed.
    ///
    /// WAAPI URI: `ak.wwise.core.transport.stateChanged`.
    ak_wwise_core_transport_stateChanged,
    /// Sent when an assert has failed. This is only available in Debug builds.
    ///
    /// WAAPI URI: `ak.wwise.debug.assertFailed`.
    ak_wwise_debug_assertFailed,
    /// Sent when a command is executed. The objects for which the command is executed are sent in the publication.
    ///
    /// WAAPI URI: `ak.wwise.ui.commands.executed`.
    ak_wwise_ui_commands_executed,
    /// Sent when the selection changes in the project.
    ///
    /// WAAPI URI: `ak.wwise.ui.selectionChanged`.
    ak_wwise_ui_selectionChanged,
    /// Emitted when a push button or a radio button gets clicked.
    ///
    /// WAAPI URI: `ak.wwise.ui.signal.click`.
    ak_wwise_ui_signal_click,
    /// Emitted when a toggle button or a checkbox gets toggled.
    ///
    /// WAAPI URI: `ak.wwise.ui.signal.toggle`.
    ak_wwise_ui_signal_toggle,
}

impl WaapiTopic {
    /// Returns the WAAPI URI string for this endpoint.
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
            Self::ak_wwise_core_object_structureChanged => {
                ak::wwise::core::object::structureChanged
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
