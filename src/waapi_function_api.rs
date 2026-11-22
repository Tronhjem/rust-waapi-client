/// Wwise Authoring API URI constants organized by namespace.
#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
pub mod ak {
    /// The `ak.soundengine` WAAPI namespace.
    pub mod soundengine {
        /// Executes an action on all nodes that are referenced in the specified event in a Play action. See `AK::SoundEngine::ExecuteActionOnEvent`.
        ///
        /// WAAPI URI: `ak.soundengine.executeActionOnEvent`.
        pub const executeActionOnEvent: &str = "ak.soundengine.executeActionOnEvent";
        /// Gets the current state of a State Group. When using setState just prior to getState, allow a brief delay (no more than 10ms) for the information to update in the sound engine.
        ///
        /// WAAPI URI: `ak.soundengine.getState`.
        pub const getState: &str = "ak.soundengine.getState";
        /// Gets the current state of a Switch Group for a given Game Object.
        ///
        /// WAAPI URI: `ak.soundengine.getSwitch`.
        pub const getSwitch: &str = "ak.soundengine.getSwitch";
        /// Load a SoundBank. See `AK::SoundEngine::LoadBank`.
        ///
        /// WAAPI URI: `ak.soundengine.loadBank`.
        pub const loadBank: &str = "ak.soundengine.loadBank";
        /// Asynchronously post an Event to the sound engine (by event ID). See `AK::SoundEngine::PostEvent`.
        ///
        /// WAAPI URI: `ak.soundengine.postEvent`.
        pub const postEvent: &str = "ak.soundengine.postEvent";
        /// Display a message in the Profiler's Capture Log view.
        ///
        /// WAAPI URI: `ak.soundengine.postMsgMonitor`.
        pub const postMsgMonitor: &str = "ak.soundengine.postMsgMonitor";
        /// Posts the specified Trigger. See `AK::SoundEngine::PostTrigger`.
        ///
        /// WAAPI URI: `ak.soundengine.postTrigger`.
        pub const postTrigger: &str = "ak.soundengine.postTrigger";
        /// Register a game object. Registering a game object twice does nothing. Unregistering it once unregisters it no matter how many times it has been registered. See `AK::SoundEngine::RegisterGameObj`.
        ///
        /// WAAPI URI: `ak.soundengine.registerGameObj`.
        pub const registerGameObj: &str = "ak.soundengine.registerGameObj";
        /// Resets the value of a real-time parameter control to its default value, as specified in the Wwise project. See `AK::SoundEngine::ResetRTPCValue`.
        ///
        /// WAAPI URI: `ak.soundengine.resetRTPCValue`.
        pub const resetRTPCValue: &str = "ak.soundengine.resetRTPCValue";
        /// Seeks inside all playing objects that are referenced in Play Actions of the specified Event. See `AK::SoundEngine::SeekOnEvent`.
        ///
        /// WAAPI URI: `ak.soundengine.seekOnEvent`.
        pub const seekOnEvent: &str = "ak.soundengine.seekOnEvent";
        /// Sets the default active listeners for all subsequent game objects that are registered. See `AK::SoundEngine::SetDefaultListeners`.
        ///
        /// WAAPI URI: `ak.soundengine.setDefaultListeners`.
        pub const setDefaultListeners: &str = "ak.soundengine.setDefaultListeners";
        /// Sets the Auxiliary Busses to route the specified game object. See `AK::SoundEngine::SetGameObjectAuxSendValues`.
        ///
        /// WAAPI URI: `ak.soundengine.setGameObjectAuxSendValues`.
        pub const setGameObjectAuxSendValues: &str = "ak.soundengine.setGameObjectAuxSendValues";
        /// Set the output bus volume (direct) to be used for the specified game object. See `AK::SoundEngine::SetGameObjectOutputBusVolume`.
        ///
        /// WAAPI URI: `ak.soundengine.setGameObjectOutputBusVolume`.
        pub const setGameObjectOutputBusVolume: &str =
            "ak.soundengine.setGameObjectOutputBusVolume";
        /// Sets a listener's spatialization parameters. This lets you define listener-specific volume offsets for each audio channel. See `AK::SoundEngine::SetListenerSpatialization`.
        ///
        /// WAAPI URI: `ak.soundengine.setListenerSpatialization`.
        pub const setListenerSpatialization: &str = "ak.soundengine.setListenerSpatialization";
        /// Sets a single game object's active listeners. By default, all new game objects have no listeners active, but this behavior can be overridden with `SetDefaultListeners()`. Inactive listeners are not computed. See `AK::SoundEngine::SetListeners`.
        ///
        /// WAAPI URI: `ak.soundengine.setListeners`.
        pub const setListeners: &str = "ak.soundengine.setListeners";
        /// Sets multiple positions for a single game object. Setting multiple positions for a single game object is a way to simulate multiple emission sources while using the resources of only one voice. This can be used to simulate wall openings, area sounds, or multiple objects emitting the same sound in the same area. See `AK::SoundEngine::SetMultiplePositions`.
        ///
        /// WAAPI URI: `ak.soundengine.setMultiplePositions`.
        pub const setMultiplePositions: &str = "ak.soundengine.setMultiplePositions";
        /// Set a game object's obstruction and occlusion levels. This function is used to affect how an object should be heard by a specific listener. See `AK::SoundEngine::SetObjectObstructionAndOcclusion`.
        ///
        /// WAAPI URI: `ak.soundengine.setObjectObstructionAndOcclusion`.
        pub const setObjectObstructionAndOcclusion: &str =
            "ak.soundengine.setObjectObstructionAndOcclusion";
        /// Sets the position of a game object. See `AK::SoundEngine::SetPosition`.
        ///
        /// WAAPI URI: `ak.soundengine.setPosition`.
        pub const setPosition: &str = "ak.soundengine.setPosition";
        /// Sets the value of a Game Parameter for real-time parameter control. See `AK::SoundEngine::SetRTPCValue`.
        ///
        /// WAAPI URI: `ak.soundengine.setRTPCValue`.
        pub const setRTPCValue: &str = "ak.soundengine.setRTPCValue";
        /// Sets the scaling factor of a game object. You can modify the attenuation computations on this game object to simulate sounds with a larger or smaller affected areas. See `AK::SoundEngine::SetScalingFactor`.
        ///
        /// WAAPI URI: `ak.soundengine.setScalingFactor`.
        pub const setScalingFactor: &str = "ak.soundengine.setScalingFactor";
        /// Sets the State of a State Group. See `AK::SoundEngine::SetState`.
        ///
        /// WAAPI URI: `ak.soundengine.setState`.
        pub const setState: &str = "ak.soundengine.setState";
        /// Sets the State of a Switch Group. See `AK::SoundEngine::SetSwitch`.
        ///
        /// WAAPI URI: `ak.soundengine.setSwitch`.
        pub const setSwitch: &str = "ak.soundengine.setSwitch";
        /// Stop playing the current content associated to the specified game object ID. If no game object is specified, all sounds are stopped. See `AK::SoundEngine::StopAll`.
        ///
        /// WAAPI URI: `ak.soundengine.stopAll`.
        pub const stopAll: &str = "ak.soundengine.stopAll";
        /// Stops the current content, associated to the specified playing ID, from playing. See `AK::SoundEngine::StopPlayingID`.
        ///
        /// WAAPI URI: `ak.soundengine.stopPlayingID`.
        pub const stopPlayingID: &str = "ak.soundengine.stopPlayingID";
        /// Unload a SoundBank. See `AK::SoundEngine::UnloadBank`.
        ///
        /// WAAPI URI: `ak.soundengine.unloadBank`.
        pub const unloadBank: &str = "ak.soundengine.unloadBank";
        /// Unregisters a game object. Registering a game object twice does nothing. Unregistering it once unregisters it no matter how many times it has been registered. Unregistering a game object while it is in use is allowed, but the control over the parameters of this game object is lost. For example, say a sound associated with this game object is a 3D moving sound. It stops moving when the game object is unregistered, and there is no way to regain control over the game object. See `AK::SoundEngine::UnregisterGameObj`.
        ///
        /// WAAPI URI: `ak.soundengine.unregisterGameObj`.
        pub const unregisterGameObj: &str = "ak.soundengine.unregisterGameObj";
    }
    /// The `ak.wwise` WAAPI namespace.
    pub mod wwise {
        /// The `ak.wwise.core` WAAPI namespace.
        pub mod core {
            /// Execute Lua code from a file path (using `luaScript`) or directly as a string (using `luaString`). Optionally, specify additional Lua search paths, additional modules, and additional Lua scripts to load prior to the main script. The script can return a value, which can be any of the Lua types, including tables. All WAAPI arguments will be passed to the Lua script in the `wa_args` global variable, as a Lua table. Define your own key-values in the WAAPI arguments and access them from the Lua code. Use `wa_call` to call a WAAPI function from the Lua script. See `ak.wwise.waapi.getFunctions` and `ak.wwise.waapi.getSchema` to learn more about the functions available and their schema before using `wa_call`. If both `luaScript` and `luaString` are provided, the script file will be executed first, followed by the string code. Note that the Lua environment is reset at each call to this function.
            ///
            /// WAAPI URI: `ak.wwise.core.executeLuaScript`.
            pub const executeLuaScript: &str = "ak.wwise.core.executeLuaScript";
            /// Retrieve global Wwise information.
            ///
            /// WAAPI URI: `ak.wwise.core.getInfo`.
            pub const getInfo: &str = "ak.wwise.core.getInfo";
            /// Retrieve information about the current project opened, including platforms, languages and project directories.
            ///
            /// WAAPI URI: `ak.wwise.core.getProjectInfo`.
            pub const getProjectInfo: &str = "ak.wwise.core.getProjectInfo";
            /// Verify if WAAPI is currently available.
            ///
            /// WAAPI URI: `ak.wwise.core.ping`.
            pub const ping: &str = "ak.wwise.core.ping";

            /// The `ak.wwise.core.audio` WAAPI namespace.
            pub mod audio {
                /// Creates a converted audio file. When errors occur, this function returns a list of messages with corresponding levels of severity.
                ///
                /// WAAPI URI: `ak.wwise.core.audio.convert`.
                pub const convert: &str = "ak.wwise.core.audio.convert";
                /// Creates Wwise objects and imports audio files. This function does not return an error when something fails during the import process, please see the log for the result of each import command. This function uses the same importation processor available through the Tab Delimited import in the Audio File Importer. The function returns an array of all objects created, replaced or re-used. Use the options to specify how the objects are returned. For more information, see the WAAPI Import documentation.
                ///
                /// WAAPI URI: `ak.wwise.core.audio.import`.
                pub const import: &str = "ak.wwise.core.audio.import";
                /// Scripted object creation and audio file import from a tab-delimited file.
                ///
                /// WAAPI URI: `ak.wwise.core.audio.importTabDelimited`.
                pub const importTabDelimited: &str = "ak.wwise.core.audio.importTabDelimited";
                /// Mutes an object.
                ///
                /// WAAPI URI: `ak.wwise.core.audio.mute`.
                pub const mute: &str = "ak.wwise.core.audio.mute";
                /// Unmute all muted objects.
                ///
                /// WAAPI URI: `ak.wwise.core.audio.resetMute`.
                pub const resetMute: &str = "ak.wwise.core.audio.resetMute";
                /// Unsolo all soloed objects.
                ///
                /// WAAPI URI: `ak.wwise.core.audio.resetSolo`.
                pub const resetSolo: &str = "ak.wwise.core.audio.resetSolo";
                /// Changes the plug-in to use for audio file conversion.
                ///
                /// WAAPI URI: `ak.wwise.core.audio.setConversionPlugin`.
                pub const setConversionPlugin: &str = "ak.wwise.core.audio.setConversionPlugin";
                /// Solos an object.
                ///
                /// WAAPI URI: `ak.wwise.core.audio.solo`.
                pub const solo: &str = "ak.wwise.core.audio.solo";
            }

            /// The `ak.wwise.core.audioSourcePeaks` WAAPI namespace.
            pub mod audioSourcePeaks {
                /// Gets the min/max peak pairs, in the given region of an audio source, as a collection of binary strings (one per channel). The strings are base-64 encoded, 16-bit signed int arrays, with min and max values being interleaved. If getCrossChannelPeaks is true, only one binary string represents the peaks across all channels globally.
                ///
                /// WAAPI URI: `ak.wwise.core.audioSourcePeaks.getMinMaxPeaksInRegion`.
                pub const getMinMaxPeaksInRegion: &str =
                    "ak.wwise.core.audioSourcePeaks.getMinMaxPeaksInRegion";
                /// Gets the min/max peak pairs in the entire trimmed region of an audio source, for each channel, as an array of binary strings (one per channel). The strings are base-64 encoded, 16-bit signed int arrays, with min and max values being interleaved. If getCrossChannelPeaks is true, there is only one binary string representing peaks across all channels globally.
                ///
                /// WAAPI URI: `ak.wwise.core.audioSourcePeaks.getMinMaxPeaksInTrimmedRegion`.
                pub const getMinMaxPeaksInTrimmedRegion: &str =
                    "ak.wwise.core.audioSourcePeaks.getMinMaxPeaksInTrimmedRegion";
            }

            /// The `ak.wwise.core.blendContainer` WAAPI namespace.
            pub mod blendContainer {
                /// Adds a new assignment to a Blend Track. Equivalent to performing a drag-and-drop operation in the Blend Tracks Editor.
                ///
                /// WAAPI URI: `ak.wwise.core.blendContainer.addAssignment`.
                pub const addAssignment: &str = "ak.wwise.core.blendContainer.addAssignment";
                /// Adds a new Blend Track to a Blend Container. Equivalent to clicking New Blend Track in the Blend Track Editor. To get the list of Blend Tracks, use the function ak.wwise.core.object.get with {return = "blendTracks"}.
                ///
                /// WAAPI URI: `ak.wwise.core.blendContainer.addTrack`.
                pub const addTrack: &str = "ak.wwise.core.blendContainer.addTrack";
                /// Returns a list of a Blend Track assignments.
                ///
                /// WAAPI URI: `ak.wwise.core.blendContainer.getAssignments`.
                pub const getAssignments: &str = "ak.wwise.core.blendContainer.getAssignments";
                /// Removes an assignment from a Blend Track. Equivalent to deleting an entry in the Blend Tracks Editor.
                ///
                /// WAAPI URI: `ak.wwise.core.blendContainer.removeAssignment`.
                pub const removeAssignment: &str = "ak.wwise.core.blendContainer.removeAssignment";
            }

