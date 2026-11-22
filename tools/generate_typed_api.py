#!/usr/bin/env python3
"""Generate typed Rust WAAPI bindings from the Wwise JSON schemas.

Reads the endpoint schemas and shared definition files from
tools/waapi_schemas/<version>/ and writes src/api/generated/.

Requires no running Wwise instance. Regenerate with:

    python tools/generate_typed_api.py
"""

import json
import re
import shutil
import xml.etree.ElementTree as ET
from pathlib import Path

SCHEMA_VERSION = "2024.1.7"
SCHEMA_DIR = Path(__file__).resolve().parent / "waapi_schemas" / SCHEMA_VERSION
OUT_DIR = Path(__file__).resolve().parent.parent / "src" / "api" / "generated"

PILOT_ENDPOINTS = [
    # Core Info & Project
    "ak.wwise.core.getInfo",
    "ak.wwise.core.getProjectInfo",
    "ak.wwise.core.project.save",
    # Core Object CRUD & Helpers
    "ak.wwise.core.object.get",
    "ak.wwise.core.object.create",
    "ak.wwise.core.object.delete",
    "ak.wwise.core.object.set",
    "ak.wwise.core.object.move",
    "ak.wwise.core.object.copy",
    "ak.wwise.core.object.setName",
    "ak.wwise.core.object.setNotes",
    "ak.wwise.core.object.setProperty",
    "ak.wwise.core.object.setReference",
    "ak.wwise.core.object.setRandomizer",
    "ak.wwise.core.object.getAttenuationCurve",
    "ak.wwise.core.object.setAttenuationCurve",
    "ak.wwise.core.object.diff",
    "ak.wwise.core.object.pasteProperties",
    # Switch Container
    "ak.wwise.core.switchContainer.addAssignment",
    "ak.wwise.core.switchContainer.getAssignments",
    "ak.wwise.core.switchContainer.removeAssignment",
    # Audio
    "ak.wwise.core.audio.import",
    # Transport
    "ak.wwise.core.transport.create",
    "ak.wwise.core.transport.destroy",
    "ak.wwise.core.transport.executeAction",
    "ak.wwise.core.transport.getState",
    "ak.wwise.core.transport.getList",
    "ak.wwise.core.transport.prepare",
    "ak.wwise.core.transport.useOriginals",
    # Undo
    "ak.wwise.core.undo.beginGroup",
    "ak.wwise.core.undo.endGroup",
    "ak.wwise.core.undo.cancelGroup",
    "ak.wwise.core.undo.undo",
    "ak.wwise.core.undo.redo",
    # UI
    "ak.wwise.ui.getSelectedObjects",
    "ak.wwise.ui.bringToForeground",
    "ak.wwise.ui.commands.execute",
    "ak.wwise.ui.commands.getCommands",
    "ak.wwise.ui.project.open",
    "ak.wwise.ui.project.close",
    # SoundEngine
    "ak.soundengine.postEvent",
    "ak.soundengine.setRTPCValue",
    "ak.soundengine.registerGameObj",
    "ak.soundengine.unregisterGameObj",
    "ak.soundengine.setPosition",
    "ak.soundengine.setDefaultListeners",
    "ak.soundengine.setSwitch",
    "ak.soundengine.setState",
    "ak.soundengine.getSwitch",
    "ak.soundengine.getState",
    "ak.soundengine.loadBank",
    "ak.soundengine.unloadBank",
    "ak.soundengine.stopAll",
    "ak.soundengine.stopPlayingID",
    "ak.soundengine.seekOnEvent",
    "ak.soundengine.setListeners",
    "ak.soundengine.setListenerSpatialization",
    "ak.soundengine.setMultiplePositions",
    "ak.soundengine.setGameObjectAuxSendValues",
    "ak.soundengine.setGameObjectOutputBusVolume",
    "ak.soundengine.setObjectObstructionAndOcclusion",
    "ak.soundengine.setScalingFactor",
    "ak.soundengine.executeActionOnEvent",
    "ak.soundengine.postTrigger",
    "ak.soundengine.postMsgMonitor",
    "ak.soundengine.resetRTPCValue",
    # Object introspection
    "ak.wwise.core.object.getPropertyAndReferenceNames",
    "ak.wwise.core.object.getPropertyInfo",
    "ak.wwise.core.object.getTypes",
    "ak.wwise.core.object.isLinked",
    "ak.wwise.core.object.isPropertyEnabled",
    "ak.wwise.core.object.setLinked",
    "ak.wwise.core.object.setStateGroups",
    "ak.wwise.core.object.setStateProperties",
    # Singles
    "ak.wwise.core.sound.setActiveSource",
    "ak.wwise.core.gameParameter.setRange",
    "ak.wwise.core.ping",
    # Blend Container
    "ak.wwise.core.blendContainer.addAssignment",
    "ak.wwise.core.blendContainer.addTrack",
    "ak.wwise.core.blendContainer.getAssignments",
    "ak.wwise.core.blendContainer.removeAssignment",
    # SoundBank
    "ak.wwise.core.soundbank.convertExternalSources",
    "ak.wwise.core.soundbank.generate",
    "ak.wwise.core.soundbank.getInclusions",
    "ak.wwise.core.soundbank.processDefinitionFiles",
    "ak.wwise.core.soundbank.setInclusions",
    # Audio
    "ak.wwise.core.audio.convert",
    "ak.wwise.core.audio.importTabDelimited",
    "ak.wwise.core.audio.mute",
    "ak.wwise.core.audio.resetMute",
    "ak.wwise.core.audio.resetSolo",
    "ak.wwise.core.audio.setConversionPlugin",
    "ak.wwise.core.audio.solo",
]

# Subscription topics for which typed payload structs and `subscribe_*` chained
# methods are generated. Each topic schema has a `publishSchema` (the payload)
# and an optional `optionsSchema` (subscription options).
TOPIC_ENDPOINTS = [
    # Object lifecycle and property topics
    "ak.wwise.core.object.created",
    "ak.wwise.core.object.preDeleted",
    "ak.wwise.core.object.postDeleted",
    "ak.wwise.core.object.childAdded",
    "ak.wwise.core.object.childRemoved",
    "ak.wwise.core.object.nameChanged",
    "ak.wwise.core.object.notesChanged",
    "ak.wwise.core.object.propertyChanged",
    "ak.wwise.core.object.referenceChanged",
    "ak.wwise.core.object.curveChanged",
    "ak.wwise.core.object.attenuationCurveChanged",
    "ak.wwise.core.object.attenuationCurveLinkChanged",
    # Switch Container
    "ak.wwise.core.switchContainer.assignmentAdded",
    "ak.wwise.core.switchContainer.assignmentRemoved",
    # Transport
    "ak.wwise.core.transport.stateChanged",
    # SoundBank
    "ak.wwise.core.soundbank.generated",
    "ak.wwise.core.soundbank.generationDone",
    # Project
    "ak.wwise.core.project.preClosed",
    "ak.wwise.core.project.loaded",
    "ak.wwise.core.project.postClosed",
    "ak.wwise.core.project.saved",
    # Audio
    "ak.wwise.core.audio.imported",
    # Log
    "ak.wwise.core.log.itemAdded",
    # Profiler
    "ak.wwise.core.profiler.gameObjectRegistered",
    "ak.wwise.core.profiler.gameObjectUnregistered",
    "ak.wwise.core.profiler.gameObjectReset",
    "ak.wwise.core.profiler.stateChanged",
    "ak.wwise.core.profiler.switchChanged",
    "ak.wwise.core.profiler.captureLog.itemAdded",
    # Debug
    "ak.wwise.debug.assertFailed",
    # UI
    "ak.wwise.ui.selectionChanged",
    "ak.wwise.ui.commands.executed",
    "ak.wwise.ui.signal.click",
    "ak.wwise.ui.signal.toggle",
]

# Constructor parameter order where the schema's `required` list differs from
# the established API.
CTOR_ORDER = {
    "ak.wwise.core.object.create": ["parent", "type", "name"],
    "ak.wwise.core.object.setRandomizer": ["object", "property"],
}

# Rust field names for result properties that would otherwise collide with
# keywords or read poorly.
RESULT_FIELD_NAMES = {
    ("ak.wwise.core.object.get", "return"): "objects",
    ("ak.wwise.core.switchContainer.getAssignments", "return"): "assignments",
    ("ak.wwise.core.blendContainer.getAssignments", "return"): "assignments",
    ("ak.wwise.core.object.getTypes", "return"): "types",
    ("ak.wwise.core.object.getPropertyAndReferenceNames", "return"): "names",
    ("ak.wwise.core.object.isPropertyEnabled", "return"): "enabled",
    ("ak.soundengine.postEvent", "return"): "playing_id",
    ("ak.soundengine.getSwitch", "return"): "switch",
    ("ak.soundengine.getState", "return"): "state",
}

# Endpoints whose resultSchema is `$ref objectReturn` but whose actual WAAPI
# response wraps the object under a top-level `"return"` key (e.g.
# `{"return": {"id": ..., "name": ...}}`), unlike object.copy which returns a
# bare objectReturn.
RESULT_WRAPPED_OBJECT_RETURN = {
    "ak.soundengine.getSwitch",
    "ak.soundengine.getState",
}

# Response fields that Wwise always returns even when the JSON schema omits `required`.
RESPONSE_REQUIRED_OVERRIDES = {
    "ak.wwise.core.transport.create": ["transport"],
    "ak.wwise.core.transport.getList": ["transport", "object"],
    "ak.wwise.core.object.getAttenuationCurve": ["curveType", "use", "points"],
    "ak.wwise.ui.project.close": ["hadProjectOpen"],
}
# Global wire-name -> Rust field name overrides.
FIELD_NAME_OVERRIDES = {
    "type": "object_type",
}

