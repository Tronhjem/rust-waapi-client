/// Wwise Authoring API endpoints
#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
pub mod ak {
    pub mod soundengine {
        pub const executeActionOnEvent: &str = "ak.soundengine.executeActionOnEvent";
        pub const getState: &str = "ak.soundengine.getState";
        pub const getSwitch: &str = "ak.soundengine.getSwitch";
        pub const loadBank: &str = "ak.soundengine.loadBank";
        pub const postEvent: &str = "ak.soundengine.postEvent";
        pub const postMsgMonitor: &str = "ak.soundengine.postMsgMonitor";
        pub const postTrigger: &str = "ak.soundengine.postTrigger";
        pub const registerGameObj: &str = "ak.soundengine.registerGameObj";
        pub const resetRTPCValue: &str = "ak.soundengine.resetRTPCValue";
        pub const seekOnEvent: &str = "ak.soundengine.seekOnEvent";
        pub const setDefaultListeners: &str = "ak.soundengine.setDefaultListeners";
        pub const setGameObjectAuxSendValues: &str = "ak.soundengine.setGameObjectAuxSendValues";
        pub const setGameObjectOutputBusVolume: &str =
            "ak.soundengine.setGameObjectOutputBusVolume";
        pub const setListenerSpatialization: &str = "ak.soundengine.setListenerSpatialization";
        pub const setListeners: &str = "ak.soundengine.setListeners";
        pub const setMultiplePositions: &str = "ak.soundengine.setMultiplePositions";
        pub const setObjectObstructionAndOcclusion: &str =
            "ak.soundengine.setObjectObstructionAndOcclusion";
        pub const setPosition: &str = "ak.soundengine.setPosition";
        pub const setRTPCValue: &str = "ak.soundengine.setRTPCValue";
        pub const setScalingFactor: &str = "ak.soundengine.setScalingFactor";
        pub const setState: &str = "ak.soundengine.setState";
        pub const setSwitch: &str = "ak.soundengine.setSwitch";
        pub const stopAll: &str = "ak.soundengine.stopAll";
        pub const stopPlayingID: &str = "ak.soundengine.stopPlayingID";
        pub const unloadBank: &str = "ak.soundengine.unloadBank";
        pub const unregisterGameObj: &str = "ak.soundengine.unregisterGameObj";
    }
    pub mod wwise {
        pub mod core {
            pub const executeLuaScript: &str = "ak.wwise.core.executeLuaScript";
            pub const getInfo: &str = "ak.wwise.core.getInfo";
            pub const getProjectInfo: &str = "ak.wwise.core.getProjectInfo";
            pub const ping: &str = "ak.wwise.core.ping";

            pub mod audio {
                pub const convert: &str = "ak.wwise.core.audio.convert";
                pub const import: &str = "ak.wwise.core.audio.import";
                pub const importTabDelimited: &str = "ak.wwise.core.audio.importTabDelimited";
                pub const mute: &str = "ak.wwise.core.audio.mute";
                pub const resetMute: &str = "ak.wwise.core.audio.resetMute";
                pub const resetSolo: &str = "ak.wwise.core.audio.resetSolo";
                pub const setConversionPlugin: &str = "ak.wwise.core.audio.setConversionPlugin";
                pub const solo: &str = "ak.wwise.core.audio.solo";
            }

            pub mod audioSourcePeaks {
                pub const getMinMaxPeaksInRegion: &str =
                    "ak.wwise.core.audioSourcePeaks.getMinMaxPeaksInRegion";
                pub const getMinMaxPeaksInTrimmedRegion: &str =
                    "ak.wwise.core.audioSourcePeaks.getMinMaxPeaksInTrimmedRegion";
            }

            pub mod blendContainer {
                pub const addAssignment: &str = "ak.wwise.core.blendContainer.addAssignment";
                pub const addTrack: &str = "ak.wwise.core.blendContainer.addTrack";
                pub const getAssignments: &str = "ak.wwise.core.blendContainer.getAssignments";
                pub const removeAssignment: &str = "ak.wwise.core.blendContainer.removeAssignment";
            }

            pub mod gameParameter {
                pub const setRange: &str = "ak.wwise.core.gameParameter.setRange";
            }

            pub mod log {
                pub const addItem: &str = "ak.wwise.core.log.addItem";
                pub const clear: &str = "ak.wwise.core.log.clear";
                pub const get: &str = "ak.wwise.core.log.get";
            }

            pub mod object {
                pub const copy: &str = "ak.wwise.core.object.copy";
                pub const create: &str = "ak.wwise.core.object.create";
                pub const delete: &str = "ak.wwise.core.object.delete";
                pub const diff: &str = "ak.wwise.core.object.diff";
                pub const get: &str = "ak.wwise.core.object.get";
                pub const getAttenuationCurve: &str = "ak.wwise.core.object.getAttenuationCurve";
                pub const getPropertyAndReferenceNames: &str =
                    "ak.wwise.core.object.getPropertyAndReferenceNames";
                pub const getPropertyInfo: &str = "ak.wwise.core.object.getPropertyInfo";
                pub const getTypes: &str = "ak.wwise.core.object.getTypes";
                pub const isLinked: &str = "ak.wwise.core.object.isLinked";
                pub const isPropertyEnabled: &str = "ak.wwise.core.object.isPropertyEnabled";
                pub const move_: &str = "ak.wwise.core.object.move";
                pub const pasteProperties: &str = "ak.wwise.core.object.pasteProperties";
                pub const set: &str = "ak.wwise.core.object.set";
                pub const setAttenuationCurve: &str = "ak.wwise.core.object.setAttenuationCurve";
                pub const setLinked: &str = "ak.wwise.core.object.setLinked";
                pub const setName: &str = "ak.wwise.core.object.setName";
                pub const setNotes: &str = "ak.wwise.core.object.setNotes";
                pub const setProperty: &str = "ak.wwise.core.object.setProperty";
                pub const setRandomizer: &str = "ak.wwise.core.object.setRandomizer";
                pub const setReference: &str = "ak.wwise.core.object.setReference";
                pub const setStateGroups: &str = "ak.wwise.core.object.setStateGroups";
                pub const setStateProperties: &str = "ak.wwise.core.object.setStateProperties";
            }

