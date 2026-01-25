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
                pub const r#move: &str = "ak.wwise.core.object.move";
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
                pub const r#move: &str = "ak.wwise.core.sourceControl.move";
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