# Property type overrides for properties whose schema is a plain string enum
# that already has a hand-written Rust type, and for inline nested objects
# that map to generated structs.
PROP_TYPE_OVERRIDES = {
    ("ak.wwise.core.object.create", "args", "children"): "Vec<ChildObject>",
    ("ak.wwise.core.object.create", "onNameConflict"): "OnNameConflict",
    ("ak.wwise.core.object.set", "onNameConflict"): "OnNameConflict",
    ("ak.wwise.core.object.set", "listMode"): "ListMode",
    ("ak.wwise.core.object.move", "onNameConflict"): "OnNameConflict",
    ("ak.wwise.core.object.copy", "onNameConflict"): "OnNameConflict",
    ("ak.wwise.core.object.setReference", "reference"): "Property",
    ("ak.wwise.core.object.getAttenuationCurve", "curveType"): "AttenuationCurveType",
    ("ak.wwise.core.object.getAttenuationCurve", "use"): "AttenuationCurveUse",
    ("ak.wwise.core.object.setAttenuationCurve", "curveType"): "AttenuationCurveType",
    ("ak.wwise.core.object.setAttenuationCurve", "use"): "AttenuationCurveUse",
    ("ak.wwise.core.object.pasteProperties", "pasteMode"): "PasteMode",
    ("ak.wwise.core.object.pasteProperties", "inclusion"): "Vec<Property>",
    ("ak.wwise.core.object.pasteProperties", "exclusion"): "Vec<Property>",
    ("ak.wwise.core.switchContainer.getAssignments", "response", "return"): "Vec<SwitchContainerAssignment>",
    ("ak.wwise.core.transport.executeAction", "action"): "TransportAction",
    ("ak.wwise.core.transport.getState", "state"): "TransportState",
    ("ak.wwise.core.transport.getList", "list"): "Vec<TransportObject>",
    ("ak.wwise.core.getInfo", "version"): "VersionInfo",
    ("ak.wwise.core.getInfo", "directories"): "DirectoriesInfo",
    ("ak.wwise.core.getProjectInfo", "platforms"): "Vec<ProjectPlatformInfo>",
    ("ak.wwise.core.getProjectInfo", "languages"): "Vec<ProjectLanguageInfo>",
    ("ak.wwise.core.getProjectInfo", "directories"): "ProjectDirectoriesInfo",
    ("ak.wwise.core.audio.import", "imports"): "Vec<ImportItem>",
    ("ak.wwise.core.audio.import", "default"): "ImportDefaults",
    ("ak.wwise.ui.project.open", "onMigrationRequired"): "ProjectMigrationAction",
    ("ak.wwise.ui.project.open", "onUpgrade"): "ProjectMigrationAction",
    ("ak.soundengine.setPosition", "args", "position"): "Position",
    ("ak.soundengine.executeActionOnEvent", "args", "actionType"): "ActionOnEventType",
    ("ak.soundengine.executeActionOnEvent", "args", "fadeCurve"): "AkCurveInterpolation",
    ("ak.soundengine.stopPlayingID", "args", "fadeCurve"): "AkCurveInterpolation",
    ("ak.soundengine.setMultiplePositions", "args", "positions"): "Vec<MultiPosition>",
    ("ak.soundengine.setMultiplePositions", "args", "multiPositionType"): "MultiPositionType",
    ("ak.soundengine.setGameObjectAuxSendValues", "args", "auxSendValues"): "Vec<AuxSendValue>",
    ("ak.wwise.core.object.getPropertyInfo", "response", "supports"): "PropertyInfoSupports",
    ("ak.wwise.core.object.getPropertyInfo", "response", "default"): "serde_json::Value",
    ("ak.wwise.core.object.getPropertyInfo", "response", "type"): "Option<String>",
    ("ak.wwise.core.object.isLinked", "args", "property"): "Property",
    ("ak.wwise.core.object.getTypes", "response", "return"): "Vec<WwiseObjectTypeInfo>",
    ("ak.wwise.core.gameParameter.setRange", "args", "onCurveUpdate"): "OnCurveUpdate",
    ("ak.wwise.core.blendContainer.addAssignment", "args", "edges"): "Vec<BlendEdge>",
    ("ak.wwise.core.blendContainer.getAssignments", "response", "return"): "Vec<BlendTrackAssignment>",
    ("ak.wwise.core.soundbank.generate", "args", "soundbanks"): "Vec<SoundBankInfo>",
    ("ak.wwise.core.soundbank.generate", "response", "logs"): "Vec<LogCentralItem>",
    ("ak.wwise.core.soundbank.getInclusions", "response", "inclusions"): "Vec<SoundBankInclusion>",
    ("ak.wwise.core.soundbank.setInclusions", "args", "operation"): "SoundBankInclusionOperation",
    ("ak.wwise.core.soundbank.setInclusions", "args", "inclusions"): "Vec<SoundBankInclusion>",
    ("ak.wwise.core.soundbank.convertExternalSources", "args", "sources"): "Vec<ExternalSourceInfo>",
    ("ak.wwise.core.audio.convert", "response", "errors"): "Vec<AudioConvertError>",
    ("ak.wwise.core.transport.stateChanged", "response", "state"): "TransportState",
    ("ak.wwise.core.object.attenuationCurveChanged", "response", "curveType"): "AttenuationCurveType",
    ("ak.wwise.core.object.attenuationCurveLinkChanged", "response", "curveType"): "AttenuationCurveType",
    ("ak.wwise.core.object.propertyChanged", "response", "old"): "serde_json::Value",
    ("ak.wwise.core.object.propertyChanged", "response", "new"): "serde_json::Value",
}

# Well-known $ref targets mapped to Rust types. Hand-written types live in
# crate::api::primitives; the rest are generated in generated/primitives.rs.
REF_TYPES = {
    "common_definitions.json#/definitions/guid": "String",
    "common_definitions.json#/definitions/uint32": "u32",
    "common_definitions.json#/definitions/int32": "i32",
    "common_definitions.json#/definitions/uint64": "u64",
    "common_definitions.json#/definitions/int64": "i64",
    "common_definitions.json#/definitions/uint16": "u16",
    "common_definitions.json#/definitions/int8": "i8",
    "common_definitions.json#/definitions/id64bits": "u64",
    "common_definitions.json#/definitions/propertyValue": "PropertyValue",
    "waapi_definitions.json#/definitions/objectArg": "ObjectRef",
    "waapi_definitions.json#/definitions/platformArg": "PlatformRef",
    "waapi_definitions.json#/definitions/languageArg": "LanguageRef",
    "waapi_definitions.json#/definitions/propertyName": "Property",
    "waapi_definitions.json#/definitions/pointsDefinition": "Vec<CurvePoint2D>",
    "waapi_definitions.json#/definitions/curveShape": "CurveShape",
    "waapi_definitions.json#/definitions/soundEngineObjectArg": "SoundEngineObjectRef",
    "waapi_definitions.json#/definitions/gameObject": "GameObjectRef",
    "waapi_definitions.json#/definitions/objectReturn": "WwiseObject",
    "waapi_definitions.json#/definitions/importOperationType": "ImportOperation",
    "waapi_definitions.json#/definitions/logCentralSeverity": "LogSeverity",
    "waapi_definitions.json#/definitions/uniqueName": "String",
    "waapi_definitions.json#/definitions/uniqueQualifiedName": "String",
    "waapi_definitions.json#/definitions/objectPathArg": "String",
    "waapi_definitions.json#/definitions/shortId": "u32",
    "waapi_definitions.json#/definitions/statePropertiesArg": "Vec<String>",
    "waapi_definitions.json#/definitions/returnExpression": "Vec<ObjectReturnField>",
    "waapi_definitions.json#/definitions/blendTrackEdge": "BlendEdge",
    "waapi_definitions.json#/definitions/logCentralItem": "LogCentralItem",
    "waapi_definitions.json#/definitions/logCentralChannel": "LogChannel",
}

OBJECT_RETURN_OPTIONS_REF = "waapi_definitions.json#/definitions/objectReturnOptions"
OBJECT_RETURN_REF = "waapi_definitions.json#/definitions/objectReturn"

RUST_KEYWORDS = {
    "as", "break", "const", "continue", "crate", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod",
    "move", "mut", "pub", "ref", "return", "self", "static", "struct",
    "super", "trait", "true", "type", "unsafe", "use", "where", "while",
    "async", "await", "dyn", "box",
}


# ─────────────────────────────────────────────────────────────────────────────
# Naming helpers
# ─────────────────────────────────────────────────────────────────────────────

def _words(name):
    out = []
    for part in re.split(r"[^a-zA-Z0-9]+", name):
        if not part:
            continue
        out.extend(re.findall(r"[A-Z]+(?![a-z])|[A-Z][a-z0-9]*|[a-z0-9]+", part))
    return out


def pascal_case(name):
    return "".join(w[:1].upper() + w[1:].lower() if len(w) > 1 else w.upper() for w in _words(name))


def snake_case(name):
    ident = "_".join(w.lower() for w in _words(name))
    if not ident:
        ident = "value"
    if ident[0].isdigit():
        ident = "n_" + ident
    if ident in RUST_KEYWORDS:
        ident += "_"
    return ident


def clean_doc(text):
    if not text:
        return ""
    text = re.sub(r"\\ref \S+", "", text)
    text = re.sub(r"\s+", " ", text).strip()
    return text


def unique_names(values, name_fn, suffix):
    """Map wire names to unique Rust identifiers.

    The schemas list both legacy colon accessors (``workunit:type``) and their
    modern equivalents (``workunitType``); these collapse to the same Rust
    name, so the legacy colon form gets ``suffix`` appended.
    """
    names = {}
    taken = {}
    for v in values:
        n = name_fn(v)
        if n not in taken:
            names[v] = n
            taken[n] = v
            continue
        other = taken[n]
        if ":" in v:
            bumped, bumper = v, n
        else:
            bumped, bumper = other, n
        candidate = bumper + suffix
        i = 2
        while candidate in taken:
            candidate = f"{bumper}{suffix}{i}"
            i += 1
        names[bumped] = candidate
        taken[candidate] = bumped
        if bumped is other:
            names[v] = n
    return names


# ─────────────────────────────────────────────────────────────────────────────
# Schema loading and $ref resolution
# ─────────────────────────────────────────────────────────────────────────────

class Schemas:
    def __init__(self, root):
        self.root = root
        self._defs = {}
        self._endpoints = {}

    def load_definitions(self, fname):
        if fname not in self._defs:
            self._defs[fname] = json.loads((self.root / "Definitions" / fname).read_text())
        return self._defs[fname]

    def load_endpoint(self, uri):
        if uri not in self._endpoints:
            self._endpoints[uri] = json.loads((self.root / f"{uri}.json").read_text())
        return self._endpoints[uri]

    @staticmethod
    def _walk(doc, pointer):
        node = doc
        for part in pointer.strip("/").split("/"):
            if part == "":
                continue
            part = part.replace("~1", "/").replace("~0", "~")
            if not isinstance(node, dict) or part not in node:
                return None
            node = node[part]
        return node

    def resolve(self, ref, doc, local_root=None):
        """Resolve a $ref. Returns the target schema dict, or None if dangling."""
        fname, _, pointer = ref.partition("#")
        if fname:
            if fname.endswith("definitions.json"):
                target = self.load_definitions(fname)
            else:
                target = json.loads((self.root / fname).read_text())
            return self._walk(target, pointer)
        node = self._walk(doc, pointer)
        if node is None and local_root is not None:
            # Some Wwise schemas use pointers relative to the args/result
            # schema instead of the document root.
            node = self._walk(local_root, pointer)
        return node


class Ctx:
    """Per-endpoint generation context."""

    def __init__(self, schemas, uri):
        self.schemas = schemas
        self.uri = uri
        self.doc = schemas.load_endpoint(uri)
        self.local_root = None


# ─────────────────────────────────────────────────────────────────────────────
# Type mapping
# ─────────────────────────────────────────────────────────────────────────────

def rust_type(sch, ctx, self_name=None):
    if not isinstance(sch, dict):
        return "serde_json::Value"
    if "$ref" in sch:
        ref = sch["$ref"]
        if ref in REF_TYPES:
            return REF_TYPES[ref]
        if ref == "#" and self_name:
            return self_name
        resolved = ctx.schemas.resolve(ref, ctx.doc, ctx.local_root)
        if resolved is None:
            return "serde_json::Value"
        return rust_type(resolved, ctx, self_name)
    t = sch.get("type")
    if t == "string":
        return "String"
    if t == "integer":
        return "i64"
    if t == "number":
        return "f64"
    if t == "boolean":
        return "bool"
    if t == "array":
        return f"Vec<{rust_type(sch.get('items', {}), ctx, self_name)}>"
    if t == "object":
        props = sch.get("properties", {})
        if "id" in props and "name" in props:
            return "WwiseObjectRef"
        return "serde_json::Value"
    branches = sch.get("anyOf") or sch.get("oneOf")
    if branches:
        tys = {rust_type(b, ctx) for b in branches}
        if len(tys) == 1:
            return tys.pop()
    return "serde_json::Value"


