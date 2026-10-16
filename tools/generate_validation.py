#!/usr/bin/env python3
"""Bind every frozen semantic node to an explicitly reviewed native operation.

This generator reads source expressions, never evaluates JavaScript, and fails on
unrecognized semantics. Implementation/test evidence is verified by native tests;
the registry itself does not claim that generation establishes conformance.
"""
import hashlib
import json
from pathlib import Path
import re
import sys

ROOT = Path(__file__).resolve().parents[1]
source = json.loads((ROOT / "contracts/typescript.json").read_text())
if source["baseline"]["sourceFingerprint"] != "a972723bb524cf8fce652d50297d94ed7af278b13b9a045b5c6bdc79212eb48f":
    raise ValueError("Frozen source or referenced helper changed and needs native review")
# Each source expression was reviewed for this exact native binding. A source
# change requires a fresh semantic review, even if a familiar substring remains.
REVIEWED = {
    "s000009": ("native_transport", "b074ab1a52d08114aebcf0820d978c188bd8eb61bb1669d0f53667ab45575635"),
    "s000014": ("native_cancellation", "d58d066466c3d2a30c708e3f6c6619cd6a98aad27887dce979dd1b0e92a59a3e"),
    "s000017": ("nonblank", "d31dbe1d44f0103c65716dfd3033914a81f4a067ec83d5d5e012332c419b0438"),
    "s000035": ("chart_bounds", "6c5b9621d54e8372998bd0006c87360ca73242969ae266376e203c0711b8fd1b"),
    "s000041": ("finite_json_object", "4c523f70ebff9fb35129394909d0342c234a91a49bc40a35ea2415224fd551d7"),
    "s000063": ("chart_bounds", "ea6196e75bc93e9e2d6cac4a3f10062b3a39b4f3a14b6d8a751010bb98649402"),
    "s000065": ("chart_bounds", "ae5c228bf99b344a0d397fa9f9d2adc340b51d15b2862ae301fd57c120b1dcda"),
    "s000071": ("nonblank", "d93f8fc05df022189185bc726f4c172427f9001f5b1e334d32093d8a29a9460f"),
    "s000075": ("default", "16c58604cf23781d3e0bffe63b5b3aa6ecff96cf7c62799e3c405ad47a6cbc6b"),
    "s000080": ("default", "ecc10e35c1d19907684082abe44b43d2aab80375271b957fb6ee3a5bac9617b5"),
    "s000084": ("default", "6daf297b540fe1dfe372da3140e45fafacbb015f50463e1b6d0990947c2bc62c"),
    "s000086": ("default", "411fba098a35ec9962fa86f44af10088c18452b1dcef3d9ae4143b3aad9808ff"),
    "s000088": ("default", "1f32e32196703abce461fbd4522af0b93f3a50ba02eac0496583d8e98f110a8a"),
    "s000090": ("default", "88852f15a4b9c0206e63bd10e55d1d8a63f7d89b421d9e5f2918c0c6cdee6849"),
    "s000093": ("default", "a7bdcafefa9340ff237fc0e6cbebc0ae8a6112a21e149bd8dadf20f093c557b3"),
    "s000095": ("default", "9c658715f97b8d2684f4d673b8f2d7e2a9a2bb0005974c490ec469641dc3e9b0"),
    "s000196": ("time_order", "a390dda76eff6cf2cf0d53bc113df2acf80291de6208832b3cdd11de1de8dcbb"),
    "s000489": ("nonempty_object", "1a7737f0ab5a3cec7be06176d268008b2190c98fe962f473370a95adee3d5f8b"),
    "s000491": ("finite_json", "575abb7afa5716527ef20a3bf4e639f44c3085118eaa9e24e8def5891fb02355"),
    "s001616": ("secret_create", "1f806be8eaa1cede2eeb4705bb16903c12958d671c0a3dfb4d02099f99527f71"),
    "s001692": ("secret_update", "bcc715a9ec7c471c9ba9712312e83a2ff16f4d237d8bea320a6509a9b1bcc1c3"),
    "s002084": ("unwrap_data", "29fac2e4f1c20e97720d9e73a560c388a54cd60114e7a941bb462d857cfb1bb8"),
    "s004593": ("native_binary", "fd957658a213458b8d96441c576824b8fc3dbb260ff306e9e9a8e646ef050ad7"),
    "s004597": ("native_binary", "fd957658a213458b8d96441c576824b8fc3dbb260ff306e9e9a8e646ef050ad7"),
    "s004623": ("native_binary", "fd957658a213458b8d96441c576824b8fc3dbb260ff306e9e9a8e646ef050ad7"),
    "s004774": ("native_binary", "fd957658a213458b8d96441c576824b8fc3dbb260ff306e9e9a8e646ef050ad7"),
    "s004836": ("native_binary", "fd957658a213458b8d96441c576824b8fc3dbb260ff306e9e9a8e646ef050ad7"),
    "s004897": ("native_binary", "fd957658a213458b8d96441c576824b8fc3dbb260ff306e9e9a8e646ef050ad7"),
    "s006118": ("identity", "a9a0b5c81f2638ff287f4e9855c0f62a3261416d4ee73d4183f7061127dc35f9"),
    "s006119": ("realtime_type", "ab0ab4c3204685512fb17e85464b4bf0ab3948a08d47239004a45d18bd39ef44"),
    "s006129": ("usage_reset_exclusive", "a9c22a24306caa0484bd7cac1ace892fe8f3b5b908e0a5b594c8424fcf636e03"),
    "s006130": ("nonempty_object", "1a7737f0ab5a3cec7be06176d268008b2190c98fe962f473370a95adee3d5f8b"),
    "s006197": ("nonempty_object", "1a7737f0ab5a3cec7be06176d268008b2190c98fe962f473370a95adee3d5f8b"),
    "s006240": ("nonempty_object", "1a7737f0ab5a3cec7be06176d268008b2190c98fe962f473370a95adee3d5f8b"),
    "s006247": ("guardrail_checks", "d3e95169795d572d25bd38df605c1d2dcf1c2886c7c8ca2e2c9bcc5f811f27d8"),
    "s006256": ("nonempty_object", "1a7737f0ab5a3cec7be06176d268008b2190c98fe962f473370a95adee3d5f8b"),
    "s006274": ("nonempty_object", "1a7737f0ab5a3cec7be06176d268008b2190c98fe962f473370a95adee3d5f8b"),
    "s006289": ("rotation_exclusive", "fb1d36bca870f0e9e9c4df5a41e67e745300e382b2afdbf29826a99d617d6bc2"),
    "s006290": ("nonempty_object", "1a7737f0ab5a3cec7be06176d268008b2190c98fe962f473370a95adee3d5f8b"),
    "s006323": ("nonempty_object", "1a7737f0ab5a3cec7be06176d268008b2190c98fe962f473370a95adee3d5f8b"),
    "s006366": ("nonempty_object", "1a7737f0ab5a3cec7be06176d268008b2190c98fe962f473370a95adee3d5f8b"),
    "s006409": ("nonempty_object", "1a7737f0ab5a3cec7be06176d268008b2190c98fe962f473370a95adee3d5f8b"),
    "s006433": ("nonempty_object", "1a7737f0ab5a3cec7be06176d268008b2190c98fe962f473370a95adee3d5f8b"),
    "s006455": ("nonempty_object", "1a7737f0ab5a3cec7be06176d268008b2190c98fe962f473370a95adee3d5f8b"),
    "s006508": ("nonempty_object", "1a7737f0ab5a3cec7be06176d268008b2190c98fe962f473370a95adee3d5f8b"),
    "s006529": ("nonempty_object", "6df085089f53f173d22203270e3d86f98422f60517751d5569b3b89974c08e62"),
    "s006533": ("nonempty_object", "1a7737f0ab5a3cec7be06176d268008b2190c98fe962f473370a95adee3d5f8b"),
    "s006536": ("nonempty_object", "1a7737f0ab5a3cec7be06176d268008b2190c98fe962f473370a95adee3d5f8b"),
    "s007738": ("profile_selector", "42b823159659204ffafe32889e12f790817be93c302827b69ccbc5e9317ae51c"),
    "s007788": ("tool_event", "3c580721dabb039e0662d3eec2ad67282bdff619cd7be9a1c6c6a58866b3c1e0"),
    "s008858": ("message_object", "dbada94df53efb39cb0bf5a02b7a711e4231a9525e6cb76a0c5db566fb651970"),
    "s008873": ("default", "380268ce40f7475ed4aa9fdeaebb55d7cf7ff7158f68bb0443c408055283c801"),
    "s008908": ("default", "886d91c8309857f87c1919d379df798290afe23551494ad20d4e030a39df6473"),
    "s008913": ("message_object", "dbada94df53efb39cb0bf5a02b7a711e4231a9525e6cb76a0c5db566fb651970"),
    "s008972": ("message_object", "dbada94df53efb39cb0bf5a02b7a711e4231a9525e6cb76a0c5db566fb651970"),
    "s009653": ("default", "288da2b16b68d7a1aa7932acbad42d721fac22a917993ea86d911e6ca7a6f6c8"),
    "s009658": ("message_object", "dbada94df53efb39cb0bf5a02b7a711e4231a9525e6cb76a0c5db566fb651970"),
    "s009825": ("default", "d3f52f62bc83ee7c76be099d60c8a3e0f1b4b60db34eb367cfc07c0cb812f3bb"),
    "s009828": ("default", "d3f52f62bc83ee7c76be099d60c8a3e0f1b4b60db34eb367cfc07c0cb812f3bb"),
    "s009831": ("default", "d3f52f62bc83ee7c76be099d60c8a3e0f1b4b60db34eb367cfc07c0cb812f3bb"),
    "s010265": ("default", "70635f9259b19b46e2bd8f8663271d6b0fcdf07bed71e7a6e610a8e97f3e8cd4"),
    "s010574": ("default", "8fa9a1240b81f931ce85cfc2d55bdeb84ff64d07508e959f0efec80fc6ef0bdd"),
    "s010705": ("default", "bc2595ae1eaf3ca9f70d3db1796e6b4e35e77d931004046562e9ab03e01f31bf"),
    "s010711": ("default", "9068f5a0a243aef0444e00af430bb40c4889eb68feb25d67c25b92bf3e7c568d"),
    "s010723": ("default", "bd4391a4a8a30ecabdab128f4a8458edd0af2e62f240be27383ced4a3cac55c5"),
    "s010733": ("default", "d5340861291ceeb37e6b337426844cc88c4129dc0cd98d0d891bcd2207257729"),
    "s010743": ("default", "c9d22d7ca74fd6b3eb38ac7bbdaefe4948faeeb7d537ac9591434b4fa837f301"),
    "s010825": ("default", "0be51b5c215b85284b485f7d83dfb82b6008eca65b7fbf25b7f1d5e5174da5d2"),
    "s012814": ("default", "2ecda2ae376e1d12d5f8dad73a0e3b86bffade3134952bc5e0c49f70567d1742"),
}