            /// The `ak.wwise.core.gameParameter` WAAPI namespace.
            pub mod gameParameter {
                /// Sets the Min and Max properties on a Game Parameter. Modifies the RTPC curves and blend tracks that use this Game Parameter for their X axis.
                ///
                /// WAAPI URI: `ak.wwise.core.gameParameter.setRange`.
                pub const setRange: &str = "ak.wwise.core.gameParameter.setRange";
            }

            /// The `ak.wwise.core.log` WAAPI namespace.
            pub mod log {
                /// Adds a new item to the logs on the specified channel.
                ///
                /// WAAPI URI: `ak.wwise.core.log.addItem`.
                pub const addItem: &str = "ak.wwise.core.log.addItem";
                /// Clears the logs on the specified channel.
                ///
                /// WAAPI URI: `ak.wwise.core.log.clear`.
                pub const clear: &str = "ak.wwise.core.log.clear";
                /// Retrieves the latest log for a specific channel. See `ak.wwise.core.log.itemAdded` to be notified when an item is added to the log. The log is empty when used in WwiseConsole.
                ///
                /// WAAPI URI: `ak.wwise.core.log.get`.
                pub const get: &str = "ak.wwise.core.log.get";
            }

            /// The `ak.wwise.core.mediaPool` WAAPI namespace.
            pub mod mediaPool {
                /// Retrieve files from Media Pool. Use the return options to specify which properties of the files to return.
                ///
                /// WAAPI URI: `ak.wwise.core.mediaPool.get`.
                pub const get: &str = "ak.wwise.core.mediaPool.get";
                /// Retrieve all fields present in the Media Pool. You can then use the fields to query the Media Pool. The Media Pool discovers some fields when it scans audio files. Others, such as WAV fields, are always available.
                ///
                /// WAAPI URI: `ak.wwise.core.mediaPool.getFields`.
                pub const getFields: &str = "ak.wwise.core.mediaPool.getFields";
            }

            /// The `ak.wwise.core.object` WAAPI namespace.
            pub mod object {
                /// Copies an object to the given parent. Note that if a Work Unit is copied, the operation cannot be undone and the project will be saved.
                ///
                /// WAAPI URI: `ak.wwise.core.object.copy`.
                pub const copy: &str = "ak.wwise.core.object.copy";
                /// Creates an object of type 'type', as a child of 'parent'. See the WAAPI Import documentation for more information about creating objects. Also see `ak.wwise.core.audio.import` to import audio files to Wwise. To create Effect or Source plug-ins, use `ak.wwise.core.object.set`, and see the Wwise Objects Reference for the classId.
                ///
                /// WAAPI URI: `ak.wwise.core.object.create`.
                pub const create: &str = "ak.wwise.core.object.create";
                /// Deletes the specified object. Note that if a Work Unit is deleted, the operation cannot be undone and the project will be saved.
                ///
                /// WAAPI URI: `ak.wwise.core.object.delete`.
                pub const delete: &str = "ak.wwise.core.object.delete";
                /// Compares properties and lists of the source object with those in the target object.
                ///
                /// WAAPI URI: `ak.wwise.core.object.diff`.
                pub const diff: &str = "ak.wwise.core.object.diff";
                /// Performs a query and returns the data, as specified in the options, for each object in the query result. The query can specify either a 'waql' argument or a 'from' argument with an optional 'transform' argument. See the WAQL introduction documentation or the WAAPI query documentation for more information. See the WAAPI query return options documentation to learn about options.
                ///
                /// WAAPI URI: `ak.wwise.core.object.get`.
                pub const get: &str = "ak.wwise.core.object.get";
                /// Gets the specified attenuation curve for a given attenuation object.
                ///
                /// WAAPI URI: `ak.wwise.core.object.getAttenuationCurve`.
                pub const getAttenuationCurve: &str = "ak.wwise.core.object.getAttenuationCurve";
                /// Retrieves the list of property and reference names for an object.
                ///
                /// WAAPI URI: `ak.wwise.core.object.getPropertyAndReferenceNames`.
                pub const getPropertyAndReferenceNames: &str =
                    "ak.wwise.core.object.getPropertyAndReferenceNames";
                /// Retrieves information about an object property. Note that this function does not return the value of a property. To retrieve the value of a property, see `ak.wwise.core.object.get` and the WAAPI query return options documentation.
                ///
                /// WAAPI URI: `ak.wwise.core.object.getPropertyInfo`.
                pub const getPropertyInfo: &str = "ak.wwise.core.object.getPropertyInfo";
                /// Retrieves the list of all object types registered in Wwise's object model. This function returns the equivalent of the Wwise Objects Reference.
                ///
                /// WAAPI URI: `ak.wwise.core.object.getTypes`.
                pub const getTypes: &str = "ak.wwise.core.object.getTypes";
                /// Indicates whether a property, reference, or object list is bound to a particular platform or to all platforms.
                ///
                /// WAAPI URI: `ak.wwise.core.object.isLinked`.
                pub const isLinked: &str = "ak.wwise.core.object.isLinked";
                /// Returns true if a property is enabled based on the values of the properties it depends on.
                ///
                /// WAAPI URI: `ak.wwise.core.object.isPropertyEnabled`.
                pub const isPropertyEnabled: &str = "ak.wwise.core.object.isPropertyEnabled";
                /// Moves an object to the given parent. Returns the moved object.
                ///
                /// WAAPI URI: `ak.wwise.core.object.move`.
                pub const move_: &str = "ak.wwise.core.object.move";
                /// Pastes properties, references and lists from one object to any number of target objects. Only those properties, references and lists which differ between source and target are pasted. See the Wwise Objects Reference for more information on the properties, references and lists available on each object type.
                ///
                /// WAAPI URI: `ak.wwise.core.object.pasteProperties`.
                pub const pasteProperties: &str = "ak.wwise.core.object.pasteProperties";
                /// Allows for batch processing of the following operations: Object creation in a child hierarchy, Object creation in a list, Setting name, notes, properties and references. See the WAAPI Import documentation for more information about creating objects. Also see `ak.wwise.core.audio.import` to import audio files to Wwise.
                ///
                /// WAAPI URI: `ak.wwise.core.object.set`.
                pub const set: &str = "ak.wwise.core.object.set";
                /// Sets the specified attenuation curve for a given attenuation object.
                ///
                /// WAAPI URI: `ak.wwise.core.object.setAttenuationCurve`.
                pub const setAttenuationCurve: &str = "ak.wwise.core.object.setAttenuationCurve";
                /// Link or unlink a property/reference or object list to a particular platform.
                ///
                /// WAAPI URI: `ak.wwise.core.object.setLinked`.
                pub const setLinked: &str = "ak.wwise.core.object.setLinked";
                /// Renames an object.
                ///
                /// WAAPI URI: `ak.wwise.core.object.setName`.
                pub const setName: &str = "ak.wwise.core.object.setName";
                /// Sets the object's notes.
                ///
                /// WAAPI URI: `ak.wwise.core.object.setNotes`.
                pub const setNotes: &str = "ak.wwise.core.object.setNotes";
                /// Sets a property value of an object for a specific platform. See the Wwise Objects Reference for more information on the properties available on each object type. See `ak.wwise.core.object.setReference` to set a reference to an object. See `ak.wwise.core.object.get` to obtain the value of a property for an object.
                ///
                /// WAAPI URI: `ak.wwise.core.object.setProperty`.
                pub const setProperty: &str = "ak.wwise.core.object.setProperty";
                /// Sets the randomizer values of a property of an object for a specific platform. See the Wwise Objects Reference for more information on the properties available on each object type.
                ///
                /// WAAPI URI: `ak.wwise.core.object.setRandomizer`.
                pub const setRandomizer: &str = "ak.wwise.core.object.setRandomizer";
                /// Sets an object's reference value. See the Wwise Objects Reference for more information on the references available on each object type.
                ///
                /// WAAPI URI: `ak.wwise.core.object.setReference`.
                pub const setReference: &str = "ak.wwise.core.object.setReference";
                /// Sets the State Group objects associated with an object. Note, this will remove any previously associated State Group.
                ///
                /// WAAPI URI: `ak.wwise.core.object.setStateGroups`.
                pub const setStateGroups: &str = "ak.wwise.core.object.setStateGroups";
                /// Set the state properties of an object. Note, this will remove any previous state property, including the default ones.
                ///
                /// WAAPI URI: `ak.wwise.core.object.setStateProperties`.
                pub const setStateProperties: &str = "ak.wwise.core.object.setStateProperties";
            }

            /// The `ak.wwise.core.profiler` WAAPI namespace.
            pub mod profiler {
                /// Specifies the type of data you want to capture. Overrides the user's profiler settings.
                ///
                /// WAAPI URI: `ak.wwise.core.profiler.enableProfilerData`.
                pub const enableProfilerData: &str = "ak.wwise.core.profiler.enableProfilerData";
                /// Retrieves the Audio Objects at a specific profiler capture time.
                ///
                /// WAAPI URI: `ak.wwise.core.profiler.getAudioObjects`.
                pub const getAudioObjects: &str = "ak.wwise.core.profiler.getAudioObjects";
                /// Retrieves the busses at a specific profiler capture time.
                ///
                /// WAAPI URI: `ak.wwise.core.profiler.getBusses`.
                pub const getBusses: &str = "ak.wwise.core.profiler.getBusses";
                /// Retrieves CPU usage statistics at a specific profiler capture time. This data can also be found in the Advanced Profiler, under the CPU tab. To ensure the CPU data is received, see `ak.wwise.core.profiler.enableProfilerData`. The returned data includes "Inclusive" and "Exclusive" values, where "Inclusive" refers to the time spent in the element plus the time spent in any called elements, and "Exclusive" values pertain to execution only within the element itself.
                ///
                /// WAAPI URI: `ak.wwise.core.profiler.getCpuUsage`.
                pub const getCpuUsage: &str = "ak.wwise.core.profiler.getCpuUsage";
                /// Returns the current time of the specified profiler cursor, in milliseconds.
                ///
                /// WAAPI URI: `ak.wwise.core.profiler.getCursorTime`.
                pub const getCursorTime: &str = "ak.wwise.core.profiler.getCursorTime";
                /// Retrieves the game objects at a specific profiler capture time.
                ///
                /// WAAPI URI: `ak.wwise.core.profiler.getGameObjects`.
                pub const getGameObjects: &str = "ak.wwise.core.profiler.getGameObjects";
                /// Retrieves the loaded media at a specific profiler capture time. This data can also be found in the Advanced Profiler, under the Loaded Media tab. To ensure the Loaded Media data is received, see `ak.wwise.core.profiler.enableProfilerData`.
                ///
                /// WAAPI URI: `ak.wwise.core.profiler.getLoadedMedia`.
                pub const getLoadedMedia: &str = "ak.wwise.core.profiler.getLoadedMedia";
                /// Retrieves the Meter data for all registered busses, aux busses and devices. Only the Main Audio Bus is registered by default. Use `ak.wwise.core.profiler.registerMeter` for other busses, before retrieval of the meter data.
                ///
                /// WAAPI URI: `ak.wwise.core.profiler.getMeters`.
                pub const getMeters: &str = "ak.wwise.core.profiler.getMeters";
                /// Retrieves the Performance Monitor statistics at a specific profiler capture time. See the Wwise global counter IDs reference for the available counters.
                ///
                /// WAAPI URI: `ak.wwise.core.profiler.getPerformanceMonitor`.
                pub const getPerformanceMonitor: &str =
                    "ak.wwise.core.profiler.getPerformanceMonitor";
                /// Retrieves active RTPCs at a specific profiler capture time.
                ///
                /// WAAPI URI: `ak.wwise.core.profiler.getRTPCs`.
                pub const getRTPCs: &str = "ak.wwise.core.profiler.getRTPCs";
                /// Retrieves the streaming media at a specific profiler capture time. This data can also be found in the Advanced Profiler, under the Streams tab. To ensure the Streams data is received, see `ak.wwise.core.profiler.enableProfilerData`.
                ///
                /// WAAPI URI: `ak.wwise.core.profiler.getStreamedMedia`.
                pub const getStreamedMedia: &str = "ak.wwise.core.profiler.getStreamedMedia";
                /// Retrieves all parameters affecting voice volume, highpass and lowpass for a voice path, resolved from pipeline IDs.
                ///
                /// WAAPI URI: `ak.wwise.core.profiler.getVoiceContributions`.
                pub const getVoiceContributions: &str =
                    "ak.wwise.core.profiler.getVoiceContributions";
                /// Retrieves the voices at a specific profiler capture time.
                ///
                /// WAAPI URI: `ak.wwise.core.profiler.getVoices`.
                pub const getVoices: &str = "ak.wwise.core.profiler.getVoices";
                /// Moves the user time cursor of the profiler to a specific position (first, last, next, previous), and returns the new time. This sets the cursor position in the profiler views.
                ///
                /// WAAPI URI: `ak.wwise.core.profiler.moveCursor`.
                pub const moveCursor: &str = "ak.wwise.core.profiler.moveCursor";
                /// Registers a bus, an aux bus or device to receive meter data. Only the Main Audio Bus is registered by default. Use `ak.wwise.core.profiler.getMeters` to retrieve the meter data after registering. Every call to ak.wwise.core.profiler.registerMeter must have a matching call to `ak.wwise.core.profiler.unregisterMeter`.
                ///
                /// WAAPI URI: `ak.wwise.core.profiler.registerMeter`.
                pub const registerMeter: &str = "ak.wwise.core.profiler.registerMeter";
                /// Saves profiler as a.prof file according to the given file path.
                ///
                /// WAAPI URI: `ak.wwise.core.profiler.saveCapture`.
                pub const saveCapture: &str = "ak.wwise.core.profiler.saveCapture";
                /// Sets the user time cursor of the profiler to the specified time, in milliseconds, and returns the new time. Use the time obtained from the other profiler functions. Note that the specified time will be snapped to a buffer interval (~10ms). This sets the cursor position in the profiler views. To synchronize the user time cursor to the latest capture time, use `ak.wwise.ui.commands.execute` with `ToggleFollowCapture`.
                ///
                /// WAAPI URI: `ak.wwise.core.profiler.setCursorTime`.
                pub const setCursorTime: &str = "ak.wwise.core.profiler.setCursorTime";
                /// Starts the profiler capture and returns the time at the beginning of the capture, in milliseconds.
                ///
                /// WAAPI URI: `ak.wwise.core.profiler.startCapture`.
                pub const startCapture: &str = "ak.wwise.core.profiler.startCapture";
                /// Stops the profiler capture and returns the time at the end of the capture, in milliseconds.
                ///
                /// WAAPI URI: `ak.wwise.core.profiler.stopCapture`.
                pub const stopCapture: &str = "ak.wwise.core.profiler.stopCapture";
                /// Unregisters a bus or device that was registered with `ak.wwise.core.profiler.registerMeter`.
                ///
                /// WAAPI URI: `ak.wwise.core.profiler.unregisterMeter`.
                pub const unregisterMeter: &str = "ak.wwise.core.profiler.unregisterMeter";