def stringy_items(sch, ctx):
    """True if an array schema's items always resolve to plain strings."""
    items = sch.get("items", {})
    if not isinstance(items, dict):
        return False
    return rust_type(items, ctx) == "String"


# ─────────────────────────────────────────────────────────────────────────────
# Rust emission helpers
# ─────────────────────────────────────────────────────────────────────────────

def doc_lines(text, prefix=""):
    text = clean_doc(text)
    if not text:
        return []
    return [f"{prefix}/// {text}"]


class Field:
    def __init__(self, wire, rust_name, ty, optional, doc=""):
        self.wire = wire
        self.rust_name = rust_name
        self.ty = ty
        self.optional = optional
        self.doc = doc

    def serde_attrs(self, kind):
        attrs = []
        rename = self.wire != self.rust_name
        pieces = []
        if rename:
            pieces.append(f'rename = "{self.wire}"')
        if self.optional:
            if kind in ("args", "options"):
                pieces.append('skip_serializing_if = "Option::is_none"')
            else:
                pieces.append("default")
                if self.ty.startswith("Option<"):
                    pieces.append('skip_serializing_if = "Option::is_none"')
        if pieces:
            attrs.append(f"#[serde({', '.join(pieces)})]")
        return attrs

    def declaration(self, kind):
        if self.optional and kind == "response" and self.ty.startswith("Vec<"):
            ty = self.ty  # relies on #[serde(default)]
        elif self.optional and not self.ty.startswith("Option<"):
            ty = f"Option<{self.ty}>"
        else:
            ty = self.ty
        lines = doc_lines(self.doc, "    ")
        lines += [f"    {a}" for a in self.serde_attrs(kind)]
        lines.append(f"    pub {self.rust_name}: {ty},")
        return lines


def field_name(uri, wire, kind):
    if kind == "response" and (uri, wire) in RESULT_FIELD_NAMES:
        return RESULT_FIELD_NAMES[(uri, wire)]
    if wire in FIELD_NAME_OVERRIDES:
        return FIELD_NAME_OVERRIDES[wire]
    return snake_case(wire)


def schema_required(schema):
    req = schema.get("required")
    if req:
        return list(req)
    branches = schema.get("anyOf") or schema.get("oneOf") or []
    branch_reqs = [b.get("required") for b in branches if isinstance(b, dict) and b.get("required")]
    if branch_reqs:
        common = set(branch_reqs[0])
        for r in branch_reqs[1:]:
            common &= set(r)
        return [k for k in branch_reqs[0] if k in common]
    return []


def collect_fields(ctx, schema, kind, struct_name):
    fields = []
    req_set = set(schema_required(schema))
    if kind == "response" and ctx.uri in RESPONSE_REQUIRED_OVERRIDES:
        req_set |= set(RESPONSE_REQUIRED_OVERRIDES[ctx.uri])
    for wire, sub in (schema.get("properties") or {}).items():
        ty = PROP_TYPE_OVERRIDES.get((ctx.uri, kind, wire)) or PROP_TYPE_OVERRIDES.get((ctx.uri, wire))
        if ty is None:
            if wire == "objectType" or (wire == "type" and "\\ref wobjects_index" in sub.get("description", "")):
                ty = "ObjectType"
            else:
                ty = rust_type(sub, ctx, self_name=struct_name)
        optional = wire not in req_set
        fields.append(Field(wire, field_name(ctx.uri, wire, kind), ty, optional, sub.get("description", "")))
    return fields


def emit_struct(kind, struct_name, doc, fields, ctx, ctor_order=None, required_order=None, extra_flatten=False):
    """Emit an Args/Options (Serialize) or Response (Serialize+Deserialize) struct."""
    lines = doc_lines(doc) or [f"/// `{ctx.uri}` {kind}."]
    if kind == "response":
        lines.append("#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]")
    elif all(f.optional for f in fields):
        lines.append("#[derive(Debug, Clone, Default, serde::Serialize)]")
    else:
        lines.append("#[derive(Debug, Clone, serde::Serialize)]")
    lines.append(f"pub struct {struct_name} {{")
    for f in fields:
        lines += f.declaration(kind)
    if extra_flatten:
        lines.append("    #[serde(flatten)]")
        lines.append("    pub extra: std::collections::HashMap<String, serde_json::Value>,")
    lines.append("}")
    lines.append("")

    required = [f for f in fields if not f.optional]
    optional = [f for f in fields if f.optional]
    if kind == "response":
        if extra_flatten:
            lines.append(f"impl {struct_name} {{")
            lines.append("    /// Look up a property value in `extra` (checks `\"@Property\"` first, then `\"Property\"`).")
            lines.append("    pub fn property(&self, prop: Property) -> Option<&serde_json::Value> {")
            lines.append("        self.extra")
            lines.append("            .get(prop.as_at_str())")
            lines.append("            .or_else(|| self.extra.get(prop.as_str()))")
            lines.append("    }")
            lines.append("}")
        return "\n".join(lines)

    lines.append(f"impl {struct_name} {{")
    if required:
        order = ctor_order or required_order or [f.wire for f in required]
        by_wire = {f.wire: f for f in required}
        ctor_fields = [by_wire[w] for w in order]
        params = ", ".join(f"{f.rust_name}: {f.ty}" for f in ctor_fields)
        lines.append(f"    pub fn new({params}) -> Self {{")
        lines.append("        Self {")
        for f in ctor_fields:
            lines.append(f"            {f.rust_name},")
        for f in optional:
            lines.append(f"            {f.rust_name}: None,")
        if extra_flatten:
            lines.append("            extra: std::collections::HashMap::new(),")
        lines.append("        }")
        lines.append("    }")
    elif kind != "response":
        lines.append("    pub fn new() -> Self {")
        lines.append("        Self::default()")
        lines.append("    }")
    if kind != "response":
        for f in optional:
            lines.append("")
            lines.append(f"    pub fn {f.rust_name}(mut self, {f.rust_name}: impl Into<{f.ty}>) -> Self {{")
            lines.append(f"        self.{f.rust_name} = Some({f.rust_name}.into());")
            lines.append("        self")
            lines.append("    }")
        if extra_flatten:
            lines.append("")
            lines.append("    /// Set a `@`-prefixed property, reference, or list field.")
            lines.append("    /// Accepts a [`Property`] variant (e.g. `Property::Volume`) or a bare property name (`\"Volume\"`).")
            lines.append("    pub fn property(mut self, name: impl AsRef<str>, value: impl serde::Serialize) -> Self {")
            lines.append("        let name = name.as_ref().strip_prefix('@').unwrap_or(name.as_ref());")
            lines.append("        self.extra.insert(")
            lines.append("            format!(\"@{name}\"),")
            lines.append("            serde_json::to_value(value).expect(\"value must serialize to JSON\"),")
            lines.append("        );")
            lines.append("        self")
            lines.append("    }")
            lines.append("")
            lines.append("    /// Set an arbitrary `@`-prefixed property, reference, or list field.")
            lines.append("    /// The key must include the `@` prefix, e.g. `\"@Volume\"`.")
            lines.append("    pub fn extra(mut self, key: impl Into<String>, value: impl Into<serde_json::Value>) -> Self {")
            lines.append("        self.extra.insert(key.into(), value.into());")
            lines.append("        self")
            lines.append("    }")
    lines.append("}")
    return "\n".join(lines)


def emit_enum(name, doc, values, derive_both=True):
    """Emit a simple string enum with explicit serde renames."""
    lines = doc_lines(doc)
    derives = "serde::Serialize, serde::Deserialize" if derive_both else "serde::Serialize"
    lines.append(f"#[derive(Debug, Clone, Copy, PartialEq, Eq, {derives})]")
    lines.append(f"pub enum {name} {{")
    for v, variant in unique_names(values, pascal_case, "Legacy").items():
        lines.append(f'    #[serde(rename = "{v}")]')
        lines.append(f"    {variant},")
    lines.append("}")
    return "\n".join(lines)


def emit_int_enum(name, doc, values):
    """Emit an integer enum (serializes as a bare number) with explicit discriminants.

    `values` is a list of ``(VariantName, int_value)`` tuples.
    """
    lines = doc_lines(doc)
    lines.append("#[derive(Debug, Clone, Copy, PartialEq, Eq)]")
    lines.append("#[repr(u32)]")
    lines.append(f"pub enum {name} {{")
    for variant, value in values:
        lines.append(f"    {variant} = {value},")
    lines.append("}")
    lines.append("")
    lines.append(f"impl {name} {{")
    lines.append(f"    pub const fn as_u32(self) -> u32 {{ self as u32 }}")
    lines.append("    pub const fn from_u32(value: u32) -> Option<Self> {")
    lines.append("        match value {")
    for variant, value in values:
        lines.append(f"            {value} => Some({name}::{variant}),")
    lines.append("            _ => None,")
    lines.append("        }")
    lines.append("    }")
    lines.append("}")
    lines.append("")
    lines.append(f"impl serde::Serialize for {name} {{")
    lines.append("    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {")
    lines.append("        serializer.serialize_u32(self.as_u32())")
    lines.append("    }")
    lines.append("}")
    lines.append("")
    lines.append(f"impl<'de> serde::Deserialize<'de> for {name} {{")
    lines.append("    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {")
    lines.append("        let value = u32::deserialize(deserializer)?;")
    lines.append("        Self::from_u32(value).ok_or_else(|| serde::de::Error::custom(format!(")
    lines.append(f"            \"invalid {name} value: {{}}\", value")
    lines.append("        )))")
    lines.append("    }")
    lines.append("}")
    return "\n".join(lines)


# ─────────────────────────────────────────────────────────────────────────────
# object_types.rs and properties.rs (from WObjects.xml)
# ─────────────────────────────────────────────────────────────────────────────