entries = []
for node in source["schemas"]["manualSemantics"]:
    expr = node["originalExpression"]["source"]
    text = expr["text"]
    if hashlib.sha256(text.encode()).hexdigest() != expr["sha256"]:
        raise ValueError(f"Semantic source hash mismatch: {node['schema']}")
    binding = None
    default = None
    if node["kind"] == "ZodDefault":
        binding = "default"
        match = re.search(r"\.default\((\[\]|-?\d+(?:\.\d+)?|'[^']*'|\"[^\"]*\")\)\s*$", text)
        if not match:
            raise ValueError(f"Unrecognized default: {node['schema']}")
        default = json.loads(match[1].replace("'", '"'))
    elif "typeof globalThis.fetch" in text: binding = "native_transport"
    elif "instanceof AbortSignal" in text: binding = "native_cancellation"
    elif "instanceof(Blob)" in text: binding = "native_binary"
    elif node["kind"] == "effect:transform":
        if "value.data" in text: binding = "unwrap_data"
        elif "({ message })" in text: binding = "message_object"
        elif "as GatewayRealtimeEvent" in text: binding = "identity"
    elif "trim().length > 0" in text: binding = "nonblank"
    elif "validChartBounds" in text: binding = "chart_bounds"
    elif "Date.parse(v.start_time)" in text: binding = "time_order"
    elif "isFiniteJsonValue" in text: binding = "finite_json_object" if "!Array.isArray(value)" in text else "finite_json"
    elif "manager_type" in text and "Authentication configuration must match" in text: binding = "secret_create"
    elif "allow_all_workspaces" in text and "Provide at least one update field" in text: binding = "secret_update"
    elif "typeof value.type === 'string'" in text: binding = "realtime_type"
    elif "periodic_reset and periodic_reset_days" in text: binding = "usage_reset_exclusive"
    elif "LLM guardrails require checks" in text: binding = "guardrail_checks"
    elif "rotation_period and next_rotation_at" in text: binding = "rotation_exclusive"
    elif "d.profile_id || d.profile_name" in text: binding = "profile_selector"
    elif "value.input !== undefined || value.output !== undefined" in text: binding = "tool_event"
    elif "Object.keys(" in text and "length > 0" in text: binding = "nonempty_object"
    if not binding:
        raise ValueError(f"Unrecognized semantic {node['schema']}")
    if REVIEWED.get(node["schema"]) != (binding, expr["sha256"]):
        raise ValueError(f"Semantic source changed and needs native review: {node['schema']}")
    entry = {"schema": node["schema"], "sourceKind": node["kind"], "binding": binding,
             "source": {k: expr[k] for k in ["file", "line", "sha256"]},
             "implementation": "crates/scm-sdk/src/validation.rs",
             "test": "validation::tests::all_semantic_bindings_are_exercised",
             "domain": "typed_native_boundary" if binding.startswith("native_") else "json"}
    if binding == "default": entry["default"] = default
    entries.append(entry)