                /// The `ak.wwise.core.profiler.captureLog` WAAPI namespace.
                pub mod captureLog {
                    /// Saves the content of the capture log to a text file according to the given file path.
                    ///
                    /// WAAPI URI: `ak.wwise.core.profiler.captureLog.save`.
                    pub const save: &str = "ak.wwise.core.profiler.captureLog.save";
                }
            }

            /// The `ak.wwise.core.project` WAAPI namespace.
            pub mod project {
                /// Saves the current project.
                ///
                /// WAAPI URI: `ak.wwise.core.project.save`.
                pub const save: &str = "ak.wwise.core.project.save";
            }

            /// The `ak.wwise.core.remote` WAAPI namespace.
            pub mod remote {
                /// Connects the Wwise Authoring application to a Wwise Sound Engine running executable or to a saved profile file. The host must be running code with communication enabled. If only "host" is provided, Wwise connects to the first Sound Engine instance found. To distinguish between different instances, you can also provide the name of the application to connect to.
                ///
                /// WAAPI URI: `ak.wwise.core.remote.connect`.
                pub const connect: &str = "ak.wwise.core.remote.connect";
                /// Disconnects the Wwise Authoring application from a connected Wwise Sound Engine running executable.
                ///
                /// WAAPI URI: `ak.wwise.core.remote.disconnect`.
                pub const disconnect: &str = "ak.wwise.core.remote.disconnect";
                /// Retrieves all consoles available for connecting Wwise Authoring to a Sound Engine instance.
                ///
                /// WAAPI URI: `ak.wwise.core.remote.getAvailableConsoles`.
                pub const getAvailableConsoles: &str = "ak.wwise.core.remote.getAvailableConsoles";
                /// Retrieves the connection status.
                ///
                /// WAAPI URI: `ak.wwise.core.remote.getConnectionStatus`.
                pub const getConnectionStatus: &str = "ak.wwise.core.remote.getConnectionStatus";
            }

            /// The `ak.wwise.core.sound` WAAPI namespace.
            pub mod sound {
                /// Sets which version of the source is being used for the specified sound. Use `ak.wwise.core.object.get` with the 'activeSource' return option to get the active source of a sound.
                ///
                /// WAAPI URI: `ak.wwise.core.sound.setActiveSource`.
                pub const setActiveSource: &str = "ak.wwise.core.sound.setActiveSource";
            }

            /// The `ak.wwise.core.soundbank` WAAPI namespace.
            pub mod soundbank {
                /// Converts the external sources files for the project as detailed in the wsources file, and places them into either the default folder, or the folder specified by the output argument. External Sources are a special type of source that you can put in a Sound object in Wwise. It indicates that the real sound data will be provided at run time. While External Source conversion is also triggered by SoundBank generation, this operation can be used to process sources not contained in the Wwise Project. Please see Wwise SDK help page "Integrating External Sources".
                ///
                /// WAAPI URI: `ak.wwise.core.soundbank.convertExternalSources`.
                pub const convertExternalSources: &str =
                    "ak.wwise.core.soundbank.convertExternalSources";
                /// Generate a list of SoundBanks with the import definition specified in the WAAPI request. If you do not write the SoundBanks to disk, subscribe to `ak.wwise.core.soundbank.generated` to receive SoundBank structure info and the bank data as base64. Note: This is a synchronous operation.
                ///
                /// WAAPI URI: `ak.wwise.core.soundbank.generate`.
                pub const generate: &str = "ak.wwise.core.soundbank.generate";
                /// Retrieves a SoundBank's inclusion list.
                ///
                /// WAAPI URI: `ak.wwise.core.soundbank.getInclusions`.
                pub const getInclusions: &str = "ak.wwise.core.soundbank.getInclusions";
                /// Imports SoundBank definitions from the specified file. Multiple files can be specified. See the WAAPI log for status messages.
                ///
                /// WAAPI URI: `ak.wwise.core.soundbank.processDefinitionFiles`.
                pub const processDefinitionFiles: &str =
                    "ak.wwise.core.soundbank.processDefinitionFiles";
                /// Modifies a SoundBank's inclusion list. The 'operation' argument determines how the 'inclusions' argument modifies the SoundBank's inclusion list; 'inclusions' may be added to / removed from / replace the SoundBank's inclusion list.
                ///
                /// WAAPI URI: `ak.wwise.core.soundbank.setInclusions`.
                pub const setInclusions: &str = "ak.wwise.core.soundbank.setInclusions";
            }

            /// The `ak.wwise.core.sourceControl` WAAPI namespace.
            pub mod sourceControl {
                /// Add files to source control. Equivalent to Mark for Add for Perforce.
                ///
                /// WAAPI URI: `ak.wwise.core.sourceControl.add`.
                pub const add: &str = "ak.wwise.core.sourceControl.add";
                /// Check out files from source control. Equivalent to Check Out for Perforce.
                ///
                /// WAAPI URI: `ak.wwise.core.sourceControl.checkOut`.
                pub const checkOut: &str = "ak.wwise.core.sourceControl.checkOut";
                /// Commit files to source control. Equivalent to Submit Changes for Perforce.
                ///
                /// WAAPI URI: `ak.wwise.core.sourceControl.commit`.
                pub const commit: &str = "ak.wwise.core.sourceControl.commit";
                /// Delete files from source control. Equivalent to Mark for Delete for Perforce.
                ///
                /// WAAPI URI: `ak.wwise.core.sourceControl.delete`.
                pub const delete: &str = "ak.wwise.core.sourceControl.delete";
                /// Retrieve all original files.
                ///
                /// WAAPI URI: `ak.wwise.core.sourceControl.getSourceFiles`.
                pub const getSourceFiles: &str = "ak.wwise.core.sourceControl.getSourceFiles";
                /// Get the source control status of the specified files.
                ///
                /// WAAPI URI: `ak.wwise.core.sourceControl.getStatus`.
                pub const getStatus: &str = "ak.wwise.core.sourceControl.getStatus";
                /// Move or rename files in source control. Always pass the same number of elements in files and newFiles. Equivalent to Move for Perforce.
                ///
                /// WAAPI URI: `ak.wwise.core.sourceControl.move`.
                pub const move_: &str = "ak.wwise.core.sourceControl.move";
                /// Revert changes to files in source control.
                ///
                /// WAAPI URI: `ak.wwise.core.sourceControl.revert`.
                pub const revert: &str = "ak.wwise.core.sourceControl.revert";
                /// Change the source control provider and credentials. This is the same setting as the Source Control option in the Project Settings dialog in Wwise.
                ///
                /// WAAPI URI: `ak.wwise.core.sourceControl.setProvider`.
                pub const setProvider: &str = "ak.wwise.core.sourceControl.setProvider";
            }

            /// The `ak.wwise.core.switchContainer` WAAPI namespace.
            pub mod switchContainer {
                /// Assigns a Switch Container's child to a Switch. This is the equivalent of doing a drag&drop of the child to a state in the Assigned Objects view. The child is always added at the end for each state.
                ///
                /// WAAPI URI: `ak.wwise.core.switchContainer.addAssignment`.
                pub const addAssignment: &str = "ak.wwise.core.switchContainer.addAssignment";
                /// Returns the list of assignments between a Switch Container's children and states.
                ///
                /// WAAPI URI: `ak.wwise.core.switchContainer.getAssignments`.
                pub const getAssignments: &str = "ak.wwise.core.switchContainer.getAssignments";
                /// Removes an assignment between a Switch Container's child and a State.
                ///
                /// WAAPI URI: `ak.wwise.core.switchContainer.removeAssignment`.
                pub const removeAssignment: &str = "ak.wwise.core.switchContainer.removeAssignment";
            }

            /// The `ak.wwise.core.transport` WAAPI namespace.
            pub mod transport {
                /// Creates a transport object for the given Wwise object. The return transport object can be used to play, stop, pause and resume the Wwise object via the other transport functions.
                ///
                /// WAAPI URI: `ak.wwise.core.transport.create`.
                pub const create: &str = "ak.wwise.core.transport.create";
                /// Destroys the given transport object.
                ///
                /// WAAPI URI: `ak.wwise.core.transport.destroy`.
                pub const destroy: &str = "ak.wwise.core.transport.destroy";
                /// Executes an action on the given transport object, or all transport objects if none is specified.
                ///
                /// WAAPI URI: `ak.wwise.core.transport.executeAction`.
                pub const executeAction: &str = "ak.wwise.core.transport.executeAction";
                /// Returns the list of transport objects.
                ///
                /// WAAPI URI: `ak.wwise.core.transport.getList`.
                pub const getList: &str = "ak.wwise.core.transport.getList";
                /// Gets the state of the given transport object.
                ///
                /// WAAPI URI: `ak.wwise.core.transport.getState`.
                pub const getState: &str = "ak.wwise.core.transport.getState";
                /// Prepare the object and its dependencies for playback. Use this function before calling PostEventSync or PostMIDIOnEventSync from IAkGlobalPluginContext.
                ///
                /// WAAPI URI: `ak.wwise.core.transport.prepare`.
                pub const prepare: &str = "ak.wwise.core.transport.prepare";
                /// Sets the Original/Converted transport toggle globally. This allows playing the original or the converted sound files.
                ///
                /// WAAPI URI: `ak.wwise.core.transport.useOriginals`.
                pub const useOriginals: &str = "ak.wwise.core.transport.useOriginals";
            }

            /// The `ak.wwise.core.undo` WAAPI namespace.
            pub mod undo {
                /// Begins an undo group. Make sure to call `ak.wwise.core.undo.endGroup` exactly once for every ak.wwise.core.beginUndoGroup call you make. Calls to ak.wwise.core.undo.beginGroup can be nested. When closing a WAMP session, a check is made to ensure that all undo groups are closed. If not, a cancelGroup is called for each of the groups still open.
                ///
                /// WAAPI URI: `ak.wwise.core.undo.beginGroup`.
                pub const beginGroup: &str = "ak.wwise.core.undo.beginGroup";
                /// Cancels the last undo group.
                ///
                /// WAAPI URI: `ak.wwise.core.undo.cancelGroup`.
                pub const cancelGroup: &str = "ak.wwise.core.undo.cancelGroup";
                /// Ends the last undo group.
                ///
                /// WAAPI URI: `ak.wwise.core.undo.endGroup`.
                pub const endGroup: &str = "ak.wwise.core.undo.endGroup";
                /// Redoes the last operation in the Undo stack.
                ///
                /// WAAPI URI: `ak.wwise.core.undo.redo`.
                pub const redo: &str = "ak.wwise.core.undo.redo";
                /// Undoes the last operation in the Undo stack.
                ///
                /// WAAPI URI: `ak.wwise.core.undo.undo`.
                pub const undo: &str = "ak.wwise.core.undo.undo";
            }