def gen_object_types(schema_dir: Path) -> str:
    wobjects_path = schema_dir / "Definitions" / "WObjects.xml"
    tree = ET.parse(wobjects_path)
    root = tree.getroot()

    entries = {}
    for el in root.findall("WwiseObject"):
        name = el.attrib.get("Name")
        if not name or name in entries:
            continue
        company_id = int(el.attrib.get("CompanyID", "1"))
        plugin_id = int(el.attrib.get("PluginID", "0"))
        class_id = (plugin_id << 16) | (company_id << 4)
        entries[name] = {
            "class_id": class_id,
            "plugin_id": plugin_id,
        }

    sorted_names = sorted(entries)
    variants = [(name, property_variant_name(name), entries[name]) for name in sorted_names]

    lines = [
        "// Auto-generated Wwise object types from WObjects.xml.",
        "// This file is generated by tools/generate_typed_api.py; do not edit by hand.",
        "",
        "/// Typed identifier for a Wwise object type (`\\ref wobjects_index`).",
        "///",
        "/// Derived from `<WwiseObject>` definitions in `WObjects.xml`.",
        "#[allow(non_camel_case_types)]",
        "#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]",
        "pub enum ObjectType {",
    ]
    for wire, variant, meta in variants:
        lines.append(f"    /// `{wire}` (class ID `{meta['class_id']}`).")
        lines.append(f"    {variant},")
    lines.append("}")
    lines.append("")
    lines.append("impl ObjectType {")
    lines.append("    /// Returns the Wwise object type name (e.g. `\"Sound\"`, `\"ActorMixer\"`).")
    lines.append("    pub const fn as_str(self) -> &'static str {")
    lines.append("        match self {")
    for wire, variant, _ in variants:
        lines.append(f'            Self::{variant} => "{wire}",')
    lines.append("        }")
    lines.append("    }")
    lines.append("")
    lines.append("    /// Returns the numeric Wwise class ID for this object type.")
    lines.append("    pub const fn class_id(self) -> u32 {")
    lines.append("        match self {")
    for _, variant, meta in variants:
        lines.append(f"            Self::{variant} => {meta['class_id']},")
    lines.append("        }")
    lines.append("    }")
    lines.append("")
    lines.append("    /// Looks up an [`ObjectType`] by its numeric Wwise class ID.")
    lines.append("    pub const fn from_class_id(class_id: u32) -> Option<Self> {")
    lines.append("        match class_id {")
    for _, variant, meta in variants:
        lines.append(f"            {meta['class_id']} => Some(Self::{variant}),")
    lines.append("            _ => None,")
    lines.append("        }")
    lines.append("    }")
    lines.append("}")
    lines.append("")
    lines.append("impl AsRef<str> for ObjectType {")
    lines.append("    fn as_ref(&self) -> &str {")
    lines.append("        self.as_str()")
    lines.append("    }")
    lines.append("}")
    lines.append("")
    lines.append("impl std::fmt::Display for ObjectType {")
    lines.append("    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {")
    lines.append("        f.write_str(self.as_str())")
    lines.append("    }")
    lines.append("}")
    lines.append("")
    lines.append("impl std::str::FromStr for ObjectType {")
    lines.append("    type Err = String;")
    lines.append("")
    lines.append("    fn from_str(s: &str) -> Result<Self, Self::Err> {")
    lines.append("        match s {")
    for wire, variant, _ in variants:
        lines.append(f'            "{wire}" => Ok(Self::{variant}),')
    lines.append('            _ => Err(format!("unknown Wwise ObjectType: {s}")),')
    lines.append("        }")
    lines.append("    }")
    lines.append("}")
    lines.append("")
    lines.append("impl From<ObjectType> for &'static str {")
    lines.append("    fn from(object_type: ObjectType) -> Self {")
    lines.append("        object_type.as_str()")
    lines.append("    }")
    lines.append("}")
    lines.append("")
    lines.append("impl From<ObjectType> for String {")
    lines.append("    fn from(object_type: ObjectType) -> Self {")
    lines.append("        object_type.as_str().to_string()")
    lines.append("    }")
    lines.append("}")
    lines.append("")
    lines.append("impl serde::Serialize for ObjectType {")
    lines.append("    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {")
    lines.append("        serializer.serialize_str(self.as_str())")
    lines.append("    }")
    lines.append("}")
    lines.append("")
    lines.append("impl<'de> serde::Deserialize<'de> for ObjectType {")
    lines.append("    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {")
    lines.append("        let s = String::deserialize(deserializer)?;")
    lines.append("        s.parse().map_err(serde::de::Error::custom)")
    lines.append("    }")
    lines.append("}")
    return "\n".join(lines) + "\n"


def property_variant_name(wire: str) -> str:
    variant = re.sub(r"[^A-Za-z0-9_]", "_", wire)
    if variant[0].isdigit():
        variant = f"_{variant}"
    if variant in RUST_KEYWORDS:
        variant = f"{variant}_"
    return variant


def gen_properties(schema_dir: Path) -> str:
    wobjects_path = schema_dir / "Definitions" / "WObjects.xml"
    tree = ET.parse(wobjects_path)
    root = tree.getroot()

    entries = {}
    for tag, kind_label in (
        ("Property", "Property"),
        ("Reference", "Reference"),
        ("ObjectList", "ObjectList"),
    ):
        for el in root.findall(f".//{tag}"):
            name = el.attrib.get("Name")
            if not name or name in entries:
                continue
            ui = el.find("UserInterface")
            display = ui.attrib.get("DisplayName") if ui is not None else None
            entries[name] = {
                "kind": kind_label,
                "type": el.attrib.get("Type"),
                "meaning": el.attrib.get("DataMeaning"),
                "display": display,
            }

    sorted_names = sorted(entries)
    variants = [(name, property_variant_name(name), entries[name]) for name in sorted_names]

    lines = [
        "// Auto-generated Wwise object property, reference, and list names from WObjects.xml.",
        "// This file is generated by tools/generate_typed_api.py; do not edit by hand.",
        "",
        "/// Typed identifier for a Wwise object property, reference, or object list.",
        "///",
        "/// Derived from `WObjects.xml` in the Wwise Authoring data.",
        "/// Use [`Property::as_str`] for the bare name (e.g. `\"Volume\"`) or",
        "/// [`Property::as_at_str`] for the `@`-prefixed accessor (e.g. `\"@Volume\"`).",
        "#[allow(non_camel_case_types)]",
        "#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]",
        "pub enum Property {",
    ]
    for wire, variant, meta in variants:
        details = [f"`{wire}` ({meta['kind']})"]
        if meta["display"]:
            details.append(f"— {meta['display']}")
        qualifiers = [q for q in (meta["type"], meta["meaning"]) if q]
        if qualifiers:
            details.append(f"[{', '.join(qualifiers)}]")
        lines.append(f"    /// {' '.join(details)}.")
        lines.append(f"    {variant},")
    lines.append("}")
    lines.append("")
    lines.append("impl Property {")
    lines.append("    /// Returns the bare property name without the `@` prefix (e.g. `\"Volume\"`).")
    lines.append("    pub const fn as_str(self) -> &'static str {")
    lines.append("        match self {")
    for wire, variant, _ in variants:
        lines.append(f'            Self::{variant} => "{wire}",')
    lines.append("        }")
    lines.append("    }")
    lines.append("")
    lines.append("    /// Returns the `@`-prefixed property accessor (e.g. `\"@Volume\"`).")
    lines.append("    pub const fn as_at_str(self) -> &'static str {")
    lines.append("        match self {")
    for wire, variant, _ in variants:
        lines.append(f'            Self::{variant} => "@{wire}",')
    lines.append("        }")
    lines.append("    }")
    lines.append("}")
    lines.append("")
    lines.append("impl AsRef<str> for Property {")
    lines.append("    fn as_ref(&self) -> &str {")
    lines.append("        self.as_str()")
    lines.append("    }")
    lines.append("}")
    lines.append("")
    lines.append("impl std::fmt::Display for Property {")
    lines.append("    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {")
    lines.append("        f.write_str(self.as_str())")
    lines.append("    }")
    lines.append("}")
    lines.append("")
    lines.append("impl From<Property> for &'static str {")
    lines.append("    fn from(prop: Property) -> Self {")
    lines.append("        prop.as_str()")
    lines.append("    }")
    lines.append("}")
    lines.append("")
    lines.append("impl From<Property> for String {")
    lines.append("    fn from(prop: Property) -> Self {")
    lines.append("        prop.as_str().to_string()")
    lines.append("    }")
    lines.append("}")
    lines.append("")
    lines.append("impl serde::Serialize for Property {")
    lines.append("    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {")
    lines.append("        serializer.serialize_str(self.as_str())")
    lines.append("    }")
    lines.append("}")
    lines.append("")
    lines.append("impl<'de> serde::Deserialize<'de> for Property {")
    lines.append("    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {")
    lines.append("        let s = String::deserialize(deserializer)?;")
    lines.append("        let bare = s.strip_prefix('@').unwrap_or(s.as_str());")
    lines.append("        match bare {")
    for wire, variant, _ in variants:
        lines.append(f'            "{wire}" => Ok(Self::{variant}),')
    lines.append('            _ => Err(serde::de::Error::custom(format!("unknown Wwise Property: {s}"))),')
    lines.append("        }")
    lines.append("    }")
    lines.append("}")
    return "\n".join(lines) + "\n"


# ─────────────────────────────────────────────────────────────────────────────
# primitives.rs
# ─────────────────────────────────────────────────────────────────────────────

def gen_object_return_field(waapi_defs):
    enum_vals = waapi_defs["definitions"]["returnExpression"]["items"]["anyOf"][0]["enum"]
    lines = doc_lines("A field that can be requested in `return` options.")
    lines.append("#[derive(Debug, Clone)]")
    lines.append("pub enum ObjectReturnField {")
    arms = []
    for v, variant in unique_names(enum_vals, pascal_case, "Legacy").items():
        lines.append(f"    {variant},")
        arms.append(f"            ObjectReturnField::{variant} => \"{v}\",")
    lines.append("    /// Request a Wwise object property, reference, or list (`@Property`).")
    lines.append("    Property(Property),")
    lines.append("    /// Any other accessor, e.g. `@Property` or a dot-separated query.")
    lines.append("    Custom(String),")
    lines.append("}")
    lines.append("")
    lines.append("impl serde::Serialize for ObjectReturnField {")
    lines.append("    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {")
    lines.append("        serializer.serialize_str(match self {")
    lines += arms
    lines.append("            ObjectReturnField::Property(prop) => prop.as_at_str(),")
    lines.append("            ObjectReturnField::Custom(value) => value.as_str(),")
    lines.append("        })")
    lines.append("    }")
    lines.append("}")
    lines.append("")
    lines.append("impl From<Property> for ObjectReturnField {")
    lines.append("    fn from(prop: Property) -> Self { ObjectReturnField::Property(prop) }")
    lines.append("}")
    lines.append("impl From<&str> for ObjectReturnField {")
    lines.append("    fn from(s: &str) -> Self { ObjectReturnField::Custom(s.to_string()) }")
    lines.append("}")
    lines.append("impl From<String> for ObjectReturnField {")
    lines.append("    fn from(s: String) -> Self { ObjectReturnField::Custom(s) }")
    lines.append("}")
    return "\n".join(lines)


def gen_wwise_object(ctx):
    waapi_defs = ctx.schemas.load_definitions("waapi_definitions.json")
    obj_return = waapi_defs["definitions"]["objectReturn"]
    props = obj_return.get("properties", {})
    names = unique_names(props.keys(), lambda w: field_name(ctx.uri, w, "response"), "_legacy")
    fields = []
    for wire, sub in props.items():
        if wire == "type" and "\\ref wobjects_index" in sub.get("description", ""):
            ty = "ObjectType"
        else:
            ty = rust_type(sub, ctx)
        fields.append(Field(wire, names[wire], ty, True, sub.get("description", "")))
    lines = [
        "/// A Wwise object as returned by `object.get` and similar endpoints.",
        "/// Fields are optional because their presence depends on the `return` options;",
        "/// accessors not listed here (e.g. `@Property` values) land in `extra`.",
        "#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]",
        "pub struct WwiseObject {",
    ]
    for f in fields:
        lines += f.declaration("response")
    lines.append("    #[serde(flatten)]")
    lines.append("    pub extra: std::collections::HashMap<String, serde_json::Value>,")
    lines.append("}")
    lines.append("")
    lines.append("impl WwiseObject {")
    lines.append("    /// Look up a property value in `extra` (checks `\"@Property\"` first, then `\"Property\"`).")
    lines.append("    pub fn property(&self, prop: Property) -> Option<&serde_json::Value> {")
    lines.append("        self.extra")
    lines.append("            .get(prop.as_at_str())")
    lines.append("            .or_else(|| self.extra.get(prop.as_str()))")
    lines.append("    }")
    lines.append("}")
    return "\n".join(lines)


