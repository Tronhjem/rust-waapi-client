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