            /// The `ak.wwise.core.workUnit` WAAPI namespace.
            pub mod workUnit {
                /// Load a Work Unit that was previously unloaded. The Undo history will be cleared.
                ///
                /// WAAPI URI: `ak.wwise.core.workUnit.load`.
                pub const load: &str = "ak.wwise.core.workUnit.load";
                /// Unload a Work Unit. No object contained in the Work Unit will be available after this call. If the Work Unit is modified and not saved, the function will return error. The Undo history will be cleared.
                ///
                /// WAAPI URI: `ak.wwise.core.workUnit.unload`.
                pub const unload: &str = "ak.wwise.core.workUnit.unload";
            }
        }
        /// The `ak.wwise.debug` WAAPI namespace.
        pub mod debug {
            /// Enables debug assertions. Every call to enableAsserts with 'false' increments the ref count. Calling with true decrements the ref count. This is only available with Debug builds.
            ///
            /// WAAPI URI: `ak.wwise.debug.enableAsserts`.
            pub const enableAsserts: &str = "ak.wwise.debug.enableAsserts";
            /// Enables or disables the automation mode for Wwise. This reduces the potential interruptions caused by message boxes and dialogs. For instance, enabling the automation mode silently accepts: project migration, project load log, EULA acceptance, project licence display and generic message boxes.
            ///
            /// WAAPI URI: `ak.wwise.debug.enableAutomationMode`.
            pub const enableAutomationMode: &str = "ak.wwise.debug.enableAutomationMode";
            /// Generate a WAV file playing a tone with a simple envelope and save it to the specified location. This is provided as a utility to generate test WAV files.
            ///
            /// WAAPI URI: `ak.wwise.debug.generateToneWAV`.
            pub const generateToneWAV: &str = "ak.wwise.debug.generateToneWAV";
            /// Retrieves the WAL tree, which describes the nodes that are synchronized in the Sound Engine. Private use only.
            ///
            /// WAAPI URI: `ak.wwise.debug.getWalTree`.
            pub const getWalTree: &str = "ak.wwise.debug.getWalTree";
            /// Restart WAAPI servers. For internal use only.
            ///
            /// WAAPI URI: `ak.wwise.debug.restartWaapiServers`.
            pub const restartWaapiServers: &str = "ak.wwise.debug.restartWaapiServers";
            /// Private use only.
            ///
            /// WAAPI URI: `ak.wwise.debug.testAssert`.
            pub const testAssert: &str = "ak.wwise.debug.testAssert";
            /// Private use only.
            ///
            /// WAAPI URI: `ak.wwise.debug.testCrash`.
            pub const testCrash: &str = "ak.wwise.debug.testCrash";
            /// Validate the arguments, options and result of a WAAPI function call. Does not actually call the function. This is only available with Debug builds.
            ///
            /// WAAPI URI: `ak.wwise.debug.validateCall`.
            pub const validateCall: &str = "ak.wwise.debug.validateCall";
        }
        /// The `ak.wwise.ui` WAAPI namespace.
        pub mod ui {
            /// Bring Wwise main window to foreground. See SetForegroundWindow and AllowSetForegroundWindow on MSDN for more information on the restrictions. See ak.wwise.core.getInfo to obtain the Wwise process ID for AllowSetForegroundWindow.
            ///
            /// WAAPI URI: `ak.wwise.ui.bringToForeground`.
            pub const bringToForeground: &str = "ak.wwise.ui.bringToForeground";
            /// Captures a part of the Wwise UI relative to a view.
            ///
            /// WAAPI URI: `ak.wwise.ui.captureScreen`.
            pub const captureScreen: &str = "ak.wwise.ui.captureScreen";
            /// Retrieves the list of files currently selected by the user in the active view. Note that this function is not available in WwiseConsole.
            ///
            /// WAAPI URI: `ak.wwise.ui.getSelectedFiles`.
            pub const getSelectedFiles: &str = "ak.wwise.ui.getSelectedFiles";
            /// Retrieves the list of objects currently selected by the user in the active view. Note that this function is not available in WwiseConsole.
            ///
            /// WAAPI URI: `ak.wwise.ui.getSelectedObjects`.
            pub const getSelectedObjects: &str = "ak.wwise.ui.getSelectedObjects";

            /// The `ak.wwise.ui.cli` WAAPI namespace.
            pub mod cli {
                /// Execute a Lua script. Optionally, specify additional Lua search paths, additional modules, and additional Lua scripts to load prior to the main script. The script can return a value. All arguments will be passed to the Lua script in the "wa_args" global variable.
                ///
                /// WAAPI URI: `ak.wwise.ui.cli.executeLuaScript`.
                pub const executeLuaScript: &str = "ak.wwise.ui.cli.executeLuaScript";
                /// Launch the main window of WwiseXP.
                ///
                /// WAAPI URI: `ak.wwise.ui.cli.launch`.
                pub const launch: &str = "ak.wwise.ui.cli.launch";
            }

            /// The `ak.wwise.ui.commands` WAAPI namespace.
            pub mod commands {
                /// Executes a command. Some commands can take a list of objects as parameters. See the Wwise global command IDs reference for the available commands.
                ///
                /// WAAPI URI: `ak.wwise.ui.commands.execute`.
                pub const execute: &str = "ak.wwise.ui.commands.execute";
                /// Gets the list of commands.
                ///
                /// WAAPI URI: `ak.wwise.ui.commands.getCommands`.
                pub const getCommands: &str = "ak.wwise.ui.commands.getCommands";
                /// Registers an array of add-on commands. Registered commands remain until the Wwise process is terminated. See the Wwise custom commands documentation for more information about registering commands. Also see `ak.wwise.ui.commands.executed`.
                ///
                /// WAAPI URI: `ak.wwise.ui.commands.register`.
                pub const register: &str = "ak.wwise.ui.commands.register";
                /// Unregisters an array of add-on UI commands.
                ///
                /// WAAPI URI: `ak.wwise.ui.commands.unregister`.
                pub const unregister: &str = "ak.wwise.ui.commands.unregister";
            }

            /// The `ak.wwise.ui.layout` WAAPI namespace.
            pub mod layout {
                /// Requests to close a view by its unique id. The view might not yet be closed when returning from this function.
                ///
                /// WAAPI URI: `ak.wwise.ui.layout.closeView`.
                pub const closeView: &str = "ak.wwise.ui.layout.closeView";
                /// Dock a floating view into a layout.
                ///
                /// WAAPI URI: `ak.wwise.ui.layout.dockView`.
                pub const dockView: &str = "ak.wwise.ui.layout.dockView";
                /// Retrieves the current layout name.
                ///
                /// WAAPI URI: `ak.wwise.ui.layout.getCurrentLayoutName`.
                pub const getCurrentLayoutName: &str = "ak.wwise.ui.layout.getCurrentLayoutName";
                /// Retrieves the current allocated rectangle of a layout element. An empty rect is returned if the element is not found.
                ///
                /// WAAPI URI: `ak.wwise.ui.layout.getElementRectangle`.
                pub const getElementRectangle: &str = "ak.wwise.ui.layout.getElementRectangle";
                /// Serializes a specific layout into a JSON format.
                ///
                /// WAAPI URI: `ak.wwise.ui.layout.getLayout`.
                pub const getLayout: &str = "ak.wwise.ui.layout.getLayout";
                /// Retrieves a list of the factory layout names.
                ///
                /// WAAPI URI: `ak.wwise.ui.layout.getLayoutNames`.
                pub const getLayoutNames: &str = "ak.wwise.ui.layout.getLayoutNames";
                /// Gets a view, if it exists in the current layout, or creates a new one.
                ///
                /// WAAPI URI: `ak.wwise.ui.layout.getOrCreateView`.
                pub const getOrCreateView: &str = "ak.wwise.ui.layout.getOrCreateView";
                /// Gets the frontend handle of an instantiated view.
                ///
                /// WAAPI URI: `ak.wwise.ui.layout.getViewHandle`.
                pub const getViewHandle: &str = "ak.wwise.ui.layout.getViewHandle";
                /// Retrieves a list of all view instances of a layout.
                ///
                /// WAAPI URI: `ak.wwise.ui.layout.getViewInstances`.
                pub const getViewInstances: &str = "ak.wwise.ui.layout.getViewInstances";
                /// Retrieves a list of all view types registered in Wwise.
                ///
                /// WAAPI URI: `ak.wwise.ui.layout.getViewTypes`.
                pub const getViewTypes: &str = "ak.wwise.ui.layout.getViewTypes";
                /// Moves a splitter by a delta given in pixels.
                ///
                /// WAAPI URI: `ak.wwise.ui.layout.moveSplitter`.
                pub const moveSplitter: &str = "ak.wwise.ui.layout.moveSplitter";
                /// Unregisters a temporary layout, previously registered with `ak.wwise.ui.layout.setLayout`.
                ///
                /// WAAPI URI: `ak.wwise.ui.layout.removeLayout`.
                pub const removeLayout: &str = "ak.wwise.ui.layout.removeLayout";
                /// Reset layouts to their default state.
                ///
                /// WAAPI URI: `ak.wwise.ui.layout.resetLayouts`.
                pub const resetLayouts: &str = "ak.wwise.ui.layout.resetLayouts";
                /// Registers a new layout from a JSON format.
                ///
                /// WAAPI URI: `ak.wwise.ui.layout.setLayout`.
                pub const setLayout: &str = "ak.wwise.ui.layout.setLayout";
                /// Switches the current layout.
                ///
                /// WAAPI URI: `ak.wwise.ui.layout.switchLayout`.
                pub const switchLayout: &str = "ak.wwise.ui.layout.switchLayout";
                /// Undock a view from a layout.
                ///
                /// WAAPI URI: `ak.wwise.ui.layout.undockView`.
                pub const undockView: &str = "ak.wwise.ui.layout.undockView";
            }

            /// The `ak.wwise.ui.model` WAAPI namespace.
            pub mod model {
                /// Creates a frontend handle and instantiates its widgets.
                ///
                /// WAAPI URI: `ak.wwise.ui.model.createHandle`.
                pub const createHandle: &str = "ak.wwise.ui.model.createHandle";
                /// Destroys a frontend handle and its widgets.
                ///
                /// WAAPI URI: `ak.wwise.ui.model.destroyHandle`.
                pub const destroyHandle: &str = "ak.wwise.ui.model.destroyHandle";
                /// Gets the state of widgets.
                ///
                /// WAAPI URI: `ak.wwise.ui.model.getState`.
                pub const getState: &str = "ak.wwise.ui.model.getState";
                /// Registers WAFM files and inline widget descriptors.
                ///
                /// WAAPI URI: `ak.wwise.ui.model.registerWafm`.
                pub const registerWafm: &str = "ak.wwise.ui.model.registerWafm";
                /// Sets the state of widgets.
                ///
                /// WAAPI URI: `ak.wwise.ui.model.setState`.
                pub const setState: &str = "ak.wwise.ui.model.setState";
            }

            /// The `ak.wwise.ui.project` WAAPI namespace.
            pub mod project {
                /// Closes the current project.
                ///
                /// WAAPI URI: `ak.wwise.ui.project.close`.
                pub const close: &str = "ak.wwise.ui.project.close";
                /// Creates, saves and opens new empty project, specified by path and platform. The project has no factory setting WorkUnit. Please see `ak.wwise.core.project.loaded` for further explanations on how to be notified when the operation has completed.
                ///
                /// WAAPI URI: `ak.wwise.ui.project.create`.
                pub const create: &str = "ak.wwise.ui.project.create";
                /// Opens a project, specified by path. Please see `ak.wwise.core.project.loaded` for further explanations on how to be notified when the operation has completed.
                ///
                /// WAAPI URI: `ak.wwise.ui.project.open`.
                pub const open: &str = "ak.wwise.ui.project.open";
            }

            /// The `ak.wwise.ui.signal` WAAPI namespace.
            pub mod signal {
                /// Emits a signal on a widget.
                ///
                /// WAAPI URI: `ak.wwise.ui.signal.emit`.
                pub const emit: &str = "ak.wwise.ui.signal.emit";
            }

            /// The `ak.wwise.ui.window` WAAPI namespace.
            pub mod window {
                /// Closes a window and unrealizes its widgets.
                ///
                /// WAAPI URI: `ak.wwise.ui.window.close`.
                pub const close: &str = "ak.wwise.ui.window.close";
                /// Creates a window in which a frontend handle can be instantiated.
                ///
                /// WAAPI URI: `ak.wwise.ui.window.create`.
                pub const create: &str = "ak.wwise.ui.window.create";
                /// Presents a window and realizes its widgets.
                ///
                /// WAAPI URI: `ak.wwise.ui.window.present`.
                pub const present: &str = "ak.wwise.ui.window.present";
            }
        }
        /// The `ak.wwise.waapi` WAAPI namespace.
        pub mod waapi {
            /// Retrieves the list of functions.
            ///
            /// WAAPI URI: `ak.wwise.waapi.getFunctions`.
            pub const getFunctions: &str = "ak.wwise.waapi.getFunctions";
            /// Retrieves the JSON schema of a Waapi Function or Topic URI.
            ///
            /// WAAPI URI: `ak.wwise.waapi.getSchema`.
            pub const getSchema: &str = "ak.wwise.waapi.getSchema";
            /// Retrieves the list of topics to which a client can subscribe.
            ///
            /// WAAPI URI: `ak.wwise.waapi.getTopics`.
            pub const getTopics: &str = "ak.wwise.waapi.getTopics";
        }
    }
}