def gen_object_query(ctx):
    args = ctx.doc["argsSchema"]
    from_variants = args["properties"]["from"]["oneOf"]
    variants = []
    for v in from_variants:
        prop = list(v["properties"].keys())[0]
        sub = v["properties"][prop]
        if prop == "ofType" or "\\ref wobjects_index" in sub.get("description", ""):
            item_ty = "ObjectType"
        elif stringy_items(sub, ctx):
            item_ty = "String"
        else:
            item_ty = "serde_json::Value"
        variants.append((prop, pascal_case(prop), item_ty))
    lines = ["/// Argument to the `from` field of `object.get`."]
    lines.append("#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]")
    lines.append("pub enum ObjectQuery {")
    for wire, variant, item_ty in variants:
        lines.append(f'    #[serde(rename = "{wire}")]')
        lines.append(f"    {variant}(Vec<{item_ty}>),")
    lines.append("}")
    lines.append("")
    lines.append("impl From<String> for ObjectQuery {")
    lines.append("    fn from(s: String) -> Self {")
    lines.append("        if s.starts_with('{') && s.ends_with('}') {")
    lines.append("            ObjectQuery::Id(vec![serde_json::Value::String(s)])")
    lines.append("        } else if s.starts_with('\\\\') {")
    lines.append("            ObjectQuery::Path(vec![s])")
    lines.append("        } else if s.contains(':') {")
    lines.append("            ObjectQuery::Name(vec![s])")
    lines.append("        } else {")
    lines.append("            ObjectQuery::Path(vec![s])")
    lines.append("        }")
    lines.append("    }")
    lines.append("}")
    lines.append("impl From<&str> for ObjectQuery {")
    lines.append("    fn from(s: &str) -> Self { Self::from(s.to_string()) }")
    lines.append("}")
    lines.append("impl From<Vec<String>> for ObjectQuery {")
    lines.append("    fn from(v: Vec<String>) -> Self {")
    lines.append("        ObjectQuery::Id(v.into_iter().map(serde_json::Value::String).collect())")
    lines.append("    }")
    lines.append("}")
    lines.append("impl From<ObjectType> for ObjectQuery {")
    lines.append("    fn from(object_type: ObjectType) -> Self { ObjectQuery::OfType(vec![object_type]) }")
    lines.append("}")
    lines.append("impl From<Vec<ObjectType>> for ObjectQuery {")
    lines.append("    fn from(object_types: Vec<ObjectType>) -> Self { ObjectQuery::OfType(object_types) }")
    lines.append("}")
    return "\n".join(lines)


def gen_transform_select(ctx):
    args = ctx.doc["argsSchema"]
    select_variant = args["properties"]["transform"]["items"]["oneOf"][0]
    values = select_variant["properties"]["select"]["items"]["enum"]
    return emit_enum("TransformSelect", "Selector for an `object.get` select transformation.", values)


def gen_import_types(ctx):
    args = ctx.doc["argsSchema"]
    ctx.local_root = args
    item_fields = collect_fields(ctx, args["properties"]["imports"]["items"], "args", "ImportItem")
    default_fields = collect_fields(ctx, args["properties"]["default"], "args", "ImportDefaults")

    result = ctx.doc["resultSchema"]
    ctx.local_root = result
    log_fields = collect_fields(ctx, result["properties"]["log"]["items"], "response", "ImportLogEntry")
    ctx.local_root = None

    out = emit_struct("args", "ImportItem", "One item in the `imports` array for `audio.import`.",
                      item_fields, ctx, extra_flatten=True)
    out += "\n\n"
    out += emit_struct("args", "ImportDefaults", "Default values applied to every item in `audio.import`.",
                       default_fields, ctx, extra_flatten=True)
    out += "\n\n"
    # ImportLogEntry needs both Serialize and Deserialize.
    lines = ["/// One entry in the audio import log."]
    lines.append("#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]")
    lines.append("pub struct ImportLogEntry {")
    for f in log_fields:
        lines += f.declaration("response")
    lines.append("}")
    out += "\n".join(lines)
    return out


def gen_version_info(ctx):
    result = ctx.doc["resultSchema"]
    out = emit_struct("response", "VersionInfo", "Wwise version information returned by `getInfo`.",
                      collect_fields(ctx, result["properties"]["version"], "response", "VersionInfo"), ctx)
    out += "\n\n"
    out += emit_struct("response", "DirectoriesInfo", "Directories used by Wwise, returned by `getInfo`.",
                       collect_fields(ctx, result["properties"]["directories"], "response", "DirectoriesInfo"), ctx)
    # Response structs with all-optional fields should derive Default.
    out = out.replace("#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]\npub struct DirectoriesInfo",
                      "#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]\npub struct DirectoriesInfo")
    return out


def gen_project_info_types(ctx):
    result = ctx.doc["resultSchema"]
    out = emit_struct(
        "response",
        "ProjectPlatformInfo",
        "Platform defined in the project, returned by `getProjectInfo`.",
        collect_fields(ctx, result["properties"]["platforms"]["items"], "response", "ProjectPlatformInfo"),
        ctx,
    )
    out += "\n\n"
    out += emit_struct(
        "response",
        "ProjectLanguageInfo",
        "Language defined in the project, returned by `getProjectInfo`.",
        collect_fields(ctx, result["properties"]["languages"]["items"], "response", "ProjectLanguageInfo"),
        ctx,
    )
    out += "\n\n"
    out += emit_struct(
        "response",
        "ProjectDirectoriesInfo",
        "Collection of project directories returned by `getProjectInfo`.",
        collect_fields(ctx, result["properties"]["directories"], "response", "ProjectDirectoriesInfo"),
        ctx,
    )
    return out


def gen_transport_types(exec_ctx, state_ctx, list_ctx):
    action_vals = exec_ctx.doc["argsSchema"]["properties"]["action"]["enum"]
    state_vals = state_ctx.doc["resultSchema"]["properties"]["state"]["enum"]
    list_item_sch = list_ctx.doc["resultSchema"]["properties"]["list"]["items"]

    parts = [
        emit_enum("TransportAction", "Action to execute on a Wwise transport object.", action_vals),
        emit_enum("TransportState", "Playback state of a Wwise transport object.", state_vals),
        emit_struct(
            "response",
            "TransportObject",
            "A transport object entry returned by `transport.getList`.",
            collect_fields(list_ctx, list_item_sch, "response", "TransportObject"),
            list_ctx,
        ),
    ]
    return "\n\n".join(parts)


def gen_attenuation_and_object_types(waapi_defs, atten_ctx, paste_ctx, switch_ctx, open_ctx):
    curve_shape_vals = waapi_defs["definitions"]["curveShape"]["enum"]
    curve_type_vals = atten_ctx.doc["argsSchema"]["properties"]["curveType"]["enum"]
    curve_use_vals = atten_ctx.doc["resultSchema"]["properties"]["use"]["enum"]
    paste_mode_vals = paste_ctx.doc["argsSchema"]["properties"]["pasteMode"]["enum"]
    migration_vals = open_ctx.doc["argsSchema"]["properties"]["onMigrationRequired"]["enum"]

    curve_point_rust = """/// A 2D point in an attenuation or RTPC curve (`pointsDefinition`).
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CurvePoint2D {
    /// X coordinate of the curve point.
    pub x: f64,
    /// Y coordinate of the curve point.
    pub y: f64,
    /// Shape of the curve segment between this point and the next.
    pub shape: CurveShape,
}

impl CurvePoint2D {
    pub const fn new(x: f64, y: f64, shape: CurveShape) -> Self {
        Self { x, y, shape }
    }
}"""

    switch_assignment_rust = """/// Assignment pairing a Switch Container child object with a State or Switch (`switchContainer.getAssignments`).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SwitchContainerAssignment {
    /// GUID of the child object assigned to a State or Switch.
    pub child: String,
    /// GUID of the State or Switch to which the child is assigned.
    #[serde(rename = "stateOrSwitch")]
    pub state_or_switch: String,
}"""

    parts = [
        emit_enum("CurveShape", "Shape of a curve segment between two points.", curve_shape_vals),
        curve_point_rust,
        emit_enum("AttenuationCurveType", "Type of an attenuation curve.", curve_type_vals),
        emit_enum("AttenuationCurveUse", "Usage mode of an attenuation curve.", curve_use_vals),
        emit_enum("PasteMode", "Paste mode for lists in `object.pasteProperties`.", paste_mode_vals),
        switch_assignment_rust,
        emit_enum("ProjectMigrationAction", "Action when opening a project that requires migration.", migration_vals),
    ]
    return "\n\n".join(parts)