            pub mod profiler {
                pub const enableProfilerData: &str = "ak.wwise.core.profiler.enableProfilerData";
                pub const getAudioObjects: &str = "ak.wwise.core.profiler.getAudioObjects";
                pub const getBusses: &str = "ak.wwise.core.profiler.getBusses";
                pub const getCpuUsage: &str = "ak.wwise.core.profiler.getCpuUsage";
                pub const getCursorTime: &str = "ak.wwise.core.profiler.getCursorTime";
                pub const getGameObjects: &str = "ak.wwise.core.profiler.getGameObjects";
                pub const getLoadedMedia: &str = "ak.wwise.core.profiler.getLoadedMedia";
                pub const getMeters: &str = "ak.wwise.core.profiler.getMeters";
                pub const getPerformanceMonitor: &str =
                    "ak.wwise.core.profiler.getPerformanceMonitor";
                pub const getRTPCs: &str = "ak.wwise.core.profiler.getRTPCs";
                pub const getStreamedMedia: &str = "ak.wwise.core.profiler.getStreamedMedia";
                pub const getVoiceContributions: &str =
                    "ak.wwise.core.profiler.getVoiceContributions";
                pub const getVoices: &str = "ak.wwise.core.profiler.getVoices";
                pub const registerMeter: &str = "ak.wwise.core.profiler.registerMeter";
                pub const saveCapture: &str = "ak.wwise.core.profiler.saveCapture";
                pub const startCapture: &str = "ak.wwise.core.profiler.startCapture";
                pub const stopCapture: &str = "ak.wwise.core.profiler.stopCapture";
                pub const unregisterMeter: &str = "ak.wwise.core.profiler.unregisterMeter";
            }

            pub mod project {
                pub const save: &str = "ak.wwise.core.project.save";
            }

            pub mod remote {
                pub const connect: &str = "ak.wwise.core.remote.connect";
                pub const disconnect: &str = "ak.wwise.core.remote.disconnect";
                pub const getAvailableConsoles: &str = "ak.wwise.core.remote.getAvailableConsoles";
                pub const getConnectionStatus: &str = "ak.wwise.core.remote.getConnectionStatus";
            }

            pub mod sound {
                pub const setActiveSource: &str = "ak.wwise.core.sound.setActiveSource";
            }

            pub mod soundbank {
                pub const convertExternalSources: &str =
                    "ak.wwise.core.soundbank.convertExternalSources";
                pub const generate: &str = "ak.wwise.core.soundbank.generate";
                pub const getInclusions: &str = "ak.wwise.core.soundbank.getInclusions";
                pub const processDefinitionFiles: &str =
                    "ak.wwise.core.soundbank.processDefinitionFiles";
                pub const setInclusions: &str = "ak.wwise.core.soundbank.setInclusions";
            }

            pub mod sourceControl {
                pub const add: &str = "ak.wwise.core.sourceControl.add";
                pub const checkOut: &str = "ak.wwise.core.sourceControl.checkOut";
                pub const commit: &str = "ak.wwise.core.sourceControl.commit";
                pub const delete: &str = "ak.wwise.core.sourceControl.delete";
                pub const getSourceFiles: &str = "ak.wwise.core.sourceControl.getSourceFiles";
                pub const getStatus: &str = "ak.wwise.core.sourceControl.getStatus";
                pub const move_: &str = "ak.wwise.core.sourceControl.move";
                pub const revert: &str = "ak.wwise.core.sourceControl.revert";
                pub const setProvider: &str = "ak.wwise.core.sourceControl.setProvider";
            }

            pub mod switchContainer {
                pub const addAssignment: &str = "ak.wwise.core.switchContainer.addAssignment";
                pub const getAssignments: &str = "ak.wwise.core.switchContainer.getAssignments";
                pub const removeAssignment: &str = "ak.wwise.core.switchContainer.removeAssignment";
            }

            pub mod transport {
                pub const create: &str = "ak.wwise.core.transport.create";
                pub const destroy: &str = "ak.wwise.core.transport.destroy";
                pub const executeAction: &str = "ak.wwise.core.transport.executeAction";
                pub const getList: &str = "ak.wwise.core.transport.getList";
                pub const getState: &str = "ak.wwise.core.transport.getState";
                pub const prepare: &str = "ak.wwise.core.transport.prepare";
                pub const useOriginals: &str = "ak.wwise.core.transport.useOriginals";
            }

            pub mod undo {
                pub const beginGroup: &str = "ak.wwise.core.undo.beginGroup";
                pub const cancelGroup: &str = "ak.wwise.core.undo.cancelGroup";
                pub const endGroup: &str = "ak.wwise.core.undo.endGroup";
                pub const redo: &str = "ak.wwise.core.undo.redo";
                pub const undo: &str = "ak.wwise.core.undo.undo";
            }
        }
        pub mod debug {
            pub const enableAsserts: &str = "ak.wwise.debug.enableAsserts";
            pub const enableAutomationMode: &str = "ak.wwise.debug.enableAutomationMode";
            pub const generateToneWAV: &str = "ak.wwise.debug.generateToneWAV";
            pub const getWalTree: &str = "ak.wwise.debug.getWalTree";
            pub const restartWaapiServers: &str = "ak.wwise.debug.restartWaapiServers";
            pub const testAssert: &str = "ak.wwise.debug.testAssert";
            pub const testCrash: &str = "ak.wwise.debug.testCrash";
            pub const validateCall: &str = "ak.wwise.debug.validateCall";
        }
        pub mod ui {
            pub const bringToForeground: &str = "ak.wwise.ui.bringToForeground";
            pub const captureScreen: &str = "ak.wwise.ui.captureScreen";
            pub const getSelectedObjects: &str = "ak.wwise.ui.getSelectedObjects";

            pub mod cli {
                pub const executeLuaScript: &str = "ak.wwise.ui.cli.executeLuaScript";
                pub const launch: &str = "ak.wwise.ui.cli.launch";
            }

            pub mod commands {
                pub const execute: &str = "ak.wwise.ui.commands.execute";
                pub const getCommands: &str = "ak.wwise.ui.commands.getCommands";
                pub const register: &str = "ak.wwise.ui.commands.register";
                pub const unregister: &str = "ak.wwise.ui.commands.unregister";
            }