/// Typed URI enum for `WaapiFunction` endpoints.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WaapiFunction {
    /// Executes an action on all nodes that are referenced in the specified event in a Play action. See `AK::SoundEngine::ExecuteActionOnEvent`.
    ///
    /// WAAPI URI: `ak.soundengine.executeActionOnEvent`.
    ak_soundengine_executeActionOnEvent,
    /// Gets the current state of a State Group. When using setState just prior to getState, allow a brief delay (no more than 10ms) for the information to update in the sound engine.
    ///
    /// WAAPI URI: `ak.soundengine.getState`.
    ak_soundengine_getState,
    /// Gets the current state of a Switch Group for a given Game Object.
    ///
    /// WAAPI URI: `ak.soundengine.getSwitch`.
    ak_soundengine_getSwitch,
    /// Load a SoundBank. See `AK::SoundEngine::LoadBank`.
    ///
    /// WAAPI URI: `ak.soundengine.loadBank`.
    ak_soundengine_loadBank,
    /// Asynchronously post an Event to the sound engine (by event ID). See `AK::SoundEngine::PostEvent`.
    ///
    /// WAAPI URI: `ak.soundengine.postEvent`.
    ak_soundengine_postEvent,
    /// Display a message in the Profiler's Capture Log view.
    ///
    /// WAAPI URI: `ak.soundengine.postMsgMonitor`.
    ak_soundengine_postMsgMonitor,
    /// Posts the specified Trigger. See `AK::SoundEngine::PostTrigger`.
    ///
    /// WAAPI URI: `ak.soundengine.postTrigger`.
    ak_soundengine_postTrigger,
    /// Register a game object. Registering a game object twice does nothing. Unregistering it once unregisters it no matter how many times it has been registered. See `AK::SoundEngine::RegisterGameObj`.
    ///
    /// WAAPI URI: `ak.soundengine.registerGameObj`.
    ak_soundengine_registerGameObj,
    /// Resets the value of a real-time parameter control to its default value, as specified in the Wwise project. See `AK::SoundEngine::ResetRTPCValue`.
    ///
    /// WAAPI URI: `ak.soundengine.resetRTPCValue`.
    ak_soundengine_resetRTPCValue,
    /// Seeks inside all playing objects that are referenced in Play Actions of the specified Event. See `AK::SoundEngine::SeekOnEvent`.
    ///
    /// WAAPI URI: `ak.soundengine.seekOnEvent`.
    ak_soundengine_seekOnEvent,
    /// Sets the default active listeners for all subsequent game objects that are registered. See `AK::SoundEngine::SetDefaultListeners`.
    ///
    /// WAAPI URI: `ak.soundengine.setDefaultListeners`.
    ak_soundengine_setDefaultListeners,
    /// Sets the Auxiliary Busses to route the specified game object. See `AK::SoundEngine::SetGameObjectAuxSendValues`.
    ///
    /// WAAPI URI: `ak.soundengine.setGameObjectAuxSendValues`.
    ak_soundengine_setGameObjectAuxSendValues,
    /// Set the output bus volume (direct) to be used for the specified game object. See `AK::SoundEngine::SetGameObjectOutputBusVolume`.
    ///
    /// WAAPI URI: `ak.soundengine.setGameObjectOutputBusVolume`.
    ak_soundengine_setGameObjectOutputBusVolume,
    /// Sets a listener's spatialization parameters. This lets you define listener-specific volume offsets for each audio channel. See `AK::SoundEngine::SetListenerSpatialization`.
    ///
    /// WAAPI URI: `ak.soundengine.setListenerSpatialization`.
    ak_soundengine_setListenerSpatialization,
    /// Sets a single game object's active listeners. By default, all new game objects have no listeners active, but this behavior can be overridden with `SetDefaultListeners()`. Inactive listeners are not computed. See `AK::SoundEngine::SetListeners`.
    ///
    /// WAAPI URI: `ak.soundengine.setListeners`.
    ak_soundengine_setListeners,
    /// Sets multiple positions for a single game object. Setting multiple positions for a single game object is a way to simulate multiple emission sources while using the resources of only one voice. This can be used to simulate wall openings, area sounds, or multiple objects emitting the same sound in the same area. See `AK::SoundEngine::SetMultiplePositions`.
    ///
    /// WAAPI URI: `ak.soundengine.setMultiplePositions`.
    ak_soundengine_setMultiplePositions,
    /// Set a game object's obstruction and occlusion levels. This function is used to affect how an object should be heard by a specific listener. See `AK::SoundEngine::SetObjectObstructionAndOcclusion`.
    ///
    /// WAAPI URI: `ak.soundengine.setObjectObstructionAndOcclusion`.
    ak_soundengine_setObjectObstructionAndOcclusion,
    /// Sets the position of a game object. See `AK::SoundEngine::SetPosition`.
    ///
    /// WAAPI URI: `ak.soundengine.setPosition`.
    ak_soundengine_setPosition,
    /// Sets the value of a Game Parameter for real-time parameter control. See `AK::SoundEngine::SetRTPCValue`.
    ///
    /// WAAPI URI: `ak.soundengine.setRTPCValue`.
    ak_soundengine_setRTPCValue,
    /// Sets the scaling factor of a game object. You can modify the attenuation computations on this game object to simulate sounds with a larger or smaller affected areas. See `AK::SoundEngine::SetScalingFactor`.
    ///
    /// WAAPI URI: `ak.soundengine.setScalingFactor`.
    ak_soundengine_setScalingFactor,
    /// Sets the State of a State Group. See `AK::SoundEngine::SetState`.
    ///
    /// WAAPI URI: `ak.soundengine.setState`.
    ak_soundengine_setState,
    /// Sets the State of a Switch Group. See `AK::SoundEngine::SetSwitch`.
    ///
    /// WAAPI URI: `ak.soundengine.setSwitch`.
    ak_soundengine_setSwitch,
    /// Stop playing the current content associated to the specified game object ID. If no game object is specified, all sounds are stopped. See `AK::SoundEngine::StopAll`.
    ///
    /// WAAPI URI: `ak.soundengine.stopAll`.
    ak_soundengine_stopAll,
    /// Stops the current content, associated to the specified playing ID, from playing. See `AK::SoundEngine::StopPlayingID`.
    ///
    /// WAAPI URI: `ak.soundengine.stopPlayingID`.
    ak_soundengine_stopPlayingID,
    /// Unload a SoundBank. See `AK::SoundEngine::UnloadBank`.
    ///
    /// WAAPI URI: `ak.soundengine.unloadBank`.
    ak_soundengine_unloadBank,
    /// Unregisters a game object. Registering a game object twice does nothing. Unregistering it once unregisters it no matter how many times it has been registered. Unregistering a game object while it is in use is allowed, but the control over the parameters of this game object is lost. For example, say a sound associated with this game object is a 3D moving sound. It stops moving when the game object is unregistered, and there is no way to regain control over the game object. See `AK::SoundEngine::UnregisterGameObj`.
    ///
    /// WAAPI URI: `ak.soundengine.unregisterGameObj`.
    ak_soundengine_unregisterGameObj,
    /// Creates a converted audio file. When errors occur, this function returns a list of messages with corresponding levels of severity.
    ///
    /// WAAPI URI: `ak.wwise.core.audio.convert`.
    ak_wwise_core_audio_convert,
    /// Creates Wwise objects and imports audio files. This function does not return an error when something fails during the import process, please see the log for the result of each import command. This function uses the same importation processor available through the Tab Delimited import in the Audio File Importer. The function returns an array of all objects created, replaced or re-used. Use the options to specify how the objects are returned. For more information, see the WAAPI Import documentation.
    ///
    /// WAAPI URI: `ak.wwise.core.audio.import`.
    ak_wwise_core_audio_import,
    /// Scripted object creation and audio file import from a tab-delimited file.
    ///
    /// WAAPI URI: `ak.wwise.core.audio.importTabDelimited`.
    ak_wwise_core_audio_importTabDelimited,
    /// Mutes an object.
    ///
    /// WAAPI URI: `ak.wwise.core.audio.mute`.
    ak_wwise_core_audio_mute,
    /// Unmute all muted objects.
    ///
    /// WAAPI URI: `ak.wwise.core.audio.resetMute`.
    ak_wwise_core_audio_resetMute,
    /// Unsolo all soloed objects.
    ///
    /// WAAPI URI: `ak.wwise.core.audio.resetSolo`.
    ak_wwise_core_audio_resetSolo,
    /// Changes the plug-in to use for audio file conversion.
    ///
    /// WAAPI URI: `ak.wwise.core.audio.setConversionPlugin`.
    ak_wwise_core_audio_setConversionPlugin,
    /// Solos an object.
    ///
    /// WAAPI URI: `ak.wwise.core.audio.solo`.
    ak_wwise_core_audio_solo,
    /// Gets the min/max peak pairs, in the given region of an audio source, as a collection of binary strings (one per channel). The strings are base-64 encoded, 16-bit signed int arrays, with min and max values being interleaved. If getCrossChannelPeaks is true, only one binary string represents the peaks across all channels globally.
    ///
    /// WAAPI URI: `ak.wwise.core.audioSourcePeaks.getMinMaxPeaksInRegion`.
    ak_wwise_core_audioSourcePeaks_getMinMaxPeaksInRegion,
    /// Gets the min/max peak pairs in the entire trimmed region of an audio source, for each channel, as an array of binary strings (one per channel). The strings are base-64 encoded, 16-bit signed int arrays, with min and max values being interleaved. If getCrossChannelPeaks is true, there is only one binary string representing peaks across all channels globally.
    ///
    /// WAAPI URI: `ak.wwise.core.audioSourcePeaks.getMinMaxPeaksInTrimmedRegion`.
    ak_wwise_core_audioSourcePeaks_getMinMaxPeaksInTrimmedRegion,
    /// Adds a new assignment to a Blend Track. Equivalent to performing a drag-and-drop operation in the Blend Tracks Editor.
    ///
    /// WAAPI URI: `ak.wwise.core.blendContainer.addAssignment`.
    ak_wwise_core_blendContainer_addAssignment,
    /// Adds a new Blend Track to a Blend Container. Equivalent to clicking New Blend Track in the Blend Track Editor. To get the list of Blend Tracks, use the function ak.wwise.core.object.get with {return = "blendTracks"}.
    ///
    /// WAAPI URI: `ak.wwise.core.blendContainer.addTrack`.
    ak_wwise_core_blendContainer_addTrack,
    /// Returns a list of a Blend Track assignments.
    ///
    /// WAAPI URI: `ak.wwise.core.blendContainer.getAssignments`.
    ak_wwise_core_blendContainer_getAssignments,
    /// Removes an assignment from a Blend Track. Equivalent to deleting an entry in the Blend Tracks Editor.
    ///
    /// WAAPI URI: `ak.wwise.core.blendContainer.removeAssignment`.
    ak_wwise_core_blendContainer_removeAssignment,
    /// Execute Lua code from a file path (using `luaScript`) or directly as a string (using `luaString`). Optionally, specify additional Lua search paths, additional modules, and additional Lua scripts to load prior to the main script. The script can return a value, which can be any of the Lua types, including tables. All WAAPI arguments will be passed to the Lua script in the `wa_args` global variable, as a Lua table. Define your own key-values in the WAAPI arguments and access them from the Lua code. Use `wa_call` to call a WAAPI function from the Lua script. See `ak.wwise.waapi.getFunctions` and `ak.wwise.waapi.getSchema` to learn more about the functions available and their schema before using `wa_call`. If both `luaScript` and `luaString` are provided, the script file will be executed first, followed by the string code. Note that the Lua environment is reset at each call to this function.
    ///
    /// WAAPI URI: `ak.wwise.core.executeLuaScript`.
    ak_wwise_core_executeLuaScript,
    /// Sets the Min and Max properties on a Game Parameter. Modifies the RTPC curves and blend tracks that use this Game Parameter for their X axis.
    ///
    /// WAAPI URI: `ak.wwise.core.gameParameter.setRange`.
    ak_wwise_core_gameParameter_setRange,
    /// Retrieve global Wwise information.
    ///
    /// WAAPI URI: `ak.wwise.core.getInfo`.
    ak_wwise_core_getInfo,
    /// Retrieve information about the current project opened, including platforms, languages and project directories.
    ///
    /// WAAPI URI: `ak.wwise.core.getProjectInfo`.
    ak_wwise_core_getProjectInfo,
    /// Adds a new item to the logs on the specified channel.
    ///
    /// WAAPI URI: `ak.wwise.core.log.addItem`.
    ak_wwise_core_log_addItem,
    /// Clears the logs on the specified channel.
    ///
    /// WAAPI URI: `ak.wwise.core.log.clear`.
    ak_wwise_core_log_clear,
    /// Retrieves the latest log for a specific channel. See `ak.wwise.core.log.itemAdded` to be notified when an item is added to the log. The log is empty when used in WwiseConsole.
    ///
    /// WAAPI URI: `ak.wwise.core.log.get`.
    ak_wwise_core_log_get,
    /// Retrieve files from Media Pool. Use the return options to specify which properties of the files to return.
    ///
    /// WAAPI URI: `ak.wwise.core.mediaPool.get`.
    ak_wwise_core_mediaPool_get,
    /// Retrieve all fields present in the Media Pool. You can then use the fields to query the Media Pool. The Media Pool discovers some fields when it scans audio files. Others, such as WAV fields, are always available.
    ///
    /// WAAPI URI: `ak.wwise.core.mediaPool.getFields`.
    ak_wwise_core_mediaPool_getFields,
    /// Copies an object to the given parent. Note that if a Work Unit is copied, the operation cannot be undone and the project will be saved.
    ///
    /// WAAPI URI: `ak.wwise.core.object.copy`.
    ak_wwise_core_object_copy,
    /// Creates an object of type 'type', as a child of 'parent'. See the WAAPI Import documentation for more information about creating objects. Also see `ak.wwise.core.audio.import` to import audio files to Wwise. To create Effect or Source plug-ins, use `ak.wwise.core.object.set`, and see the Wwise Objects Reference for the classId.
    ///
    /// WAAPI URI: `ak.wwise.core.object.create`.
    ak_wwise_core_object_create,
    /// Deletes the specified object. Note that if a Work Unit is deleted, the operation cannot be undone and the project will be saved.
    ///
    /// WAAPI URI: `ak.wwise.core.object.delete`.
    ak_wwise_core_object_delete,
    /// Compares properties and lists of the source object with those in the target object.
    ///
    /// WAAPI URI: `ak.wwise.core.object.diff`.
    ak_wwise_core_object_diff,
    /// Performs a query and returns the data, as specified in the options, for each object in the query result. The query can specify either a 'waql' argument or a 'from' argument with an optional 'transform' argument. See the WAQL introduction documentation or the WAAPI query documentation for more information. See the WAAPI query return options documentation to learn about options.
    ///
    /// WAAPI URI: `ak.wwise.core.object.get`.
    ak_wwise_core_object_get,
    /// Gets the specified attenuation curve for a given attenuation object.
    ///
    /// WAAPI URI: `ak.wwise.core.object.getAttenuationCurve`.
    ak_wwise_core_object_getAttenuationCurve,
    /// Retrieves the list of property and reference names for an object.
    ///
    /// WAAPI URI: `ak.wwise.core.object.getPropertyAndReferenceNames`.
    ak_wwise_core_object_getPropertyAndReferenceNames,
    /// Retrieves information about an object property. Note that this function does not return the value of a property. To retrieve the value of a property, see `ak.wwise.core.object.get` and the WAAPI query return options documentation.
    ///
    /// WAAPI URI: `ak.wwise.core.object.getPropertyInfo`.
    ak_wwise_core_object_getPropertyInfo,
    /// Retrieves the list of all object types registered in Wwise's object model. This function returns the equivalent of the Wwise Objects Reference.
    ///
    /// WAAPI URI: `ak.wwise.core.object.getTypes`.
    ak_wwise_core_object_getTypes,
    /// Indicates whether a property, reference, or object list is bound to a particular platform or to all platforms.
    ///
    /// WAAPI URI: `ak.wwise.core.object.isLinked`.
    ak_wwise_core_object_isLinked,
    /// Returns true if a property is enabled based on the values of the properties it depends on.
    ///
    /// WAAPI URI: `ak.wwise.core.object.isPropertyEnabled`.
    ak_wwise_core_object_isPropertyEnabled,
    /// Moves an object to the given parent. Returns the moved object.
    ///
    /// WAAPI URI: `ak.wwise.core.object.move`.
    ak_wwise_core_object_move,
    /// Pastes properties, references and lists from one object to any number of target objects. Only those properties, references and lists which differ between source and target are pasted. See the Wwise Objects Reference for more information on the properties, references and lists available on each object type.
    ///
    /// WAAPI URI: `ak.wwise.core.object.pasteProperties`.
    ak_wwise_core_object_pasteProperties,
    /// Allows for batch processing of the following operations: Object creation in a child hierarchy, Object creation in a list, Setting name, notes, properties and references. See the WAAPI Import documentation for more information about creating objects. Also see `ak.wwise.core.audio.import` to import audio files to Wwise.
    ///
    /// WAAPI URI: `ak.wwise.core.object.set`.
    ak_wwise_core_object_set,
    /// Sets the specified attenuation curve for a given attenuation object.
    ///
    /// WAAPI URI: `ak.wwise.core.object.setAttenuationCurve`.
    ak_wwise_core_object_setAttenuationCurve,
    /// Link or unlink a property/reference or object list to a particular platform.
    ///
    /// WAAPI URI: `ak.wwise.core.object.setLinked`.
    ak_wwise_core_object_setLinked,
    /// Renames an object.
    ///
    /// WAAPI URI: `ak.wwise.core.object.setName`.
    ak_wwise_core_object_setName,
    /// Sets the object's notes.
    ///
    /// WAAPI URI: `ak.wwise.core.object.setNotes`.
    ak_wwise_core_object_setNotes,
    /// Sets a property value of an object for a specific platform. See the Wwise Objects Reference for more information on the properties available on each object type. See `ak.wwise.core.object.setReference` to set a reference to an object. See `ak.wwise.core.object.get` to obtain the value of a property for an object.
    ///
    /// WAAPI URI: `ak.wwise.core.object.setProperty`.
    ak_wwise_core_object_setProperty,
    /// Sets the randomizer values of a property of an object for a specific platform. See the Wwise Objects Reference for more information on the properties available on each object type.
    ///
    /// WAAPI URI: `ak.wwise.core.object.setRandomizer`.
    ak_wwise_core_object_setRandomizer,
    /// Sets an object's reference value. See the Wwise Objects Reference for more information on the references available on each object type.
    ///
    /// WAAPI URI: `ak.wwise.core.object.setReference`.
    ak_wwise_core_object_setReference,
    /// Sets the State Group objects associated with an object. Note, this will remove any previously associated State Group.
    ///
    /// WAAPI URI: `ak.wwise.core.object.setStateGroups`.
    ak_wwise_core_object_setStateGroups,
    /// Set the state properties of an object. Note, this will remove any previous state property, including the default ones.
    ///
    /// WAAPI URI: `ak.wwise.core.object.setStateProperties`.
    ak_wwise_core_object_setStateProperties,
    /// Verify if WAAPI is currently available.
    ///
    /// WAAPI URI: `ak.wwise.core.ping`.
    ak_wwise_core_ping,
    /// Saves the content of the capture log to a text file according to the given file path.
    ///
    /// WAAPI URI: `ak.wwise.core.profiler.captureLog.save`.
    ak_wwise_core_profiler_captureLog_save,
    /// Specifies the type of data you want to capture. Overrides the user's profiler settings.
    ///
    /// WAAPI URI: `ak.wwise.core.profiler.enableProfilerData`.
    ak_wwise_core_profiler_enableProfilerData,
    /// Retrieves the Audio Objects at a specific profiler capture time.
    ///
    /// WAAPI URI: `ak.wwise.core.profiler.getAudioObjects`.
    ak_wwise_core_profiler_getAudioObjects,
    /// Retrieves the busses at a specific profiler capture time.
    ///
    /// WAAPI URI: `ak.wwise.core.profiler.getBusses`.
    ak_wwise_core_profiler_getBusses,
    /// Retrieves CPU usage statistics at a specific profiler capture time. This data can also be found in the Advanced Profiler, under the CPU tab. To ensure the CPU data is received, see `ak.wwise.core.profiler.enableProfilerData`. The returned data includes "Inclusive" and "Exclusive" values, where "Inclusive" refers to the time spent in the element plus the time spent in any called elements, and "Exclusive" values pertain to execution only within the element itself.
    ///
    /// WAAPI URI: `ak.wwise.core.profiler.getCpuUsage`.
    ak_wwise_core_profiler_getCpuUsage,
    /// Returns the current time of the specified profiler cursor, in milliseconds.
    ///
    /// WAAPI URI: `ak.wwise.core.profiler.getCursorTime`.
    ak_wwise_core_profiler_getCursorTime,
    /// Retrieves the game objects at a specific profiler capture time.
    ///
    /// WAAPI URI: `ak.wwise.core.profiler.getGameObjects`.
    ak_wwise_core_profiler_getGameObjects,
    /// Retrieves the loaded media at a specific profiler capture time. This data can also be found in the Advanced Profiler, under the Loaded Media tab. To ensure the Loaded Media data is received, see `ak.wwise.core.profiler.enableProfilerData`.
    ///
    /// WAAPI URI: `ak.wwise.core.profiler.getLoadedMedia`.
    ak_wwise_core_profiler_getLoadedMedia,
    /// Retrieves the Meter data for all registered busses, aux busses and devices. Only the Main Audio Bus is registered by default. Use `ak.wwise.core.profiler.registerMeter` for other busses, before retrieval of the meter data.
    ///
    /// WAAPI URI: `ak.wwise.core.profiler.getMeters`.
    ak_wwise_core_profiler_getMeters,
    /// Retrieves the Performance Monitor statistics at a specific profiler capture time. See the Wwise global counter IDs reference for the available counters.
    ///
    /// WAAPI URI: `ak.wwise.core.profiler.getPerformanceMonitor`.
    ak_wwise_core_profiler_getPerformanceMonitor,
    /// Retrieves active RTPCs at a specific profiler capture time.
    ///
    /// WAAPI URI: `ak.wwise.core.profiler.getRTPCs`.
    ak_wwise_core_profiler_getRTPCs,
    /// Retrieves the streaming media at a specific profiler capture time. This data can also be found in the Advanced Profiler, under the Streams tab. To ensure the Streams data is received, see `ak.wwise.core.profiler.enableProfilerData`.
    ///
    /// WAAPI URI: `ak.wwise.core.profiler.getStreamedMedia`.
    ak_wwise_core_profiler_getStreamedMedia,
    /// Retrieves all parameters affecting voice volume, highpass and lowpass for a voice path, resolved from pipeline IDs.
    ///
    /// WAAPI URI: `ak.wwise.core.profiler.getVoiceContributions`.
    ak_wwise_core_profiler_getVoiceContributions,
    /// Retrieves the voices at a specific profiler capture time.
    ///
    /// WAAPI URI: `ak.wwise.core.profiler.getVoices`.
    ak_wwise_core_profiler_getVoices,
    /// Moves the user time cursor of the profiler to a specific position (first, last, next, previous), and returns the new time. This sets the cursor position in the profiler views.
    ///
    /// WAAPI URI: `ak.wwise.core.profiler.moveCursor`.
    ak_wwise_core_profiler_moveCursor,
    /// Registers a bus, an aux bus or device to receive meter data. Only the Main Audio Bus is registered by default. Use `ak.wwise.core.profiler.getMeters` to retrieve the meter data after registering. Every call to ak.wwise.core.profiler.registerMeter must have a matching call to `ak.wwise.core.profiler.unregisterMeter`.
    ///
    /// WAAPI URI: `ak.wwise.core.profiler.registerMeter`.
    ak_wwise_core_profiler_registerMeter,
    /// Saves profiler as a.prof file according to the given file path.
    ///
    /// WAAPI URI: `ak.wwise.core.profiler.saveCapture`.
    ak_wwise_core_profiler_saveCapture,
    /// Sets the user time cursor of the profiler to the specified time, in milliseconds, and returns the new time. Use the time obtained from the other profiler functions. Note that the specified time will be snapped to a buffer interval (~10ms). This sets the cursor position in the profiler views. To synchronize the user time cursor to the latest capture time, use `ak.wwise.ui.commands.execute` with `ToggleFollowCapture`.
    ///
    /// WAAPI URI: `ak.wwise.core.profiler.setCursorTime`.
    ak_wwise_core_profiler_setCursorTime,
    /// Starts the profiler capture and returns the time at the beginning of the capture, in milliseconds.
    ///
    /// WAAPI URI: `ak.wwise.core.profiler.startCapture`.
    ak_wwise_core_profiler_startCapture,
    /// Stops the profiler capture and returns the time at the end of the capture, in milliseconds.
    ///
    /// WAAPI URI: `ak.wwise.core.profiler.stopCapture`.
    ak_wwise_core_profiler_stopCapture,
    /// Unregisters a bus or device that was registered with `ak.wwise.core.profiler.registerMeter`.
    ///
    /// WAAPI URI: `ak.wwise.core.profiler.unregisterMeter`.
    ak_wwise_core_profiler_unregisterMeter,
    /// Saves the current project.
    ///
    /// WAAPI URI: `ak.wwise.core.project.save`.
    ak_wwise_core_project_save,
    /// Connects the Wwise Authoring application to a Wwise Sound Engine running executable or to a saved profile file. The host must be running code with communication enabled. If only "host" is provided, Wwise connects to the first Sound Engine instance found. To distinguish between different instances, you can also provide the name of the application to connect to.
    ///
    /// WAAPI URI: `ak.wwise.core.remote.connect`.
    ak_wwise_core_remote_connect,
    /// Disconnects the Wwise Authoring application from a connected Wwise Sound Engine running executable.
    ///
    /// WAAPI URI: `ak.wwise.core.remote.disconnect`.
    ak_wwise_core_remote_disconnect,
    /// Retrieves all consoles available for connecting Wwise Authoring to a Sound Engine instance.
    ///
    /// WAAPI URI: `ak.wwise.core.remote.getAvailableConsoles`.
    ak_wwise_core_remote_getAvailableConsoles,
    /// Retrieves the connection status.
    ///
    /// WAAPI URI: `ak.wwise.core.remote.getConnectionStatus`.
    ak_wwise_core_remote_getConnectionStatus,
    /// Sets which version of the source is being used for the specified sound. Use `ak.wwise.core.object.get` with the 'activeSource' return option to get the active source of a sound.
    ///
    /// WAAPI URI: `ak.wwise.core.sound.setActiveSource`.
    ak_wwise_core_sound_setActiveSource,
    /// Converts the external sources files for the project as detailed in the wsources file, and places them into either the default folder, or the folder specified by the output argument. External Sources are a special type of source that you can put in a Sound object in Wwise. It indicates that the real sound data will be provided at run time. While External Source conversion is also triggered by SoundBank generation, this operation can be used to process sources not contained in the Wwise Project. Please see Wwise SDK help page "Integrating External Sources".
    ///
    /// WAAPI URI: `ak.wwise.core.soundbank.convertExternalSources`.
    ak_wwise_core_soundbank_convertExternalSources,
    /// Generate a list of SoundBanks with the import definition specified in the WAAPI request. If you do not write the SoundBanks to disk, subscribe to `ak.wwise.core.soundbank.generated` to receive SoundBank structure info and the bank data as base64. Note: This is a synchronous operation.
    ///
    /// WAAPI URI: `ak.wwise.core.soundbank.generate`.
    ak_wwise_core_soundbank_generate,
    /// Retrieves a SoundBank's inclusion list.
    ///
    /// WAAPI URI: `ak.wwise.core.soundbank.getInclusions`.
    ak_wwise_core_soundbank_getInclusions,
    /// Imports SoundBank definitions from the specified file. Multiple files can be specified. See the WAAPI log for status messages.
    ///
    /// WAAPI URI: `ak.wwise.core.soundbank.processDefinitionFiles`.
    ak_wwise_core_soundbank_processDefinitionFiles,
    /// Modifies a SoundBank's inclusion list. The 'operation' argument determines how the 'inclusions' argument modifies the SoundBank's inclusion list; 'inclusions' may be added to / removed from / replace the SoundBank's inclusion list.
    ///
    /// WAAPI URI: `ak.wwise.core.soundbank.setInclusions`.
    ak_wwise_core_soundbank_setInclusions,
    /// Add files to source control. Equivalent to Mark for Add for Perforce.
    ///
    /// WAAPI URI: `ak.wwise.core.sourceControl.add`.
    ak_wwise_core_sourceControl_add,
    /// Check out files from source control. Equivalent to Check Out for Perforce.
    ///
    /// WAAPI URI: `ak.wwise.core.sourceControl.checkOut`.
    ak_wwise_core_sourceControl_checkOut,
    /// Commit files to source control. Equivalent to Submit Changes for Perforce.
    ///
    /// WAAPI URI: `ak.wwise.core.sourceControl.commit`.
    ak_wwise_core_sourceControl_commit,
    /// Delete files from source control. Equivalent to Mark for Delete for Perforce.
    ///
    /// WAAPI URI: `ak.wwise.core.sourceControl.delete`.
    ak_wwise_core_sourceControl_delete,
    /// Retrieve all original files.
    ///
    /// WAAPI URI: `ak.wwise.core.sourceControl.getSourceFiles`.
    ak_wwise_core_sourceControl_getSourceFiles,
    /// Get the source control status of the specified files.
    ///
    /// WAAPI URI: `ak.wwise.core.sourceControl.getStatus`.
    ak_wwise_core_sourceControl_getStatus,
    /// Move or rename files in source control. Always pass the same number of elements in files and newFiles. Equivalent to Move for Perforce.
    ///
    /// WAAPI URI: `ak.wwise.core.sourceControl.move`.
    ak_wwise_core_sourceControl_move,
    /// Revert changes to files in source control.
    ///
    /// WAAPI URI: `ak.wwise.core.sourceControl.revert`.
    ak_wwise_core_sourceControl_revert,
    /// Change the source control provider and credentials. This is the same setting as the Source Control option in the Project Settings dialog in Wwise.
    ///
    /// WAAPI URI: `ak.wwise.core.sourceControl.setProvider`.
    ak_wwise_core_sourceControl_setProvider,
    /// Assigns a Switch Container's child to a Switch. This is the equivalent of doing a drag&drop of the child to a state in the Assigned Objects view. The child is always added at the end for each state.
    ///
    /// WAAPI URI: `ak.wwise.core.switchContainer.addAssignment`.
    ak_wwise_core_switchContainer_addAssignment,
    /// Returns the list of assignments between a Switch Container's children and states.
    ///
    /// WAAPI URI: `ak.wwise.core.switchContainer.getAssignments`.
    ak_wwise_core_switchContainer_getAssignments,
    /// Removes an assignment between a Switch Container's child and a State.
    ///
    /// WAAPI URI: `ak.wwise.core.switchContainer.removeAssignment`.
    ak_wwise_core_switchContainer_removeAssignment,
    /// Creates a transport object for the given Wwise object. The return transport object can be used to play, stop, pause and resume the Wwise object via the other transport functions.
    ///
    /// WAAPI URI: `ak.wwise.core.transport.create`.
    ak_wwise_core_transport_create,
    /// Destroys the given transport object.
    ///
    /// WAAPI URI: `ak.wwise.core.transport.destroy`.
    ak_wwise_core_transport_destroy,
    /// Executes an action on the given transport object, or all transport objects if none is specified.
    ///
    /// WAAPI URI: `ak.wwise.core.transport.executeAction`.
    ak_wwise_core_transport_executeAction,
    /// Returns the list of transport objects.
    ///
    /// WAAPI URI: `ak.wwise.core.transport.getList`.
    ak_wwise_core_transport_getList,
    /// Gets the state of the given transport object.
    ///
    /// WAAPI URI: `ak.wwise.core.transport.getState`.
    ak_wwise_core_transport_getState,
    /// Prepare the object and its dependencies for playback. Use this function before calling PostEventSync or PostMIDIOnEventSync from IAkGlobalPluginContext.
    ///
    /// WAAPI URI: `ak.wwise.core.transport.prepare`.
    ak_wwise_core_transport_prepare,
    /// Sets the Original/Converted transport toggle globally. This allows playing the original or the converted sound files.
    ///
    /// WAAPI URI: `ak.wwise.core.transport.useOriginals`.
    ak_wwise_core_transport_useOriginals,
    /// Begins an undo group. Make sure to call `ak.wwise.core.undo.endGroup` exactly once for every ak.wwise.core.beginUndoGroup call you make. Calls to ak.wwise.core.undo.beginGroup can be nested. When closing a WAMP session, a check is made to ensure that all undo groups are closed. If not, a cancelGroup is called for each of the groups still open.
    ///
    /// WAAPI URI: `ak.wwise.core.undo.beginGroup`.
    ak_wwise_core_undo_beginGroup,
    /// Cancels the last undo group.
    ///
    /// WAAPI URI: `ak.wwise.core.undo.cancelGroup`.
    ak_wwise_core_undo_cancelGroup,
    /// Ends the last undo group.
    ///
    /// WAAPI URI: `ak.wwise.core.undo.endGroup`.
    ak_wwise_core_undo_endGroup,
    /// Redoes the last operation in the Undo stack.
    ///
    /// WAAPI URI: `ak.wwise.core.undo.redo`.
    ak_wwise_core_undo_redo,
    /// Undoes the last operation in the Undo stack.
    ///
    /// WAAPI URI: `ak.wwise.core.undo.undo`.
    ak_wwise_core_undo_undo,
    /// Load a Work Unit that was previously unloaded. The Undo history will be cleared.
    ///
    /// WAAPI URI: `ak.wwise.core.workUnit.load`.
    ak_wwise_core_workUnit_load,
    /// Unload a Work Unit. No object contained in the Work Unit will be available after this call. If the Work Unit is modified and not saved, the function will return error. The Undo history will be cleared.
    ///
    /// WAAPI URI: `ak.wwise.core.workUnit.unload`.
    ak_wwise_core_workUnit_unload,
    /// Enables debug assertions. Every call to enableAsserts with 'false' increments the ref count. Calling with true decrements the ref count. This is only available with Debug builds.
    ///
    /// WAAPI URI: `ak.wwise.debug.enableAsserts`.
    ak_wwise_debug_enableAsserts,
    /// Enables or disables the automation mode for Wwise. This reduces the potential interruptions caused by message boxes and dialogs. For instance, enabling the automation mode silently accepts: project migration, project load log, EULA acceptance, project licence display and generic message boxes.
    ///
    /// WAAPI URI: `ak.wwise.debug.enableAutomationMode`.
    ak_wwise_debug_enableAutomationMode,
    /// Generate a WAV file playing a tone with a simple envelope and save it to the specified location. This is provided as a utility to generate test WAV files.
    ///
    /// WAAPI URI: `ak.wwise.debug.generateToneWAV`.
    ak_wwise_debug_generateToneWAV,
    /// Retrieves the WAL tree, which describes the nodes that are synchronized in the Sound Engine. Private use only.
    ///
    /// WAAPI URI: `ak.wwise.debug.getWalTree`.
    ak_wwise_debug_getWalTree,
    /// Restart WAAPI servers. For internal use only.
    ///
    /// WAAPI URI: `ak.wwise.debug.restartWaapiServers`.
    ak_wwise_debug_restartWaapiServers,
    /// Private use only.
    ///
    /// WAAPI URI: `ak.wwise.debug.testAssert`.
    ak_wwise_debug_testAssert,
    /// Private use only.
    ///
    /// WAAPI URI: `ak.wwise.debug.testCrash`.
    ak_wwise_debug_testCrash,
    /// Validate the arguments, options and result of a WAAPI function call. Does not actually call the function. This is only available with Debug builds.
    ///
    /// WAAPI URI: `ak.wwise.debug.validateCall`.
    ak_wwise_debug_validateCall,
    /// Bring Wwise main window to foreground. See SetForegroundWindow and AllowSetForegroundWindow on MSDN for more information on the restrictions. See ak.wwise.core.getInfo to obtain the Wwise process ID for AllowSetForegroundWindow.
    ///
    /// WAAPI URI: `ak.wwise.ui.bringToForeground`.
    ak_wwise_ui_bringToForeground,
    /// Captures a part of the Wwise UI relative to a view.
    ///
    /// WAAPI URI: `ak.wwise.ui.captureScreen`.
    ak_wwise_ui_captureScreen,
    /// Execute a Lua script. Optionally, specify additional Lua search paths, additional modules, and additional Lua scripts to load prior to the main script. The script can return a value. All arguments will be passed to the Lua script in the "wa_args" global variable.
    ///
    /// WAAPI URI: `ak.wwise.ui.cli.executeLuaScript`.
    ak_wwise_ui_cli_executeLuaScript,
    /// Launch the main window of WwiseXP.
    ///
    /// WAAPI URI: `ak.wwise.ui.cli.launch`.
    ak_wwise_ui_cli_launch,
    /// Executes a command. Some commands can take a list of objects as parameters. See the Wwise global command IDs reference for the available commands.
    ///
    /// WAAPI URI: `ak.wwise.ui.commands.execute`.
    ak_wwise_ui_commands_execute,
    /// Gets the list of commands.
    ///
    /// WAAPI URI: `ak.wwise.ui.commands.getCommands`.
    ak_wwise_ui_commands_getCommands,
    /// Registers an array of add-on commands. Registered commands remain until the Wwise process is terminated. See the Wwise custom commands documentation for more information about registering commands. Also see `ak.wwise.ui.commands.executed`.
    ///
    /// WAAPI URI: `ak.wwise.ui.commands.register`.
    ak_wwise_ui_commands_register,
    /// Unregisters an array of add-on UI commands.
    ///
    /// WAAPI URI: `ak.wwise.ui.commands.unregister`.
    ak_wwise_ui_commands_unregister,
    /// Retrieves the list of files currently selected by the user in the active view. Note that this function is not available in WwiseConsole.
    ///
    /// WAAPI URI: `ak.wwise.ui.getSelectedFiles`.
    ak_wwise_ui_getSelectedFiles,
    /// Retrieves the list of objects currently selected by the user in the active view. Note that this function is not available in WwiseConsole.
    ///
    /// WAAPI URI: `ak.wwise.ui.getSelectedObjects`.
    ak_wwise_ui_getSelectedObjects,
    /// Requests to close a view by its unique id. The view might not yet be closed when returning from this function.
    ///
    /// WAAPI URI: `ak.wwise.ui.layout.closeView`.
    ak_wwise_ui_layout_closeView,
    /// Dock a floating view into a layout.
    ///
    /// WAAPI URI: `ak.wwise.ui.layout.dockView`.
    ak_wwise_ui_layout_dockView,
    /// Retrieves the current layout name.
    ///
    /// WAAPI URI: `ak.wwise.ui.layout.getCurrentLayoutName`.
    ak_wwise_ui_layout_getCurrentLayoutName,
    /// Retrieves the current allocated rectangle of a layout element. An empty rect is returned if the element is not found.
    ///
    /// WAAPI URI: `ak.wwise.ui.layout.getElementRectangle`.
    ak_wwise_ui_layout_getElementRectangle,
    /// Serializes a specific layout into a JSON format.
    ///
    /// WAAPI URI: `ak.wwise.ui.layout.getLayout`.
    ak_wwise_ui_layout_getLayout,
    /// Retrieves a list of the factory layout names.
    ///
    /// WAAPI URI: `ak.wwise.ui.layout.getLayoutNames`.
    ak_wwise_ui_layout_getLayoutNames,
    /// Gets a view, if it exists in the current layout, or creates a new one.
    ///
    /// WAAPI URI: `ak.wwise.ui.layout.getOrCreateView`.
    ak_wwise_ui_layout_getOrCreateView,
    /// Gets the frontend handle of an instantiated view.
    ///
    /// WAAPI URI: `ak.wwise.ui.layout.getViewHandle`.
    ak_wwise_ui_layout_getViewHandle,
    /// Retrieves a list of all view instances of a layout.
    ///
    /// WAAPI URI: `ak.wwise.ui.layout.getViewInstances`.
    ak_wwise_ui_layout_getViewInstances,
    /// Retrieves a list of all view types registered in Wwise.
    ///
    /// WAAPI URI: `ak.wwise.ui.layout.getViewTypes`.
    ak_wwise_ui_layout_getViewTypes,
    /// Moves a splitter by a delta given in pixels.
    ///
    /// WAAPI URI: `ak.wwise.ui.layout.moveSplitter`.
    ak_wwise_ui_layout_moveSplitter,
    /// Unregisters a temporary layout, previously registered with `ak.wwise.ui.layout.setLayout`.
    ///
    /// WAAPI URI: `ak.wwise.ui.layout.removeLayout`.
    ak_wwise_ui_layout_removeLayout,
    /// Reset layouts to their default state.
    ///
    /// WAAPI URI: `ak.wwise.ui.layout.resetLayouts`.
    ak_wwise_ui_layout_resetLayouts,
    /// Registers a new layout from a JSON format.
    ///
    /// WAAPI URI: `ak.wwise.ui.layout.setLayout`.
    ak_wwise_ui_layout_setLayout,
    /// Switches the current layout.
    ///
    /// WAAPI URI: `ak.wwise.ui.layout.switchLayout`.
    ak_wwise_ui_layout_switchLayout,
    /// Undock a view from a layout.
    ///
    /// WAAPI URI: `ak.wwise.ui.layout.undockView`.
    ak_wwise_ui_layout_undockView,
    /// Creates a frontend handle and instantiates its widgets.
    ///
    /// WAAPI URI: `ak.wwise.ui.model.createHandle`.
    ak_wwise_ui_model_createHandle,
    /// Destroys a frontend handle and its widgets.
    ///
    /// WAAPI URI: `ak.wwise.ui.model.destroyHandle`.
    ak_wwise_ui_model_destroyHandle,
    /// Gets the state of widgets.
    ///
    /// WAAPI URI: `ak.wwise.ui.model.getState`.
    ak_wwise_ui_model_getState,
    /// Registers WAFM files and inline widget descriptors.
    ///
    /// WAAPI URI: `ak.wwise.ui.model.registerWafm`.
    ak_wwise_ui_model_registerWafm,
    /// Sets the state of widgets.
    ///
    /// WAAPI URI: `ak.wwise.ui.model.setState`.
    ak_wwise_ui_model_setState,
    /// Closes the current project.
    ///
    /// WAAPI URI: `ak.wwise.ui.project.close`.
    ak_wwise_ui_project_close,
    /// Creates, saves and opens new empty project, specified by path and platform. The project has no factory setting WorkUnit. Please see `ak.wwise.core.project.loaded` for further explanations on how to be notified when the operation has completed.
    ///
    /// WAAPI URI: `ak.wwise.ui.project.create`.
    ak_wwise_ui_project_create,
    /// Opens a project, specified by path. Please see `ak.wwise.core.project.loaded` for further explanations on how to be notified when the operation has completed.
    ///
    /// WAAPI URI: `ak.wwise.ui.project.open`.
    ak_wwise_ui_project_open,
    /// Emits a signal on a widget.
    ///
    /// WAAPI URI: `ak.wwise.ui.signal.emit`.
    ak_wwise_ui_signal_emit,
    /// Closes a window and unrealizes its widgets.
    ///
    /// WAAPI URI: `ak.wwise.ui.window.close`.
    ak_wwise_ui_window_close,
    /// Creates a window in which a frontend handle can be instantiated.
    ///
    /// WAAPI URI: `ak.wwise.ui.window.create`.
    ak_wwise_ui_window_create,
    /// Presents a window and realizes its widgets.
    ///
    /// WAAPI URI: `ak.wwise.ui.window.present`.
    ak_wwise_ui_window_present,
    /// Retrieves the list of functions.
    ///
    /// WAAPI URI: `ak.wwise.waapi.getFunctions`.
    ak_wwise_waapi_getFunctions,
    /// Retrieves the JSON schema of a Waapi Function or Topic URI.
    ///
    /// WAAPI URI: `ak.wwise.waapi.getSchema`.
    ak_wwise_waapi_getSchema,
    /// Retrieves the list of topics to which a client can subscribe.
    ///
    /// WAAPI URI: `ak.wwise.waapi.getTopics`.
    ak_wwise_waapi_getTopics,
}