def gen_soundengine_and_misc_types():
    """Shared types for the SoundEngine runtime surface, object introspection,
    blend containers, soundbanks, and audio conversion."""
    parts = [
        emit_int_enum(
            "ActionOnEventType",
            "Action to execute on an event via `soundengine.executeActionOnEvent`.",
            [("Stop", 0), ("Pause", 1), ("Resume", 2), ("Break", 3), ("ReleaseEnvelope", 4)],
        ),
        emit_int_enum(
            "AkCurveInterpolation",
            "Interpolation curve for fades (`AkCurveInterpolation`).",
            [("Log3", 0), ("Sine", 1), ("Log1", 2), ("InvSCurve", 3), ("Linear", 4),
             ("SCurve", 5), ("Exp1", 6), ("SineRecip", 7), ("Exp3", 8), ("Constant", 9)],
        ),
        emit_int_enum(
            "MultiPositionType",
            "How multiple positions are combined (`soundengine.setMultiplePositions`).",
            [("SingleSource", 0), ("MultiSources", 1), ("MultiDirections", 2)],
        ),
        """/// Auxiliary send value for a game object (`soundengine.setGameObjectAuxSendValues`).
#[derive(Debug, Clone, serde::Serialize)]
pub struct AuxSendValue {
    /// Game object receiving the aux send.
    pub listener: GameObjectRef,
    /// Auxiliary bus to send to.
    #[serde(rename = "auxBus")]
    pub aux_bus: SoundEngineObjectRef,
    /// Gain control value in dB.
    #[serde(rename = "controlValue")]
    pub control_value: f64,
}

impl AuxSendValue {
    pub fn new(listener: impl Into<GameObjectRef>, aux_bus: impl Into<SoundEngineObjectRef>, control_value: f64) -> Self {
        Self { listener: listener.into(), aux_bus: aux_bus.into(), control_value }
    }
}""",
        """/// A single position entry for `soundengine.setMultiplePositions`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct MultiPosition {
    pub position: Position,
}

impl MultiPosition {
    pub fn new(position: Position) -> Self {
        Self { position }
    }
}

impl From<Position> for MultiPosition {
    fn from(position: Position) -> Self {
        Self { position }
    }
}""",
        """/// A Wwise object type entry returned by `object.getTypes`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct WwiseObjectTypeInfo {
    /// The object type name (e.g. `"Sound"`, `"ActorMixer"`).
    #[serde(rename = "type")]
    pub object_type: String,
    /// The class ID of the object type.
    #[serde(rename = "classId")]
    pub class_id: u32,
    /// The display name of the object type.
    pub name: String,
}""",
        emit_enum("RtpcSupportMode", "How a property supports RTPC (`object.getPropertyInfo`).",
                  ["None", "Additive", "Exclusive", "Multiplicative"]),
        """/// RTPC/randomizer/unlink support flags for a property (`object.getPropertyInfo`).
#[derive(Debug, Clone, Copy, Default, serde::Serialize, serde::Deserialize)]
pub struct PropertyInfoSupports {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtpc: Option<RtpcSupportMode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub randomizer: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unlink: Option<bool>,
}""",
        emit_enum("BlendFadeMode", "Fade mode of a blend track edge.", ["None", "Manual", "Automatic"]),
        """/// An edge of a blend track (`blendContainer.addAssignment` / `getAssignments`).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BlendEdge {
    #[serde(rename = "fadeMode")]
    pub fade_mode: BlendFadeMode,
    #[serde(rename = "fadeShape")]
    pub fade_shape: CurveShape,
    #[serde(rename = "edgePosition")]
    pub edge_position: f64,
    #[serde(default, rename = "fadePosition", skip_serializing_if = "Option::is_none")]
    pub fade_position: Option<f64>,
}""",
        """/// A blend track assignment (`blendContainer.getAssignments`).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BlendTrackAssignment {
    /// GUID of the assigned child.
    pub child: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edges: Option<Vec<BlendEdge>>,
}""",
        emit_enum("OnCurveUpdate", "Action when a Game Parameter range is modified.", ["stretch", "preserveX"]),
        emit_enum("SoundBankInclusionType", "Inclusion type used by `soundbank.generate`.", ["event", "structure", "media"]),
        emit_enum("SoundBankInclusionFilter", "Inclusion relation used by `soundbank.get/setInclusions`.", ["events", "structures", "media"]),
        emit_enum("SoundBankInclusionOperation", "How `soundbank.setInclusions` modifies the inclusion list.", ["add", "remove", "replace"]),
        """/// A SoundBank inclusion (`soundbank.get/setInclusions`).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SoundBankInclusion {
    pub object: ObjectRef,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub filter: Vec<SoundBankInclusionFilter>,
}

impl SoundBankInclusion {
    pub fn new(object: impl Into<ObjectRef>) -> Self {
        Self { object: object.into(), filter: Vec::new() }
    }

    pub fn filter(mut self, filter: SoundBankInclusionFilter) -> Self {
        self.filter.push(filter);
        self
    }
}""",
        """/// A user-defined SoundBank to generate (`soundbank.generate`).
#[derive(Debug, Clone, serde::Serialize)]
pub struct SoundBankInfo {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inclusions: Option<Vec<SoundBankInclusionType>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<ObjectRef>>,
    #[serde(default, rename = "auxBusses", skip_serializing_if = "Option::is_none")]
    pub aux_busses: Option<Vec<ObjectRef>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rebuild: Option<bool>,
}

impl SoundBankInfo {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), inclusions: None, events: None, aux_busses: None, rebuild: None }
    }

    pub fn inclusions(mut self, inclusions: Vec<SoundBankInclusionType>) -> Self {
        self.inclusions = Some(inclusions);
        self
    }

    pub fn events(mut self, events: Vec<ObjectRef>) -> Self {
        self.events = Some(events);
        self
    }

    pub fn aux_busses(mut self, aux_busses: Vec<ObjectRef>) -> Self {
        self.aux_busses = Some(aux_busses);
        self
    }

    pub fn rebuild(mut self, rebuild: bool) -> Self {
        self.rebuild = Some(rebuild);
        self
    }
}""",
        """/// A central log entry (`logCentralItem`).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LogCentralItem {
    pub severity: LogSeverity,
    pub time: i64,
    #[serde(rename = "messageId")]
    pub message_id: String,
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub platform: Option<WwiseObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<String>>,
}""",
        emit_enum("LogChannel", "The Wwise log channel (`log.itemAdded`).",
                  ["soundbankGenerate", "conversion", "copyPlatformSettings", "waapi",
                   "projectLoad", "general", "sourceControl", "lua"]),
        """/// A source file to convert (`soundbank.convertExternalSources`).
#[derive(Debug, Clone, serde::Serialize)]
pub struct ExternalSourceInfo {
    pub platform: PlatformRef,
    pub input: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
}

impl ExternalSourceInfo {
    pub fn new(platform: impl Into<PlatformRef>, input: impl Into<String>) -> Self {
        Self { platform: platform.into(), input: input.into(), output: None }
    }

    pub fn output(mut self, output: impl Into<String>) -> Self {
        self.output = Some(output.into());
        self
    }
}""",
        """/// A logged error from `audio.convert`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AudioConvertError {
    pub severity: LogSeverity,
    pub message: String,
}""",
    ]
    return "\n\n".join(parts)


def gen_primitives(schemas, endpoints_docs):
    waapi_defs = schemas.load_definitions("waapi_definitions.json")
    get_ctx = endpoints_docs["ak.wwise.core.object.get"]
    info_ctx = endpoints_docs["ak.wwise.core.getInfo"]
    proj_info_ctx = endpoints_docs["ak.wwise.core.getProjectInfo"]
    import_ctx = endpoints_docs["ak.wwise.core.audio.import"]
    exec_ctx = endpoints_docs["ak.wwise.core.transport.executeAction"]
    state_ctx = endpoints_docs["ak.wwise.core.transport.getState"]
    list_ctx = endpoints_docs["ak.wwise.core.transport.getList"]
    atten_ctx = endpoints_docs["ak.wwise.core.object.getAttenuationCurve"]
    paste_ctx = endpoints_docs["ak.wwise.core.object.pasteProperties"]
    switch_ctx = endpoints_docs["ak.wwise.core.switchContainer.getAssignments"]
    open_ctx = endpoints_docs["ak.wwise.ui.project.open"]

    import_operation_vals = [
        b["enum"][0] for b in waapi_defs["definitions"]["importOperationType"]["oneOf"]
    ]
    log_severity_vals = [
        b["enum"][0] for b in waapi_defs["definitions"]["logCentralSeverity"]["oneOf"]
    ]

    parts = [HEADER]
    parts.append("pub use super::object_types::ObjectType;")
    parts.append("pub use super::properties::Property;")
    parts.append("pub use crate::api::primitives::{")
    parts.append("    ChildObject, ListMode, ObjectRef, OnNameConflict, PlatformRef, Position,")
    parts.append("    PropertyValue, Vector3,")
    parts.append("};")
    parts.append("")
    parts.append(STATIC_REFS)
    parts.append(gen_object_return_field(waapi_defs))
    parts.append(STATIC_OBJECT_RETURN_OPTIONS)
    parts.append(emit_enum("BusReturnField", "Return fields for `ak.wwise.core.profiler.getBusses`.",
                           waapi_defs["definitions"]["busReturnOptions"]["items"]["enum"]))
    parts.append(emit_enum("AudioObjectReturnField", "Return fields for `ak.wwise.core.profiler.getAudioObjects`.",
                           waapi_defs["definitions"]["audioObjectReturnOptions"]["items"]["enum"]))
    parts.append(emit_enum("VoiceReturnField", "Return fields for `ak.wwise.core.profiler.getVoices`.",
                           waapi_defs["definitions"]["voiceReturnOptions"]["items"]["enum"]))
    parts.append(emit_enum("ImportOperation", "Determines how imported objects are created.", import_operation_vals))
    parts.append(emit_enum("LogSeverity", "Severity level of an audio import log entry.", log_severity_vals))
    parts.append(gen_wwise_object(get_ctx))
    parts.append(STATIC_WWISE_OBJECT_REF)
    parts.append(gen_object_query(get_ctx))
    parts.append(gen_transform_select(get_ctx))
    parts.append(STATIC_TRANSFORM)
    parts.append(gen_version_info(info_ctx))
    parts.append(gen_project_info_types(proj_info_ctx))
    parts.append(gen_import_types(import_ctx))
    parts.append(gen_transport_types(exec_ctx, state_ctx, list_ctx))
    parts.append(gen_attenuation_and_object_types(waapi_defs, atten_ctx, paste_ctx, switch_ctx, open_ctx))
    parts.append(gen_soundengine_and_misc_types())
    return "\n\n".join(parts) + "\n"


HEADER = """// Shared primitives referenced by generated typed bindings.
// This file is generated by tools/generate_typed_api.py; do not edit by hand.
"""

STATIC_REFS = """/// Object reference accepted by soundengine endpoints: name, GUID, or Short ID.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum SoundEngineObjectRef {
    Name(String),
    Guid(String),
    ShortId(u32),
}

impl From<&str> for SoundEngineObjectRef {
    fn from(s: &str) -> Self { SoundEngineObjectRef::Name(s.to_string()) }
}
impl From<String> for SoundEngineObjectRef {
    fn from(s: String) -> Self { SoundEngineObjectRef::Name(s) }
}
impl From<u32> for SoundEngineObjectRef {
    fn from(id: u32) -> Self { SoundEngineObjectRef::ShortId(id) }
}

/// Event reference accepted by `soundengine.postEvent`.
pub type EventRef = SoundEngineObjectRef;

/// RTPC reference accepted by `soundengine.setRTPCValue`.
pub type RtpcRef = SoundEngineObjectRef;

/// Game object reference accepted by soundengine calls.
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum GameObjectRef {
    Id(u64),
}

impl GameObjectRef {
    pub fn game_object(id: u64) -> Self {
        GameObjectRef::Id(id)
    }
}

impl From<u64> for GameObjectRef {
    fn from(id: u64) -> Self { GameObjectRef::Id(id) }
}
impl From<u32> for GameObjectRef {
    fn from(id: u32) -> Self { GameObjectRef::Id(id as u64) }
}

/// Language reference used in return options.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum LanguageRef {
    Name(String),
    Guid(String),
}

impl From<&str> for LanguageRef {
    fn from(s: &str) -> Self { LanguageRef::Name(s.to_string()) }
}
impl From<String> for LanguageRef {
    fn from(s: String) -> Self { LanguageRef::Name(s) }
}"""

STATIC_OBJECT_RETURN_OPTIONS = """/// Options controlling which fields are returned for each object.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct ObjectReturnOptions {
    #[serde(rename = "return", skip_serializing_if = "Option::is_none")]
    pub return_fields: Option<Vec<ObjectReturnField>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<PlatformRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<LanguageRef>,
}

impl ObjectReturnOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn return_fields<I, T>(mut self, fields: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<ObjectReturnField>,
    {
        self.return_fields = Some(fields.into_iter().map(Into::into).collect());
        self
    }

    pub fn platform(mut self, platform: impl Into<PlatformRef>) -> Self {
        self.platform = Some(platform.into());
        self
    }

    pub fn language(mut self, language: impl Into<LanguageRef>) -> Self {
        self.language = Some(language.into());
        self
    }
}"""

STATIC_WWISE_OBJECT_REF = """/// A lightweight object reference returned inside other objects.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct WwiseObjectRef {
    #[serde(default, rename = "id")]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
}"""