            pub mod layout {
                pub const dockView: &str = "ak.wwise.ui.layout.dockView";
                pub const getCurrentLayoutName: &str = "ak.wwise.ui.layout.getCurrentLayoutName";
                pub const getElementRectangle: &str = "ak.wwise.ui.layout.getElementRectangle";
                pub const getLayout: &str = "ak.wwise.ui.layout.getLayout";
                pub const getLayoutNames: &str = "ak.wwise.ui.layout.getLayoutNames";
                pub const getOrCreateView: &str = "ak.wwise.ui.layout.getOrCreateView";
                pub const getViewHandle: &str = "ak.wwise.ui.layout.getViewHandle";
                pub const getViewInstances: &str = "ak.wwise.ui.layout.getViewInstances";
                pub const getViewTypes: &str = "ak.wwise.ui.layout.getViewTypes";
                pub const moveSplitter: &str = "ak.wwise.ui.layout.moveSplitter";
                pub const removeLayout: &str = "ak.wwise.ui.layout.removeLayout";
                pub const setLayout: &str = "ak.wwise.ui.layout.setLayout";
                pub const switchLayout: &str = "ak.wwise.ui.layout.switchLayout";
                pub const undockView: &str = "ak.wwise.ui.layout.undockView";
            }

            pub mod model {
                pub const createHandle: &str = "ak.wwise.ui.model.createHandle";
                pub const destroyHandle: &str = "ak.wwise.ui.model.destroyHandle";
                pub const getState: &str = "ak.wwise.ui.model.getState";
                pub const registerWafm: &str = "ak.wwise.ui.model.registerWafm";
                pub const setState: &str = "ak.wwise.ui.model.setState";
            }

            pub mod project {
                pub const close: &str = "ak.wwise.ui.project.close";
                pub const create: &str = "ak.wwise.ui.project.create";
                pub const open: &str = "ak.wwise.ui.project.open";
            }

            pub mod signal {
                pub const emit: &str = "ak.wwise.ui.signal.emit";
            }

            pub mod window {
                pub const close: &str = "ak.wwise.ui.window.close";
                pub const create: &str = "ak.wwise.ui.window.create";
                pub const present: &str = "ak.wwise.ui.window.present";
            }
        }
        pub mod waapi {
            pub const getFunctions: &str = "ak.wwise.waapi.getFunctions";
            pub const getSchema: &str = "ak.wwise.waapi.getSchema";
            pub const getTopics: &str = "ak.wwise.waapi.getTopics";
        }
    }
}