if len(entries) != 69 or len({entry["schema"] for entry in entries}) != 69:
    raise ValueError("Frozen semantic inventory changed")
payload = {"schemaVersion": 1, "baselineCommit": source["baseline"]["commit"],
           "sourceFingerprint": source["baseline"]["sourceFingerprint"],
           "generatorSha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
           "policy": "Every binding has concrete native behavior; registry generation alone is not test evidence. JSON fakes for typed runtime inputs are rejected.",
           "bindings": sorted(entries, key=lambda entry: entry["schema"])}
serialized = json.dumps(payload, indent=2, ensure_ascii=False) + "\n"
destination = ROOT / "contracts/native-semantics.json"
runtime = ROOT / "crates/scm-sdk/src/validation/semantics.json"
runtime_bytes = json.dumps({entry["schema"]: {k:entry[k] for k in ["binding", "default"] if k in entry} for entry in entries}, separators=(",", ":"), sort_keys=True) + "\n"
if "--check" in sys.argv:
    assert destination.read_text() == serialized, "native semantic registry is stale"
    assert runtime.read_text() == runtime_bytes, "runtime semantic registry is stale"
else:
    destination.write_text(serialized)
    runtime.parent.mkdir(parents=True,exist_ok=True)
    runtime.write_text(runtime_bytes)
print(f"Native semantic registry: {len(entries)} explicit bindings")