STATIC_TRANSFORM = """/// A single transformation step in an `object.get` query.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum Transform {
    Select {
        select: Vec<TransformSelect>,
    },
    Distinct(String),
    Range {
        range: Vec<usize>,
    },
    Where {
        #[serde(rename = "where")]
        where_: Vec<serde_json::Value>,
    },
}

impl Transform {
    /// Select related objects (`parent`, `children`, `descendants`, `ancestors`, or `referencesTo`).
    pub fn select(selector: TransformSelect) -> Self {
        Transform::Select {
            select: vec![selector],
        }
    }

    /// Deduplicate the current object list (`"distinct"`).
    pub fn distinct() -> Self {
        Transform::Distinct("distinct".to_string())
    }

    /// Take a slice of results starting at 0-based `from` with length `count` (`[from, count]`).
    pub fn range(from: usize, count: usize) -> Self {
        Transform::Range {
            range: vec![from, count],
        }
    }

    /// Filter objects whose name contains `substring` (case-insensitive, `["name:contains", substring]`).
    pub fn where_name_contains(substring: impl Into<String>) -> Self {
        Transform::Where {
            where_: vec![
                serde_json::Value::String("name:contains".to_string()),
                serde_json::Value::String(substring.into()),
            ],
        }
    }

    /// Filter objects whose name matches regular expression `regex` (`["name:matches", regex]`).
    pub fn where_name_matches(regex: impl Into<String>) -> Self {
        Transform::Where {
            where_: vec![
                serde_json::Value::String("name:matches".to_string()),
                serde_json::Value::String(regex.into()),
            ],
        }
    }

    /// Filter objects whose type is in `types` (`["type:isIn", [...]]`).
    pub fn where_type_is_in(types: impl IntoIterator<Item = ObjectType>) -> Self {
        let list = types
            .into_iter()
            .map(|t| serde_json::Value::String(t.as_str().to_string()))
            .collect();
        Transform::Where {
            where_: vec![
                serde_json::Value::String("type:isIn".to_string()),
                serde_json::Value::Array(list),
            ],
        }
    }

    /// Filter objects whose category is in `categories` (`["category:isIn", [...]]`).
    pub fn where_category_is_in<I, S>(categories: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let list = categories
            .into_iter()
            .map(|c| serde_json::Value::String(c.into()))
            .collect();
        Transform::Where {
            where_: vec![
                serde_json::Value::String("category:isIn".to_string()),
                serde_json::Value::Array(list),
            ],
        }
    }
}

impl From<TransformSelect> for Transform {
    fn from(selector: TransformSelect) -> Self {
        Transform::select(selector)
    }
}"""


# ─────────────────────────────────────────────────────────────────────────────
# Endpoint files
# ─────────────────────────────────────────────────────────────────────────────

def endpoint_segments(uri):
    return uri.split(".")


def endpoint_base_name(uri):
    segs = endpoint_segments(uri)
    if segs[:2] == ["ak", "wwise"]:
        segs = segs[2:]
    else:
        segs = segs[1:]
    return "".join(pascal_case(s) for s in segs)


def endpoint_file_stem(uri):
    return snake_case(endpoint_segments(uri)[-1])


def endpoint_module_path(uri):
    return endpoint_segments(uri)[:-1]


def method_name(uri):
    return snake_case(endpoint_segments(uri)[-1])


def is_empty_schema(sch):
    return not sch.get("properties") and "$ref" not in sch


def gen_endpoint_file(ctx):
    base = endpoint_base_name(ctx.uri)
    args_schema = ctx.doc.get("argsSchema", {})
    options_schema = ctx.doc.get("optionsSchema", {})
    result_schema = ctx.doc.get("resultSchema", {})
    if ctx.uri in RESULT_WRAPPED_OBJECT_RETURN:
        result_schema = {
            "type": "object",
            "properties": {"return": {"$ref": OBJECT_RETURN_REF}},
            "required": ["return"],
        }

    has_args = not is_empty_schema(args_schema)
    has_options = not is_empty_schema(options_schema)
    has_response = not is_empty_schema(result_schema)
    options_is_object_return = options_schema.get("$ref") == OBJECT_RETURN_OPTIONS_REF
    result_is_object_return = result_schema.get("$ref") == OBJECT_RETURN_REF

    sections = [f"// Generated bindings for {ctx.uri}", "", "#![allow(unused_imports)]",
                "#![allow(dead_code)]", "", "use crate::api::generated::primitives::*;", ""]
    exports = []

    if has_args:
        name = f"{base}Args"
        exports.append(name)
        if ctx.uri == "ak.wwise.core.object.get":
            sections.append(gen_object_get_args(ctx, name))
        elif ctx.uri == "ak.wwise.core.object.set":
            item_name = f"{base}Item"
            exports.append(item_name)
            sections.append(gen_object_set_args(ctx, name, item_name))
        else:
            ctx.local_root = args_schema
            fields = collect_fields(ctx, args_schema, "args", name)
            ctx.local_root = None
            extra_flatten = bool(args_schema.get("patternProperties"))
            sections.append(emit_struct("args", name, f"Arguments for `{ctx.uri}`.",
                                        fields, ctx, ctor_order=CTOR_ORDER.get(ctx.uri),
                                        required_order=schema_required(args_schema),
                                        extra_flatten=extra_flatten))
    if has_options:
        name = f"{base}Options"
        exports.append(name)
        if options_is_object_return:
            sections.append(f"/// Options for `{ctx.uri}`.\npub type {name} = ObjectReturnOptions;")
        else:
            ctx.local_root = options_schema
            fields = collect_fields(ctx, options_schema, "options", name)
            ctx.local_root = None
            sections.append(emit_struct("options", name, f"Options for `{ctx.uri}`.", fields, ctx))
    if has_response:
        name = f"{base}Response"
        exports.append(name)
        if result_is_object_return:
            sections.append(f"/// Response from `{ctx.uri}`.\npub type {name} = WwiseObject;")
        elif ctx.uri == "ak.wwise.core.object.set":
            item_name = f"{base}Object"
            exports.append(item_name)
            sections.append(gen_object_set_response(ctx, name, item_name))
        else:
            ctx.local_root = result_schema
            fields = collect_fields(ctx, result_schema, "response", name)
            ctx.local_root = None
            sections.append(emit_struct("response", name, f"Response from `{ctx.uri}`.", fields, ctx))

    return "\n\n".join(sections) + "\n", exports, has_args, has_options, has_response


def gen_topic_file(ctx):
    """Generate the payload struct and subscription options for a WAAPI topic."""
    base = endpoint_base_name(ctx.uri)
    options_schema = ctx.doc.get("optionsSchema", {})
    publish_schema = ctx.doc.get("publishSchema", {})

    has_options = not is_empty_schema(options_schema)
    options_is_object_return = options_schema.get("$ref") == OBJECT_RETURN_OPTIONS_REF
    publish_is_empty = is_empty_schema(publish_schema)

    sections = [f"// Generated bindings for topic {ctx.uri}", "",
                "#![allow(unused_imports)]", "#![allow(dead_code)]", "",
                "use crate::api::generated::primitives::*;", ""]
    exports = []

    payload_name = f"{base}Payload"
    exports.append(payload_name)
    if publish_is_empty:
        sections.append(f"/// Payload for `{ctx.uri}` (no data).\n"
                        "#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]\n"
                        f"pub struct {payload_name} {{}}")
    else:
        ctx.local_root = publish_schema
        fields = collect_fields(ctx, publish_schema, "response", payload_name)
        ctx.local_root = None
        sections.append(emit_struct("response", payload_name, f"Payload for `{ctx.uri}`.", fields, ctx))

    if has_options:
        name = f"{base}Options"
        exports.append(name)
        if options_is_object_return:
            sections.append(f"/// Subscription options for `{ctx.uri}`.\npub type {name} = ObjectReturnOptions;")
        else:
            ctx.local_root = options_schema
            fields = collect_fields(ctx, options_schema, "options", name)
            ctx.local_root = None
            sections.append(emit_struct("options", name, f"Subscription options for `{ctx.uri}`.", fields, ctx))

    return "\n\n".join(sections) + "\n", exports, has_options


def gen_object_get_args(ctx, name):
    return f"""/// Arguments for `ak.wwise.core.object.get`. Specify either a WAQL query or
/// a `from` query; both are optional in the schema but one is required in practice.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct {name} {{
    /// A WAQL query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waql: Option<String>,
    /// The query starting point.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<ObjectQuery>,
    /// Sequential transformations on the object list returned by `from`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform: Option<Vec<Transform>>,
}}

impl {name} {{
    pub fn new(from: ObjectQuery) -> Self {{
        Self {{
            waql: None,
            from: Some(from),
            transform: None,
        }}
    }}

    pub fn from_waql(waql: impl Into<String>) -> Self {{
        Self {{
            waql: Some(waql.into()),
            from: None,
            transform: None,
        }}
    }}

    pub fn waql(mut self, waql: impl Into<String>) -> Self {{
        self.waql = Some(waql.into());
        self
    }}

    pub fn from(mut self, from: impl Into<ObjectQuery>) -> Self {{
        self.from = Some(from.into());
        self
    }}

    pub fn transform(mut self, transform: impl Into<Vec<Transform>>) -> Self {{
        self.transform = Some(transform.into());
        self
    }}
}}"""


# Field type overrides for the nested `objects` items of `object.set`.
OBJECT_SET_ITEM_OVERRIDES = {
    "children": "Vec<ChildObject>",
    "onNameConflict": "OnNameConflict",
    "listMode": "ListMode",
}


def gen_object_set_args(ctx, args_name, item_name):
    """Emit the `objects` item struct and the top-level args struct."""
    args_schema = ctx.doc["argsSchema"]
    item_schema = args_schema["properties"]["objects"]["items"]
    ctx.local_root = args_schema
    fields = []
    for wire, sub in (item_schema.get("properties") or {}).items():
        ty = OBJECT_SET_ITEM_OVERRIDES.get(wire)
        if ty is None:
            ty = rust_type(sub, ctx, self_name=item_name)
        optional = wire not in (item_schema.get("required") or [])
        fields.append(Field(wire, field_name(ctx.uri, wire, "args"), ty, optional,
                            sub.get("description", "")))
    out = emit_struct("args", item_name,
                      "One object specification in the `objects` array of `object.set`.",
                      fields, ctx, required_order=item_schema.get("required"),
                      extra_flatten=bool(item_schema.get("patternProperties")))
    fields = collect_fields(ctx, args_schema, "args", args_name)
    ctx.local_root = None
    for f in fields:
        if f.wire == "objects":
            f.ty = f"Vec<{item_name}>"
    out += "\n\n"
    out += emit_struct("args", args_name, f"Arguments for `{ctx.uri}`.", fields, ctx,
                       required_order=args_schema.get("required"))
    return out


def gen_object_set_response(ctx, resp_name, item_name):
    """Emit the response `objects` item struct and the response struct."""
    result_schema = ctx.doc["resultSchema"]
    item_schema = result_schema["properties"]["objects"]["items"]
    ctx.local_root = result_schema
    fields = collect_fields(ctx, item_schema, "response", item_name)
    out = emit_struct("response", item_name,
                      "One object entry in the `object.set` response.",
                      fields, ctx,
                      extra_flatten=bool(item_schema.get("patternProperties")))
    fields = collect_fields(ctx, result_schema, "response", resp_name)
    ctx.local_root = None
    for f in fields:
        if f.wire == "objects":
            f.ty = f"Vec<{item_name}>"
    out += "\n\n"
    out += emit_struct("response", resp_name, f"Response from `{ctx.uri}`.", fields, ctx)
    return out