/// Typed URI enum for `WaapiFunction` endpoints.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WaapiFunction {
    /// `ak.soundengine.executeActionOnEvent`
    ak_soundengine_executeActionOnEvent,
    /// `ak.soundengine.getState`
    ak_soundengine_getState,
    /// `ak.soundengine.getSwitch`
    ak_soundengine_getSwitch,
    /// `ak.soundengine.loadBank`
    ak_soundengine_loadBank,
    /// `ak.soundengine.postEvent`
    ak_soundengine_postEvent,
    /// `ak.soundengine.postMsgMonitor`
    ak_soundengine_postMsgMonitor,
    /// `ak.soundengine.postTrigger`
    ak_soundengine_postTrigger,
    /// `ak.soundengine.registerGameObj`
    ak_soundengine_registerGameObj,
    /// `ak.soundengine.resetRTPCValue`
    ak_soundengine_resetRTPCValue,
    /// `ak.soundengine.seekOnEvent`
    ak_soundengine_seekOnEvent,
    /// `ak.soundengine.setDefaultListeners`
    ak_soundengine_setDefaultListeners,
    /// `ak.soundengine.setGameObjectAuxSendValues`
    ak_soundengine_setGameObjectAuxSendValues,
    /// `ak.soundengine.setGameObjectOutputBusVolume`
    ak_soundengine_setGameObjectOutputBusVolume,
    /// `ak.soundengine.setListenerSpatialization`
    ak_soundengine_setListenerSpatialization,
    /// `ak.soundengine.setListeners`
    ak_soundengine_setListeners,
    /// `ak.soundengine.setMultiplePositions`
    ak_soundengine_setMultiplePositions,
    /// `ak.soundengine.setObjectObstructionAndOcclusion`
    ak_soundengine_setObjectObstructionAndOcclusion,
    /// `ak.soundengine.setPosition`
    ak_soundengine_setPosition,
    /// `ak.soundengine.setRTPCValue`
    ak_soundengine_setRTPCValue,
    /// `ak.soundengine.setScalingFactor`
    ak_soundengine_setScalingFactor,
    /// `ak.soundengine.setState`
    ak_soundengine_setState,
    /// `ak.soundengine.setSwitch`
    ak_soundengine_setSwitch,
    /// `ak.soundengine.stopAll`
    ak_soundengine_stopAll,
    /// `ak.soundengine.stopPlayingID`
    ak_soundengine_stopPlayingID,
    /// `ak.soundengine.unloadBank`
    ak_soundengine_unloadBank,
    /// `ak.soundengine.unregisterGameObj`
    ak_soundengine_unregisterGameObj,
    /// `ak.wwise.core.audio.convert`
    ak_wwise_core_audio_convert,
    /// `ak.wwise.core.audio.import`
    ak_wwise_core_audio_import,
    /// `ak.wwise.core.audio.importTabDelimited`
    ak_wwise_core_audio_importTabDelimited,
    /// `ak.wwise.core.audio.mute`
    ak_wwise_core_audio_mute,
    /// `ak.wwise.core.audio.resetMute`
    ak_wwise_core_audio_resetMute,
    /// `ak.wwise.core.audio.resetSolo`
    ak_wwise_core_audio_resetSolo,
    /// `ak.wwise.core.audio.setConversionPlugin`
    ak_wwise_core_audio_setConversionPlugin,
    /// `ak.wwise.core.audio.solo`
    ak_wwise_core_audio_solo,
    /// `ak.wwise.core.audioSourcePeaks.getMinMaxPeaksInRegion`
    ak_wwise_core_audioSourcePeaks_getMinMaxPeaksInRegion,
    /// `ak.wwise.core.audioSourcePeaks.getMinMaxPeaksInTrimmedRegion`
    ak_wwise_core_audioSourcePeaks_getMinMaxPeaksInTrimmedRegion,
    /// `ak.wwise.core.blendContainer.addAssignment`
    ak_wwise_core_blendContainer_addAssignment,
    /// `ak.wwise.core.blendContainer.addTrack`
    ak_wwise_core_blendContainer_addTrack,
    /// `ak.wwise.core.blendContainer.getAssignments`
    ak_wwise_core_blendContainer_getAssignments,
    /// `ak.wwise.core.blendContainer.removeAssignment`
    ak_wwise_core_blendContainer_removeAssignment,
    /// `ak.wwise.core.executeLuaScript`
    ak_wwise_core_executeLuaScript,
    /// `ak.wwise.core.gameParameter.setRange`
    ak_wwise_core_gameParameter_setRange,
    /// `ak.wwise.core.getInfo`
    ak_wwise_core_getInfo,
    /// `ak.wwise.core.getProjectInfo`
    ak_wwise_core_getProjectInfo,
    /// `ak.wwise.core.log.addItem`
    ak_wwise_core_log_addItem,
    /// `ak.wwise.core.log.clear`
    ak_wwise_core_log_clear,
    /// `ak.wwise.core.log.get`
    ak_wwise_core_log_get,
    /// `ak.wwise.core.object.copy`
    ak_wwise_core_object_copy,
    /// `ak.wwise.core.object.create`
    ak_wwise_core_object_create,
    /// `ak.wwise.core.object.delete`
    ak_wwise_core_object_delete,
    /// `ak.wwise.core.object.diff`
    ak_wwise_core_object_diff,
    /// `ak.wwise.core.object.get`
    ak_wwise_core_object_get,
    /// `ak.wwise.core.object.getAttenuationCurve`
    ak_wwise_core_object_getAttenuationCurve,
    /// `ak.wwise.core.object.getPropertyAndReferenceNames`
    ak_wwise_core_object_getPropertyAndReferenceNames,
    /// `ak.wwise.core.object.getPropertyInfo`
    ak_wwise_core_object_getPropertyInfo,
    /// `ak.wwise.core.object.getTypes`
    ak_wwise_core_object_getTypes,
    /// `ak.wwise.core.object.isLinked`
    ak_wwise_core_object_isLinked,
    /// `ak.wwise.core.object.isPropertyEnabled`
    ak_wwise_core_object_isPropertyEnabled,
    /// `ak.wwise.core.object.move`
    ak_wwise_core_object_move,
    /// `ak.wwise.core.object.pasteProperties`
    ak_wwise_core_object_pasteProperties,
    /// `ak.wwise.core.object.set`
    ak_wwise_core_object_set,
    /// `ak.wwise.core.object.setAttenuationCurve`
    ak_wwise_core_object_setAttenuationCurve,
    /// `ak.wwise.core.object.setLinked`
    ak_wwise_core_object_setLinked,
    /// `ak.wwise.core.object.setName`
    ak_wwise_core_object_setName,
    /// `ak.wwise.core.object.setNotes`
    ak_wwise_core_object_setNotes,
    /// `ak.wwise.core.object.setProperty`
    ak_wwise_core_object_setProperty,
    /// `ak.wwise.core.object.setRandomizer`
    ak_wwise_core_object_setRandomizer,
    /// `ak.wwise.core.object.setReference`
    ak_wwise_core_object_setReference,
    /// `ak.wwise.core.object.setStateGroups`
    ak_wwise_core_object_setStateGroups,
    /// `ak.wwise.core.object.setStateProperties`
    ak_wwise_core_object_setStateProperties,
    /// `ak.wwise.core.ping`
    ak_wwise_core_ping,
    /// `ak.wwise.core.profiler.enableProfilerData`
    ak_wwise_core_profiler_enableProfilerData,
    /// `ak.wwise.core.profiler.getAudioObjects`
    ak_wwise_core_profiler_getAudioObjects,
    /// `ak.wwise.core.profiler.getBusses`
    ak_wwise_core_profiler_getBusses,
    /// `ak.wwise.core.profiler.getCpuUsage`
    ak_wwise_core_profiler_getCpuUsage,
    /// `ak.wwise.core.profiler.getCursorTime`
    ak_wwise_core_profiler_getCursorTime,
    /// `ak.wwise.core.profiler.getGameObjects`
    ak_wwise_core_profiler_getGameObjects,
    /// `ak.wwise.core.profiler.getLoadedMedia`
    ak_wwise_core_profiler_getLoadedMedia,
    /// `ak.wwise.core.profiler.getMeters`
    ak_wwise_core_profiler_getMeters,
    /// `ak.wwise.core.profiler.getPerformanceMonitor`
    ak_wwise_core_profiler_getPerformanceMonitor,
    /// `ak.wwise.core.profiler.getRTPCs`
    ak_wwise_core_profiler_getRTPCs,
    /// `ak.wwise.core.profiler.getStreamedMedia`
    ak_wwise_core_profiler_getStreamedMedia,
    /// `ak.wwise.core.profiler.getVoiceContributions`
    ak_wwise_core_profiler_getVoiceContributions,
    /// `ak.wwise.core.profiler.getVoices`
    ak_wwise_core_profiler_getVoices,
    /// `ak.wwise.core.profiler.registerMeter`
    ak_wwise_core_profiler_registerMeter,
    /// `ak.wwise.core.profiler.saveCapture`
    ak_wwise_core_profiler_saveCapture,
    /// `ak.wwise.core.profiler.startCapture`
    ak_wwise_core_profiler_startCapture,
    /// `ak.wwise.core.profiler.stopCapture`
    ak_wwise_core_profiler_stopCapture,
    /// `ak.wwise.core.profiler.unregisterMeter`
    ak_wwise_core_profiler_unregisterMeter,
    /// `ak.wwise.core.project.save`
    ak_wwise_core_project_save,
    /// `ak.wwise.core.remote.connect`
    ak_wwise_core_remote_connect,
    /// `ak.wwise.core.remote.disconnect`
    ak_wwise_core_remote_disconnect,
    /// `ak.wwise.core.remote.getAvailableConsoles`
    ak_wwise_core_remote_getAvailableConsoles,
    /// `ak.wwise.core.remote.getConnectionStatus`
    ak_wwise_core_remote_getConnectionStatus,
    /// `ak.wwise.core.sound.setActiveSource`
    ak_wwise_core_sound_setActiveSource,
    /// `ak.wwise.core.soundbank.convertExternalSources`
    ak_wwise_core_soundbank_convertExternalSources,
    /// `ak.wwise.core.soundbank.generate`
    ak_wwise_core_soundbank_generate,
    /// `ak.wwise.core.soundbank.getInclusions`
    ak_wwise_core_soundbank_getInclusions,
    /// `ak.wwise.core.soundbank.processDefinitionFiles`
    ak_wwise_core_soundbank_processDefinitionFiles,
    /// `ak.wwise.core.soundbank.setInclusions`
    ak_wwise_core_soundbank_setInclusions,
    /// `ak.wwise.core.sourceControl.add`
    ak_wwise_core_sourceControl_add,
    /// `ak.wwise.core.sourceControl.checkOut`
    ak_wwise_core_sourceControl_checkOut,
    /// `ak.wwise.core.sourceControl.commit`
    ak_wwise_core_sourceControl_commit,
    /// `ak.wwise.core.sourceControl.delete`
    ak_wwise_core_sourceControl_delete,
    /// `ak.wwise.core.sourceControl.getSourceFiles`
    ak_wwise_core_sourceControl_getSourceFiles,
    /// `ak.wwise.core.sourceControl.getStatus`
    ak_wwise_core_sourceControl_getStatus,
    /// `ak.wwise.core.sourceControl.move`
    ak_wwise_core_sourceControl_move,
    /// `ak.wwise.core.sourceControl.revert`
    ak_wwise_core_sourceControl_revert,
    /// `ak.wwise.core.sourceControl.setProvider`
    ak_wwise_core_sourceControl_setProvider,
    /// `ak.wwise.core.switchContainer.addAssignment`
    ak_wwise_core_switchContainer_addAssignment,
    /// `ak.wwise.core.switchContainer.getAssignments`
    ak_wwise_core_switchContainer_getAssignments,
    /// `ak.wwise.core.switchContainer.removeAssignment`
    ak_wwise_core_switchContainer_removeAssignment,
    /// `ak.wwise.core.transport.create`
    ak_wwise_core_transport_create,
    /// `ak.wwise.core.transport.destroy`
    ak_wwise_core_transport_destroy,
    /// `ak.wwise.core.transport.executeAction`
    ak_wwise_core_transport_executeAction,
    /// `ak.wwise.core.transport.getList`
    ak_wwise_core_transport_getList,
    /// `ak.wwise.core.transport.getState`
    ak_wwise_core_transport_getState,
    /// `ak.wwise.core.transport.prepare`
    ak_wwise_core_transport_prepare,
    /// `ak.wwise.core.transport.useOriginals`
    ak_wwise_core_transport_useOriginals,
    /// `ak.wwise.core.undo.beginGroup`
    ak_wwise_core_undo_beginGroup,
    /// `ak.wwise.core.undo.cancelGroup`
    ak_wwise_core_undo_cancelGroup,
    /// `ak.wwise.core.undo.endGroup`
    ak_wwise_core_undo_endGroup,
    /// `ak.wwise.core.undo.redo`
    ak_wwise_core_undo_redo,
    /// `ak.wwise.core.undo.undo`
    ak_wwise_core_undo_undo,
    /// `ak.wwise.debug.enableAsserts`
    ak_wwise_debug_enableAsserts,
    /// `ak.wwise.debug.enableAutomationMode`
    ak_wwise_debug_enableAutomationMode,
    /// `ak.wwise.debug.generateToneWAV`
    ak_wwise_debug_generateToneWAV,
    /// `ak.wwise.debug.getWalTree`
    ak_wwise_debug_getWalTree,
    /// `ak.wwise.debug.restartWaapiServers`
    ak_wwise_debug_restartWaapiServers,
    /// `ak.wwise.debug.testAssert`
    ak_wwise_debug_testAssert,
    /// `ak.wwise.debug.testCrash`
    ak_wwise_debug_testCrash,
    /// `ak.wwise.debug.validateCall`
    ak_wwise_debug_validateCall,
    /// `ak.wwise.ui.bringToForeground`
    ak_wwise_ui_bringToForeground,
    /// `ak.wwise.ui.captureScreen`
    ak_wwise_ui_captureScreen,
    /// `ak.wwise.ui.cli.executeLuaScript`
    ak_wwise_ui_cli_executeLuaScript,
    /// `ak.wwise.ui.cli.launch`
    ak_wwise_ui_cli_launch,
    /// `ak.wwise.ui.commands.execute`
    ak_wwise_ui_commands_execute,
    /// `ak.wwise.ui.commands.getCommands`
    ak_wwise_ui_commands_getCommands,
    /// `ak.wwise.ui.commands.register`
    ak_wwise_ui_commands_register,
    /// `ak.wwise.ui.commands.unregister`
    ak_wwise_ui_commands_unregister,
    /// `ak.wwise.ui.getSelectedObjects`
    ak_wwise_ui_getSelectedObjects,
    /// `ak.wwise.ui.layout.dockView`
    ak_wwise_ui_layout_dockView,
    /// `ak.wwise.ui.layout.getCurrentLayoutName`
    ak_wwise_ui_layout_getCurrentLayoutName,
    /// `ak.wwise.ui.layout.getElementRectangle`
    ak_wwise_ui_layout_getElementRectangle,
    /// `ak.wwise.ui.layout.getLayout`
    ak_wwise_ui_layout_getLayout,
    /// `ak.wwise.ui.layout.getLayoutNames`
    ak_wwise_ui_layout_getLayoutNames,
    /// `ak.wwise.ui.layout.getOrCreateView`
    ak_wwise_ui_layout_getOrCreateView,
    /// `ak.wwise.ui.layout.getViewHandle`
    ak_wwise_ui_layout_getViewHandle,
    /// `ak.wwise.ui.layout.getViewInstances`
    ak_wwise_ui_layout_getViewInstances,
    /// `ak.wwise.ui.layout.getViewTypes`
    ak_wwise_ui_layout_getViewTypes,
    /// `ak.wwise.ui.layout.moveSplitter`
    ak_wwise_ui_layout_moveSplitter,
    /// `ak.wwise.ui.layout.removeLayout`
    ak_wwise_ui_layout_removeLayout,
    /// `ak.wwise.ui.layout.setLayout`
    ak_wwise_ui_layout_setLayout,
    /// `ak.wwise.ui.layout.switchLayout`
    ak_wwise_ui_layout_switchLayout,
    /// `ak.wwise.ui.layout.undockView`
    ak_wwise_ui_layout_undockView,
    /// `ak.wwise.ui.model.createHandle`
    ak_wwise_ui_model_createHandle,
    /// `ak.wwise.ui.model.destroyHandle`
    ak_wwise_ui_model_destroyHandle,
    /// `ak.wwise.ui.model.getState`
    ak_wwise_ui_model_getState,
    /// `ak.wwise.ui.model.registerWafm`
    ak_wwise_ui_model_registerWafm,
    /// `ak.wwise.ui.model.setState`
    ak_wwise_ui_model_setState,
    /// `ak.wwise.ui.project.close`
    ak_wwise_ui_project_close,
    /// `ak.wwise.ui.project.create`
    ak_wwise_ui_project_create,
    /// `ak.wwise.ui.project.open`
    ak_wwise_ui_project_open,
    /// `ak.wwise.ui.signal.emit`
    ak_wwise_ui_signal_emit,
    /// `ak.wwise.ui.window.close`
    ak_wwise_ui_window_close,
    /// `ak.wwise.ui.window.create`
    ak_wwise_ui_window_create,
    /// `ak.wwise.ui.window.present`
    ak_wwise_ui_window_present,
    /// `ak.wwise.waapi.getFunctions`
    ak_wwise_waapi_getFunctions,
    /// `ak.wwise.waapi.getSchema`
    ak_wwise_waapi_getSchema,
    /// `ak.wwise.waapi.getTopics`
    ak_wwise_waapi_getTopics,
}