impl WaapiFunction {
    /// Returns the WAAPI URI string for this endpoint.
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
            Self::ak_soundengine_setGameObjectAuxSendValues => ak::soundengine::setGameObjectAuxSendValues,
            Self::ak_soundengine_setGameObjectOutputBusVolume => ak::soundengine::setGameObjectOutputBusVolume,
            Self::ak_soundengine_setListenerSpatialization => ak::soundengine::setListenerSpatialization,
            Self::ak_soundengine_setListeners => ak::soundengine::setListeners,
            Self::ak_soundengine_setMultiplePositions => ak::soundengine::setMultiplePositions,
            Self::ak_soundengine_setObjectObstructionAndOcclusion => ak::soundengine::setObjectObstructionAndOcclusion,
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
            Self::ak_wwise_core_audio_importTabDelimited => ak::wwise::core::audio::importTabDelimited,
            Self::ak_wwise_core_audio_mute => ak::wwise::core::audio::mute,
            Self::ak_wwise_core_audio_resetMute => ak::wwise::core::audio::resetMute,
            Self::ak_wwise_core_audio_resetSolo => ak::wwise::core::audio::resetSolo,
            Self::ak_wwise_core_audio_setConversionPlugin => ak::wwise::core::audio::setConversionPlugin,
            Self::ak_wwise_core_audio_solo => ak::wwise::core::audio::solo,
            Self::ak_wwise_core_audioSourcePeaks_getMinMaxPeaksInRegion => ak::wwise::core::audioSourcePeaks::getMinMaxPeaksInRegion,
            Self::ak_wwise_core_audioSourcePeaks_getMinMaxPeaksInTrimmedRegion => ak::wwise::core::audioSourcePeaks::getMinMaxPeaksInTrimmedRegion,
            Self::ak_wwise_core_blendContainer_addAssignment => ak::wwise::core::blendContainer::addAssignment,
            Self::ak_wwise_core_blendContainer_addTrack => ak::wwise::core::blendContainer::addTrack,
            Self::ak_wwise_core_blendContainer_getAssignments => ak::wwise::core::blendContainer::getAssignments,
            Self::ak_wwise_core_blendContainer_removeAssignment => ak::wwise::core::blendContainer::removeAssignment,
            Self::ak_wwise_core_executeLuaScript => ak::wwise::core::executeLuaScript,
            Self::ak_wwise_core_gameParameter_setRange => ak::wwise::core::gameParameter::setRange,
            Self::ak_wwise_core_getInfo => ak::wwise::core::getInfo,
            Self::ak_wwise_core_getProjectInfo => ak::wwise::core::getProjectInfo,
            Self::ak_wwise_core_log_addItem => ak::wwise::core::log::addItem,
            Self::ak_wwise_core_log_clear => ak::wwise::core::log::clear,
            Self::ak_wwise_core_log_get => ak::wwise::core::log::get,
            Self::ak_wwise_core_mediaPool_get => ak::wwise::core::mediaPool::get,
            Self::ak_wwise_core_mediaPool_getFields => ak::wwise::core::mediaPool::getFields,
            Self::ak_wwise_core_object_copy => ak::wwise::core::object::copy,
            Self::ak_wwise_core_object_create => ak::wwise::core::object::create,
            Self::ak_wwise_core_object_delete => ak::wwise::core::object::delete,
            Self::ak_wwise_core_object_diff => ak::wwise::core::object::diff,
            Self::ak_wwise_core_object_get => ak::wwise::core::object::get,
            Self::ak_wwise_core_object_getAttenuationCurve => ak::wwise::core::object::getAttenuationCurve,
            Self::ak_wwise_core_object_getPropertyAndReferenceNames => ak::wwise::core::object::getPropertyAndReferenceNames,
            Self::ak_wwise_core_object_getPropertyInfo => ak::wwise::core::object::getPropertyInfo,
            Self::ak_wwise_core_object_getTypes => ak::wwise::core::object::getTypes,
            Self::ak_wwise_core_object_isLinked => ak::wwise::core::object::isLinked,
            Self::ak_wwise_core_object_isPropertyEnabled => ak::wwise::core::object::isPropertyEnabled,
            Self::ak_wwise_core_object_move => ak::wwise::core::object::move_,
            Self::ak_wwise_core_object_pasteProperties => ak::wwise::core::object::pasteProperties,
            Self::ak_wwise_core_object_set => ak::wwise::core::object::set,
            Self::ak_wwise_core_object_setAttenuationCurve => ak::wwise::core::object::setAttenuationCurve,
            Self::ak_wwise_core_object_setLinked => ak::wwise::core::object::setLinked,
            Self::ak_wwise_core_object_setName => ak::wwise::core::object::setName,
            Self::ak_wwise_core_object_setNotes => ak::wwise::core::object::setNotes,
            Self::ak_wwise_core_object_setProperty => ak::wwise::core::object::setProperty,
            Self::ak_wwise_core_object_setRandomizer => ak::wwise::core::object::setRandomizer,
            Self::ak_wwise_core_object_setReference => ak::wwise::core::object::setReference,
            Self::ak_wwise_core_object_setStateGroups => ak::wwise::core::object::setStateGroups,
            Self::ak_wwise_core_object_setStateProperties => ak::wwise::core::object::setStateProperties,
            Self::ak_wwise_core_ping => ak::wwise::core::ping,
            Self::ak_wwise_core_profiler_captureLog_save => ak::wwise::core::profiler::captureLog::save,
            Self::ak_wwise_core_profiler_enableProfilerData => ak::wwise::core::profiler::enableProfilerData,
            Self::ak_wwise_core_profiler_getAudioObjects => ak::wwise::core::profiler::getAudioObjects,
            Self::ak_wwise_core_profiler_getBusses => ak::wwise::core::profiler::getBusses,
            Self::ak_wwise_core_profiler_getCpuUsage => ak::wwise::core::profiler::getCpuUsage,
            Self::ak_wwise_core_profiler_getCursorTime => ak::wwise::core::profiler::getCursorTime,
            Self::ak_wwise_core_profiler_getGameObjects => ak::wwise::core::profiler::getGameObjects,
            Self::ak_wwise_core_profiler_getLoadedMedia => ak::wwise::core::profiler::getLoadedMedia,
            Self::ak_wwise_core_profiler_getMeters => ak::wwise::core::profiler::getMeters,
            Self::ak_wwise_core_profiler_getPerformanceMonitor => ak::wwise::core::profiler::getPerformanceMonitor,
            Self::ak_wwise_core_profiler_getRTPCs => ak::wwise::core::profiler::getRTPCs,
            Self::ak_wwise_core_profiler_getStreamedMedia => ak::wwise::core::profiler::getStreamedMedia,
            Self::ak_wwise_core_profiler_getVoiceContributions => ak::wwise::core::profiler::getVoiceContributions,
            Self::ak_wwise_core_profiler_getVoices => ak::wwise::core::profiler::getVoices,
            Self::ak_wwise_core_profiler_moveCursor => ak::wwise::core::profiler::moveCursor,
            Self::ak_wwise_core_profiler_registerMeter => ak::wwise::core::profiler::registerMeter,
            Self::ak_wwise_core_profiler_saveCapture => ak::wwise::core::profiler::saveCapture,
            Self::ak_wwise_core_profiler_setCursorTime => ak::wwise::core::profiler::setCursorTime,
            Self::ak_wwise_core_profiler_startCapture => ak::wwise::core::profiler::startCapture,
            Self::ak_wwise_core_profiler_stopCapture => ak::wwise::core::profiler::stopCapture,
            Self::ak_wwise_core_profiler_unregisterMeter => ak::wwise::core::profiler::unregisterMeter,
            Self::ak_wwise_core_project_save => ak::wwise::core::project::save,
            Self::ak_wwise_core_remote_connect => ak::wwise::core::remote::connect,
            Self::ak_wwise_core_remote_disconnect => ak::wwise::core::remote::disconnect,
            Self::ak_wwise_core_remote_getAvailableConsoles => ak::wwise::core::remote::getAvailableConsoles,
            Self::ak_wwise_core_remote_getConnectionStatus => ak::wwise::core::remote::getConnectionStatus,
            Self::ak_wwise_core_sound_setActiveSource => ak::wwise::core::sound::setActiveSource,
            Self::ak_wwise_core_soundbank_convertExternalSources => ak::wwise::core::soundbank::convertExternalSources,
            Self::ak_wwise_core_soundbank_generate => ak::wwise::core::soundbank::generate,
            Self::ak_wwise_core_soundbank_getInclusions => ak::wwise::core::soundbank::getInclusions,
            Self::ak_wwise_core_soundbank_processDefinitionFiles => ak::wwise::core::soundbank::processDefinitionFiles,
            Self::ak_wwise_core_soundbank_setInclusions => ak::wwise::core::soundbank::setInclusions,
            Self::ak_wwise_core_sourceControl_add => ak::wwise::core::sourceControl::add,
            Self::ak_wwise_core_sourceControl_checkOut => ak::wwise::core::sourceControl::checkOut,
            Self::ak_wwise_core_sourceControl_commit => ak::wwise::core::sourceControl::commit,
            Self::ak_wwise_core_sourceControl_delete => ak::wwise::core::sourceControl::delete,
            Self::ak_wwise_core_sourceControl_getSourceFiles => ak::wwise::core::sourceControl::getSourceFiles,
            Self::ak_wwise_core_sourceControl_getStatus => ak::wwise::core::sourceControl::getStatus,
            Self::ak_wwise_core_sourceControl_move => ak::wwise::core::sourceControl::move_,
            Self::ak_wwise_core_sourceControl_revert => ak::wwise::core::sourceControl::revert,
            Self::ak_wwise_core_sourceControl_setProvider => ak::wwise::core::sourceControl::setProvider,
            Self::ak_wwise_core_switchContainer_addAssignment => ak::wwise::core::switchContainer::addAssignment,
            Self::ak_wwise_core_switchContainer_getAssignments => ak::wwise::core::switchContainer::getAssignments,
            Self::ak_wwise_core_switchContainer_removeAssignment => ak::wwise::core::switchContainer::removeAssignment,
            Self::ak_wwise_core_transport_create => ak::wwise::core::transport::create,
            Self::ak_wwise_core_transport_destroy => ak::wwise::core::transport::destroy,
            Self::ak_wwise_core_transport_executeAction => ak::wwise::core::transport::executeAction,
            Self::ak_wwise_core_transport_getList => ak::wwise::core::transport::getList,
            Self::ak_wwise_core_transport_getState => ak::wwise::core::transport::getState,
            Self::ak_wwise_core_transport_prepare => ak::wwise::core::transport::prepare,
            Self::ak_wwise_core_transport_useOriginals => ak::wwise::core::transport::useOriginals,
            Self::ak_wwise_core_undo_beginGroup => ak::wwise::core::undo::beginGroup,
            Self::ak_wwise_core_undo_cancelGroup => ak::wwise::core::undo::cancelGroup,
            Self::ak_wwise_core_undo_endGroup => ak::wwise::core::undo::endGroup,
            Self::ak_wwise_core_undo_redo => ak::wwise::core::undo::redo,
            Self::ak_wwise_core_undo_undo => ak::wwise::core::undo::undo,
            Self::ak_wwise_core_workUnit_load => ak::wwise::core::workUnit::load,
            Self::ak_wwise_core_workUnit_unload => ak::wwise::core::workUnit::unload,
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
            Self::ak_wwise_ui_getSelectedFiles => ak::wwise::ui::getSelectedFiles,
            Self::ak_wwise_ui_getSelectedObjects => ak::wwise::ui::getSelectedObjects,
            Self::ak_wwise_ui_layout_closeView => ak::wwise::ui::layout::closeView,
            Self::ak_wwise_ui_layout_dockView => ak::wwise::ui::layout::dockView,
            Self::ak_wwise_ui_layout_getCurrentLayoutName => ak::wwise::ui::layout::getCurrentLayoutName,
            Self::ak_wwise_ui_layout_getElementRectangle => ak::wwise::ui::layout::getElementRectangle,
            Self::ak_wwise_ui_layout_getLayout => ak::wwise::ui::layout::getLayout,
            Self::ak_wwise_ui_layout_getLayoutNames => ak::wwise::ui::layout::getLayoutNames,
            Self::ak_wwise_ui_layout_getOrCreateView => ak::wwise::ui::layout::getOrCreateView,
            Self::ak_wwise_ui_layout_getViewHandle => ak::wwise::ui::layout::getViewHandle,
            Self::ak_wwise_ui_layout_getViewInstances => ak::wwise::ui::layout::getViewInstances,
            Self::ak_wwise_ui_layout_getViewTypes => ak::wwise::ui::layout::getViewTypes,
            Self::ak_wwise_ui_layout_moveSplitter => ak::wwise::ui::layout::moveSplitter,
            Self::ak_wwise_ui_layout_removeLayout => ak::wwise::ui::layout::removeLayout,
            Self::ak_wwise_ui_layout_resetLayouts => ak::wwise::ui::layout::resetLayouts,
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