# ─────────────────────────────────────────────────────────────────────────────
# Module tree
# ─────────────────────────────────────────────────────────────────────────────

def build_module_tree(endpoints):
    tree = {}
    for uri, info in endpoints.items():
        node = tree
        for seg in endpoint_module_path(uri):
            node = node.setdefault(seg, {})
        node.setdefault("__endpoints__", []).append((uri, info))
    return tree


def emit_module_tree(tree, dir_path):
    lines = ["// Auto-generated typed WAAPI bindings.",
             "// This file is generated by tools/generate_typed_api.py; do not edit by hand.", ""]
    for name in sorted(k for k in tree if k != "__endpoints__"):
        child_dir = dir_path / name
        child_dir.mkdir(parents=True, exist_ok=True)
        emit_module_tree(tree[name], child_dir)
        if not name.islower():
            lines.append("#[allow(non_snake_case)]")
        lines.append(f"pub mod {name};")
    for uri, info in sorted(tree.get("__endpoints__", [])):
        stem = endpoint_file_stem(uri)
        (dir_path / f"{stem}.rs").write_text(info["content"])
        if stem == dir_path.name:
            lines.append("#[allow(clippy::module_inception)]")
        lines.append(f"mod {stem};")
        if info["exports"]:
            lines.append(f"pub use {stem}::{{")
            for e in info["exports"]:
                lines.append(f"    {e},")
            lines.append("};")
    (dir_path / "mod.rs").write_text("\n".join(lines) + "\n")


# ─────────────────────────────────────────────────────────────────────────────
# Client code (chained namespace clients)
# ─────────────────────────────────────────────────────────────────────────────

def endpoint_call_shape(info):
    has_args, has_options, has_response = info["has_args"], info["has_options"], info["has_response"]
    params = []
    if has_args:
        params.append(("args", f"{info['base']}Args"))
    if has_options:
        params.append(("options", f"{info['base']}Options"))
    return params, has_response


def emit_call_body(uri, info, target="self.client"):
    has_args, has_options, has_response = info["has_args"], info["has_options"], info["has_response"]
    method = "call_typed" if has_response else "call_typed_void"

    type_args = []
    type_args.append("_" if has_args else "()")
    type_args.append("_" if has_options else "()")
    if has_response:
        type_args.append("_")

    turbofish = ""
    if any(t != "_" for t in type_args):
        turbofish = f"::<{', '.join(type_args)}>"

    args_expr = "Some(args)" if has_args else "None"
    options_expr = "Some(options)" if has_options else "None"
    uri_variant = f"WaapiFunction::{uri.replace('.', '_')}"
    call = f"{target}.{method}{turbofish}({uri_variant}, {args_expr}, {options_expr}).await"

    if has_response:
        return [call]
    else:
        return [f"{call}?;", "Ok(())"]


def use_path(uri, types):
    mod_path = "::".join(endpoint_module_path(uri))
    return f"use crate::api::generated::{mod_path}::{{{', '.join(types)}}};"


def client_struct_name(path_segments):
    if path_segments == ("ak",):
        return "AkClient"
    return "".join(pascal_case(s) for s in path_segments[1:]) + "Client"


def build_chain_tree(uris):
    """Build an ordered list of (struct_name, prefix, [(method, child_struct)]) from endpoint URIs."""
    children_by_path = {}
    all_paths = []

    for uri in uris:
        mod_segs = tuple(endpoint_module_path(uri))
        for depth in range(1, len(mod_segs) + 1):
            prefix_segs = mod_segs[:depth]
            if prefix_segs not in children_by_path:
                children_by_path[prefix_segs] = []
                all_paths.append(prefix_segs)
            if depth < len(mod_segs):
                next_seg = mod_segs[depth]
                child_segs = mod_segs[: depth + 1]
                entry = (snake_case(next_seg), client_struct_name(child_segs))
                if entry not in children_by_path[prefix_segs]:
                    children_by_path[prefix_segs].append(entry)

    # Sort breadth-first by depth while preserving discovery order at each depth.
    all_paths.sort(key=len)
    return [
        (client_struct_name(p), ".".join(p), children_by_path[p])
        for p in all_paths
    ]


def gen_clients(endpoints, topics):
    lines = ["// Auto-generated chained namespace clients.",
             "// This file is generated by tools/generate_typed_api.py; do not edit by hand.",
             "", "use crate::{WaapiClient, WaapiFunction, WaapiResult, WaapiTopic};",
             "use tokio::sync::mpsc::UnboundedReceiver;",
             ""]
    for uri in sorted(endpoints):
        info = endpoints[uri]
        types = [f"{info['base']}Args"] if info["has_args"] else []
        if info["has_options"]:
            types.append(f"{info['base']}Options")
        if info["has_response"]:
            types.append(f"{info['base']}Response")
        if types:
            lines.append(use_path(uri, types))
    for uri in sorted(topics):
        info = topics[uri]
        types = [f"{info['base']}Payload"]
        if info["has_options"]:
            types.append(f"{info['base']}Options")
        lines.append(use_path(uri, types))
    lines.append("")

    all_uris = sorted(set(PILOT_ENDPOINTS) | set(TOPIC_ENDPOINTS))
    by_prefix = {}
    for uri in PILOT_ENDPOINTS:
        prefix = ".".join(endpoint_module_path(uri))
        by_prefix.setdefault(prefix, []).append(uri)
    topic_by_prefix = {}
    for uri in TOPIC_ENDPOINTS:
        prefix = ".".join(endpoint_module_path(uri))
        topic_by_prefix.setdefault(prefix, []).append(uri)

    for struct_name, prefix, children in build_chain_tree(all_uris):
        lines.append(f"pub struct {struct_name}<'a> {{")
        lines.append("    client: &'a mut WaapiClient,")
        lines.append("}")
        lines.append("")
        lines.append(f"impl<'a> {struct_name}<'a> {{")
        for method, child in children:
            lines.append(f"    pub fn {method}(self) -> {child}<'a> {{")
            lines.append(f"        {child} {{ client: self.client }}")
            lines.append("    }")
            lines.append("")
        for uri in by_prefix.get(prefix or "", []):
            info = endpoints[uri]
            params, has_response = endpoint_call_shape(info)
            ret = f"WaapiResult<{info['base']}Response>" if has_response else "WaapiResult<()>"
            param_list = ",\n        ".join(f"{n}: {t}" for n, t in params)
            sig = f"        &mut self,\n        {param_list}," if params else "        &mut self,"
            lines.append(f"    /// `{uri}`.")
            lines.append(f"    pub async fn {method_name(uri)}(")
            lines.append(sig)
            lines.append(f"    ) -> {ret} {{")
            body = emit_call_body(uri, info)
            lines += [f"        {b}" for b in body]
            lines.append("    }")
            lines.append("")
        for uri in topic_by_prefix.get(prefix or "", []):
            info = topics[uri]
            payload = f"{info['base']}Payload"
            method = f"subscribe_{method_name(uri)}"
            lines.append(f"    /// Subscribe to `{uri}`.")
            if info["has_options"]:
                opts = f"{info['base']}Options"
                lines.append(f"    pub async fn {method}(")
                lines.append(f"        &mut self,")
                lines.append(f"        options: Option<{opts}>,")
                lines.append(f"    ) -> WaapiResult<(u64, UnboundedReceiver<{payload}>)> {{")
                lines.append(f"        self.client.subscribe_payload(WaapiTopic::{uri.replace('.', '_')}, options).await")
                lines.append("    }")
            else:
                lines.append(f"    pub async fn {method}(")
                lines.append(f"        &mut self,")
                lines.append(f"    ) -> WaapiResult<(u64, UnboundedReceiver<{payload}>)> {{")
                lines.append(f"        self.client.subscribe_payload::<(), _>(WaapiTopic::{uri.replace('.', '_')}, None).await")
                lines.append("    }")
            lines.append("")
        if lines[-1] == "":
            lines.pop()
        lines.append("}")
        lines.append("")

    lines.append("impl WaapiClient {")
    lines.append("    /// Chained namespace access starting at `ak`.")
    lines.append("    pub fn ak(&mut self) -> AkClient<'_> {")
    lines.append("        AkClient { client: self }")
    lines.append("    }")
    lines.append("")
    lines.append("    /// Shortcut for `client.ak().wwise()`.")
    lines.append("    pub fn wwise(&mut self) -> WwiseClient<'_> {")
    lines.append("        WwiseClient { client: self }")
    lines.append("    }")
    lines.append("")
    lines.append("    /// Shortcut for `client.ak().soundengine()`.")
    lines.append("    pub fn soundengine(&mut self) -> SoundengineClient<'_> {")
    lines.append("        SoundengineClient { client: self }")
    lines.append("    }")
    lines.append("}")
    return "\n".join(lines) + "\n"


# ─────────────────────────────────────────────────────────────────────────────
# Main
# ─────────────────────────────────────────────────────────────────────────────

def main():
    schemas = Schemas(SCHEMA_DIR)
    endpoints = {}
    for uri in PILOT_ENDPOINTS:
        ctx = Ctx(schemas, uri)
        content, exports, has_args, has_options, has_response = gen_endpoint_file(ctx)
        endpoints[uri] = {
            "content": content,
            "exports": exports,
            "has_args": has_args,
            "has_options": has_options,
            "has_response": has_response,
            "base": endpoint_base_name(uri),
            "ctx": ctx,
        }

    topics = {}
    for uri in TOPIC_ENDPOINTS:
        ctx = Ctx(schemas, uri)
        content, exports, has_options = gen_topic_file(ctx)
        topics[uri] = {
            "content": content,
            "exports": exports,
            "has_options": has_options,
            "base": endpoint_base_name(uri),
            "ctx": ctx,
        }

    if OUT_DIR.exists():
        shutil.rmtree(OUT_DIR)
    OUT_DIR.mkdir(parents=True)

    tree = build_module_tree({**endpoints, **topics})
    emit_module_tree(tree, OUT_DIR)

    ctxs = {uri: info["ctx"] for uri, info in endpoints.items()}
    (OUT_DIR / "object_types.rs").write_text(gen_object_types(SCHEMA_DIR))
    (OUT_DIR / "properties.rs").write_text(gen_properties(SCHEMA_DIR))
    (OUT_DIR / "primitives.rs").write_text(gen_primitives(schemas, ctxs))
    (OUT_DIR / "clients.rs").write_text(gen_clients(endpoints, topics))
    (OUT_DIR / "mod.rs").write_text(
        "// Auto-generated typed WAAPI bindings.\n"
        "// This file is generated by tools/generate_typed_api.py; do not edit by hand.\n\n"
        "pub mod ak;\n"
        "pub mod clients;\n"
        "pub mod object_types;\n"
        "pub mod primitives;\n"
        "pub mod properties;\n\n"
        "pub use object_types::ObjectType;\n"
        "pub use properties::Property;\n"
    )
    print(f"Generated {len(PILOT_ENDPOINTS)} endpoints and {len(TOPIC_ENDPOINTS)} topics into {OUT_DIR}")


if __name__ == "__main__":
    main()