impl WaapiFunction {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ak_soundengine_executeActionOnEvent => ak::soundengine::executeActionOnEvent,
            Self::ak_soundengine_getState => ak::soundengine::getState,
            Self::ak_soundengine_getSwitch => ak::soundengine::getSwitch,
            Self::ak_soundengine_loadBank => ak::soundengine::loadBank,
            Self::ak_soundengine_postEvent => ak::soundengine::postEvent,
            Self::ak_soundengine_postMsgMonitor => ak::soundengine::postMsgMonitor,
            Self::ak_soundengine_postTrigger => ak::soundengine::postTrigger,
            Self::ak_soundengine_registerGameObj => ak::soundengine::registerGameObj,
            Self::ak_soundengine_resetRTPCValue => ak::soundengine::resetRTPCValue,
            Self::ak_soundengine_seekOnEvent => ak::soundengine::seekOnEvent,
            Self::ak_soundengine_setDefaultListeners => ak::soundengine::setDefaultListeners,
            Self::ak_soundengine_setGameObjectAuxSendValues => {
                ak::soundengine::setGameObjectAuxSendValues
            }
            Self::ak_soundengine_setGameObjectOutputBusVolume => {
                ak::soundengine::setGameObjectOutputBusVolume
            }
            Self::ak_soundengine_setListenerSpatialization => {
                ak::soundengine::setListenerSpatialization
            }
            Self::ak_soundengine_setListeners => ak::soundengine::setListeners,
            Self::ak_soundengine_setMultiplePositions => ak::soundengine::setMultiplePositions,
            Self::ak_soundengine_setObjectObstructionAndOcclusion => {
                ak::soundengine::setObjectObstructionAndOcclusion
            }
            Self::ak_soundengine_setPosition => ak::soundengine::setPosition,
            Self::ak_soundengine_setRTPCValue => ak::soundengine::setRTPCValue,
            Self::ak_soundengine_setScalingFactor => ak::soundengine::setScalingFactor,
            Self::ak_soundengine_setState => ak::soundengine::setState,
            Self::ak_soundengine_setSwitch => ak::soundengine::setSwitch,
            Self::ak_soundengine_stopAll => ak::soundengine::stopAll,
            Self::ak_soundengine_stopPlayingID => ak::soundengine::stopPlayingID,
            Self::ak_soundengine_unloadBank => ak::soundengine::unloadBank,
            Self::ak_soundengine_unregisterGameObj => ak::soundengine::unregisterGameObj,
            Self::ak_wwise_core_audio_convert => ak::wwise::core::audio::convert,
            Self::ak_wwise_core_audio_import => ak::wwise::core::audio::import,
            Self::ak_wwise_core_audio_importTabDelimited => {
                ak::wwise::core::audio::importTabDelimited
            }
            Self::ak_wwise_core_audio_mute => ak::wwise::core::audio::mute,
            Self::ak_wwise_core_audio_resetMute => ak::wwise::core::audio::resetMute,
            Self::ak_wwise_core_audio_resetSolo => ak::wwise::core::audio::resetSolo,
            Self::ak_wwise_core_audio_setConversionPlugin => {
                ak::wwise::core::audio::setConversionPlugin
            }
            Self::ak_wwise_core_audio_solo => ak::wwise::core::audio::solo,
            Self::ak_wwise_core_audioSourcePeaks_getMinMaxPeaksInRegion => {
                ak::wwise::core::audioSourcePeaks::getMinMaxPeaksInRegion
            }
            Self::ak_wwise_core_audioSourcePeaks_getMinMaxPeaksInTrimmedRegion => {
                ak::wwise::core::audioSourcePeaks::getMinMaxPeaksInTrimmedRegion
            }
            Self::ak_wwise_core_blendContainer_addAssignment => {
                ak::wwise::core::blendContainer::addAssignment
            }
            Self::ak_wwise_core_blendContainer_addTrack => {
                ak::wwise::core::blendContainer::addTrack
            }
            Self::ak_wwise_core_blendContainer_getAssignments => {
                ak::wwise::core::blendContainer::getAssignments
            }
            Self::ak_wwise_core_blendContainer_removeAssignment => {
                ak::wwise::core::blendContainer::removeAssignment
            }
            Self::ak_wwise_core_executeLuaScript => ak::wwise::core::executeLuaScript,
            Self::ak_wwise_core_gameParameter_setRange => ak::wwise::core::gameParameter::setRange,
            Self::ak_wwise_core_getInfo => ak::wwise::core::getInfo,
            Self::ak_wwise_core_getProjectInfo => ak::wwise::core::getProjectInfo,
            Self::ak_wwise_core_log_addItem => ak::wwise::core::log::addItem,
            Self::ak_wwise_core_log_clear => ak::wwise::core::log::clear,
            Self::ak_wwise_core_log_get => ak::wwise::core::log::get,
            Self::ak_wwise_core_object_copy => ak::wwise::core::object::copy,
            Self::ak_wwise_core_object_create => ak::wwise::core::object::create,
            Self::ak_wwise_core_object_delete => ak::wwise::core::object::delete,
            Self::ak_wwise_core_object_diff => ak::wwise::core::object::diff,
            Self::ak_wwise_core_object_get => ak::wwise::core::object::get,
            Self::ak_wwise_core_object_getAttenuationCurve => {
                ak::wwise::core::object::getAttenuationCurve
            }
            Self::ak_wwise_core_object_getPropertyAndReferenceNames => {
                ak::wwise::core::object::getPropertyAndReferenceNames
            }
            Self::ak_wwise_core_object_getPropertyInfo => ak::wwise::core::object::getPropertyInfo,
            Self::ak_wwise_core_object_getTypes => ak::wwise::core::object::getTypes,
            Self::ak_wwise_core_object_isLinked => ak::wwise::core::object::isLinked,
            Self::ak_wwise_core_object_isPropertyEnabled => {
                ak::wwise::core::object::isPropertyEnabled
            }
            Self::ak_wwise_core_object_move => ak::wwise::core::object::move_,
            Self::ak_wwise_core_object_pasteProperties => ak::wwise::core::object::pasteProperties,
            Self::ak_wwise_core_object_set => ak::wwise::core::object::set,
            Self::ak_wwise_core_object_setAttenuationCurve => {
                ak::wwise::core::object::setAttenuationCurve
            }
            Self::ak_wwise_core_object_setLinked => ak::wwise::core::object::setLinked,
            Self::ak_wwise_core_object_setName => ak::wwise::core::object::setName,
            Self::ak_wwise_core_object_setNotes => ak::wwise::core::object::setNotes,
            Self::ak_wwise_core_object_setProperty => ak::wwise::core::object::setProperty,
            Self::ak_wwise_core_object_setRandomizer => ak::wwise::core::object::setRandomizer,
            Self::ak_wwise_core_object_setReference => ak::wwise::core::object::setReference,
            Self::ak_wwise_core_object_setStateGroups => ak::wwise::core::object::setStateGroups,
            Self::ak_wwise_core_object_setStateProperties => {
                ak::wwise::core::object::setStateProperties
            }
            Self::ak_wwise_core_ping => ak::wwise::core::ping,
            Self::ak_wwise_core_profiler_enableProfilerData => {
                ak::wwise::core::profiler::enableProfilerData
            }
            Self::ak_wwise_core_profiler_getAudioObjects => {
                ak::wwise::core::profiler::getAudioObjects
            }
            Self::ak_wwise_core_profiler_getBusses => ak::wwise::core::profiler::getBusses,
            Self::ak_wwise_core_profiler_getCpuUsage => ak::wwise::core::profiler::getCpuUsage,
            Self::ak_wwise_core_profiler_getCursorTime => ak::wwise::core::profiler::getCursorTime,
            Self::ak_wwise_core_profiler_getGameObjects => {
                ak::wwise::core::profiler::getGameObjects
            }
            Self::ak_wwise_core_profiler_getLoadedMedia => {
                ak::wwise::core::profiler::getLoadedMedia
            }
            Self::ak_wwise_core_profiler_getMeters => ak::wwise::core::profiler::getMeters,
            Self::ak_wwise_core_profiler_getPerformanceMonitor => {
                ak::wwise::core::profiler::getPerformanceMonitor
            }
            Self::ak_wwise_core_profiler_getRTPCs => ak::wwise::core::profiler::getRTPCs,
            Self::ak_wwise_core_profiler_getStreamedMedia => {
                ak::wwise::core::profiler::getStreamedMedia
            }
            Self::ak_wwise_core_profiler_getVoiceContributions => {
                ak::wwise::core::profiler::getVoiceContributions
            }
            Self::ak_wwise_core_profiler_getVoices => ak::wwise::core::profiler::getVoices,
            Self::ak_wwise_core_profiler_registerMeter => ak::wwise::core::profiler::registerMeter,
            Self::ak_wwise_core_profiler_saveCapture => ak::wwise::core::profiler::saveCapture,
            Self::ak_wwise_core_profiler_startCapture => ak::wwise::core::profiler::startCapture,
            Self::ak_wwise_core_profiler_stopCapture => ak::wwise::core::profiler::stopCapture,
            Self::ak_wwise_core_profiler_unregisterMeter => {
                ak::wwise::core::profiler::unregisterMeter
            }
            Self::ak_wwise_core_project_save => ak::wwise::core::project::save,
            Self::ak_wwise_core_remote_connect => ak::wwise::core::remote::connect,
            Self::ak_wwise_core_remote_disconnect => ak::wwise::core::remote::disconnect,
            Self::ak_wwise_core_remote_getAvailableConsoles => {
                ak::wwise::core::remote::getAvailableConsoles
            }
            Self::ak_wwise_core_remote_getConnectionStatus => {
                ak::wwise::core::remote::getConnectionStatus
            }
            Self::ak_wwise_core_sound_setActiveSource => ak::wwise::core::sound::setActiveSource,
            Self::ak_wwise_core_soundbank_convertExternalSources => {
                ak::wwise::core::soundbank::convertExternalSources
            }
            Self::ak_wwise_core_soundbank_generate => ak::wwise::core::soundbank::generate,
            Self::ak_wwise_core_soundbank_getInclusions => {
                ak::wwise::core::soundbank::getInclusions
            }
            Self::ak_wwise_core_soundbank_processDefinitionFiles => {
                ak::wwise::core::soundbank::processDefinitionFiles
            }
            Self::ak_wwise_core_soundbank_setInclusions => {
                ak::wwise::core::soundbank::setInclusions
            }
            Self::ak_wwise_core_sourceControl_add => ak::wwise::core::sourceControl::add,
            Self::ak_wwise_core_sourceControl_checkOut => ak::wwise::core::sourceControl::checkOut,
            Self::ak_wwise_core_sourceControl_commit => ak::wwise::core::sourceControl::commit,
            Self::ak_wwise_core_sourceControl_delete => ak::wwise::core::sourceControl::delete,
            Self::ak_wwise_core_sourceControl_getSourceFiles => {
                ak::wwise::core::sourceControl::getSourceFiles
            }
            Self::ak_wwise_core_sourceControl_getStatus => {
                ak::wwise::core::sourceControl::getStatus
            }
            Self::ak_wwise_core_sourceControl_move => ak::wwise::core::sourceControl::move_,
            Self::ak_wwise_core_sourceControl_revert => ak::wwise::core::sourceControl::revert,
            Self::ak_wwise_core_sourceControl_setProvider => {
                ak::wwise::core::sourceControl::setProvider
            }
            Self::ak_wwise_core_switchContainer_addAssignment => {
                ak::wwise::core::switchContainer::addAssignment
            }
            Self::ak_wwise_core_switchContainer_getAssignments => {
                ak::wwise::core::switchContainer::getAssignments
            }
            Self::ak_wwise_core_switchContainer_removeAssignment => {
                ak::wwise::core::switchContainer::removeAssignment
            }
            Self::ak_wwise_core_transport_create => ak::wwise::core::transport::create,
            Self::ak_wwise_core_transport_destroy => ak::wwise::core::transport::destroy,
            Self::ak_wwise_core_transport_executeAction => {
                ak::wwise::core::transport::executeAction
            }
            Self::ak_wwise_core_transport_getList => ak::wwise::core::transport::getList,
            Self::ak_wwise_core_transport_getState => ak::wwise::core::transport::getState,
            Self::ak_wwise_core_transport_prepare => ak::wwise::core::transport::prepare,
            Self::ak_wwise_core_transport_useOriginals => ak::wwise::core::transport::useOriginals,
            Self::ak_wwise_core_undo_beginGroup => ak::wwise::core::undo::beginGroup,
            Self::ak_wwise_core_undo_cancelGroup => ak::wwise::core::undo::cancelGroup,
            Self::ak_wwise_core_undo_endGroup => ak::wwise::core::undo::endGroup,
            Self::ak_wwise_core_undo_redo => ak::wwise::core::undo::redo,
            Self::ak_wwise_core_undo_undo => ak::wwise::core::undo::undo,
            Self::ak_wwise_debug_enableAsserts => ak::wwise::debug::enableAsserts,
            Self::ak_wwise_debug_enableAutomationMode => ak::wwise::debug::enableAutomationMode,
            Self::ak_wwise_debug_generateToneWAV => ak::wwise::debug::generateToneWAV,
            Self::ak_wwise_debug_getWalTree => ak::wwise::debug::getWalTree,
            Self::ak_wwise_debug_restartWaapiServers => ak::wwise::debug::restartWaapiServers,
            Self::ak_wwise_debug_testAssert => ak::wwise::debug::testAssert,
            Self::ak_wwise_debug_testCrash => ak::wwise::debug::testCrash,
            Self::ak_wwise_debug_validateCall => ak::wwise::debug::validateCall,
            Self::ak_wwise_ui_bringToForeground => ak::wwise::ui::bringToForeground,
            Self::ak_wwise_ui_captureScreen => ak::wwise::ui::captureScreen,
            Self::ak_wwise_ui_cli_executeLuaScript => ak::wwise::ui::cli::executeLuaScript,
            Self::ak_wwise_ui_cli_launch => ak::wwise::ui::cli::launch,
            Self::ak_wwise_ui_commands_execute => ak::wwise::ui::commands::execute,
            Self::ak_wwise_ui_commands_getCommands => ak::wwise::ui::commands::getCommands,
            Self::ak_wwise_ui_commands_register => ak::wwise::ui::commands::register,
            Self::ak_wwise_ui_commands_unregister => ak::wwise::ui::commands::unregister,
            Self::ak_wwise_ui_getSelectedObjects => ak::wwise::ui::getSelectedObjects,
            Self::ak_wwise_ui_layout_dockView => ak::wwise::ui::layout::dockView,
            Self::ak_wwise_ui_layout_getCurrentLayoutName => {
                ak::wwise::ui::layout::getCurrentLayoutName
            }
            Self::ak_wwise_ui_layout_getElementRectangle => {
                ak::wwise::ui::layout::getElementRectangle
            }
            Self::ak_wwise_ui_layout_getLayout => ak::wwise::ui::layout::getLayout,
            Self::ak_wwise_ui_layout_getLayoutNames => ak::wwise::ui::layout::getLayoutNames,
            Self::ak_wwise_ui_layout_getOrCreateView => ak::wwise::ui::layout::getOrCreateView,
            Self::ak_wwise_ui_layout_getViewHandle => ak::wwise::ui::layout::getViewHandle,
            Self::ak_wwise_ui_layout_getViewInstances => ak::wwise::ui::layout::getViewInstances,
            Self::ak_wwise_ui_layout_getViewTypes => ak::wwise::ui::layout::getViewTypes,
            Self::ak_wwise_ui_layout_moveSplitter => ak::wwise::ui::layout::moveSplitter,
            Self::ak_wwise_ui_layout_removeLayout => ak::wwise::ui::layout::removeLayout,
            Self::ak_wwise_ui_layout_setLayout => ak::wwise::ui::layout::setLayout,
            Self::ak_wwise_ui_layout_switchLayout => ak::wwise::ui::layout::switchLayout,
            Self::ak_wwise_ui_layout_undockView => ak::wwise::ui::layout::undockView,
            Self::ak_wwise_ui_model_createHandle => ak::wwise::ui::model::createHandle,
            Self::ak_wwise_ui_model_destroyHandle => ak::wwise::ui::model::destroyHandle,
            Self::ak_wwise_ui_model_getState => ak::wwise::ui::model::getState,
            Self::ak_wwise_ui_model_registerWafm => ak::wwise::ui::model::registerWafm,
            Self::ak_wwise_ui_model_setState => ak::wwise::ui::model::setState,
            Self::ak_wwise_ui_project_close => ak::wwise::ui::project::close,
            Self::ak_wwise_ui_project_create => ak::wwise::ui::project::create,
            Self::ak_wwise_ui_project_open => ak::wwise::ui::project::open,
            Self::ak_wwise_ui_signal_emit => ak::wwise::ui::signal::emit,
            Self::ak_wwise_ui_window_close => ak::wwise::ui::window::close,
            Self::ak_wwise_ui_window_create => ak::wwise::ui::window::create,
            Self::ak_wwise_ui_window_present => ak::wwise::ui::window::present,
            Self::ak_wwise_waapi_getFunctions => ak::wwise::waapi::getFunctions,
            Self::ak_wwise_waapi_getSchema => ak::wwise::waapi::getSchema,
            Self::ak_wwise_waapi_getTopics => ak::wwise::waapi::getTopics,
        }
    }
}

impl std::fmt::Display for WaapiFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl AsRef<str> for WaapiFunction {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
