#!/usr/bin/env python3
"""Generate native typed model contracts from the verified frozen schema inventory."""

import argparse
import hashlib
import json
import pathlib
import re
import shutil
import subprocess
import sys


ROOT = pathlib.Path(__file__).resolve().parents[1]
SOURCE = ROOT / "contracts/typescript.json"
OUT = ROOT / "crates/scm-sdk/src/models"
FAMILIES = {"airs::scan": "scan", "airs::management": "management", "airs::model_security": "model_security", "airs::red_team": "red_team", "airs::gateway": "gateway", "airs::agentguard": "agentguard", "dlp": "dlp", "shared::iam": "iam", "shared": "shared"}
KEYWORDS = set("as async await break const continue crate dyn else enum extern false fn for if impl in let loop match mod move mut pub ref return self Self static struct super trait true type unsafe use where while abstract become box do final macro override priv typeof unsized virtual yield try gen".split())
UNION_PINS = ROOT / "contracts/native-union-variants.json"
STRING_CHECKS = {"min", "max", "uuid", "email", "url", "datetime", "regex"}
NUMBER_CHECKS = {"int", "finite", "min", "max", "multipleOf"}
REGEX_SOURCES = {r"^[A-Za-z0-9][A-Za-z0-9_-]*$",r"^[^,\s]+__[^,\s]+$",r"^[a-zA-Z0-9_-]+$",r"^\d+$"}
TRANSFORM_MESSAGE = {"s008858", "s008913", "s008972", "s009658"}


def sha(data):
    return hashlib.sha256(data).hexdigest()


def quoted(value):
    return json.dumps(value, ensure_ascii=False)


def pascal(value):
    result = "".join(part[:1].upper() + part[1:] for part in re.split(r"[^A-Za-z0-9]+", value) if part)
    if not result or result[0].isdigit() or result in KEYWORDS:
        result = "Model" + result
    return result


def field_name(value):
    value = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", value)
    value = re.sub(r"[^A-Za-z0-9_]", "_", value).lower()
    if not value or value[0].isdigit() or value == "_":
        value = "field_" + value
    return "r#" + value if value in KEYWORDS and value not in {"self", "super", "crate", "Self"} else "field_" + value if value in KEYWORDS else value


SUPPORT = r'''//! Shared model containers with redacted ordinary formatting.
use crate::data::{JsonValue, FiniteNumber};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{collections::BTreeMap, fmt};

/// Additional wire fields. Ordinary formatting conceals both keys and values.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Extensions<T = JsonValue>(BTreeMap<String, T>);
impl<T> Default for Extensions<T> { fn default() -> Self { Self(BTreeMap::new()) } }
impl<T> Extensions<T> {
    pub fn new() -> Self { Self::default() }
    pub fn insert(&mut self, key: String, value: T) -> Option<T> { self.0.insert(key, value) }
    pub fn get(&self, key: &str) -> Option<&T> { self.0.get(key) }
    pub fn iter(&self) -> impl Iterator<Item = (&String, &T)> { self.0.iter() }
    pub fn keys(&self) -> impl Iterator<Item = &String> { self.0.keys() }
    pub fn is_empty(&self) -> bool { self.0.is_empty() }
    pub fn len(&self) -> usize { self.0.len() }
    pub fn into_map(self) -> BTreeMap<String,T> { self.0 }
}
impl<T> fmt::Debug for Extensions<T> { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str("Extensions([REDACTED])") } }

/// A typed dictionary; key/value access is explicit and ordinary formatting is redacted.
pub type TypedMap<T> = Extensions<T>;

/// A typed top-level sequence with safe ordinary formatting.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Sequence<T>(Vec<T>);
impl<T> Sequence<T> {
    pub fn new(values: Vec<T>) -> Self { Self(values) }
    pub fn as_slice(&self) -> &[T] { &self.0 }
    pub fn into_vec(self) -> Vec<T> { self.0 }
    pub fn iter(&self) -> std::slice::Iter<'_,T> { self.0.iter() }
}
impl<T> From<Vec<T>> for Sequence<T> { fn from(value: Vec<T>) -> Self { Self(value) } }
impl<T> fmt::Debug for Sequence<T> { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str("Sequence([REDACTED])") } }

/// A typed extension value or an explicitly unfamiliar wire value.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OpenValue<T> { Known(T), Unknown(JsonValue) }
impl<T> fmt::Debug for OpenValue<T> { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str("OpenValue([REDACTED])") } }

/// Suggested string vocabulary with preserved future string values.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OpenString<T> { Known(T), Unknown(String) }
impl<T> fmt::Debug for OpenString<T> { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str("OpenString([REDACTED])") } }

/// Validate and normalize a schema before decoding its native model.
pub fn parse<T: serde::de::DeserializeOwned>(schema: &str, value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<T> {
    let value = crate::validation::parse(schema, value, mode)?;
    serde_json::from_value(value).map_err(|_| crate::Error::new(crate::ErrorKind::Validation))
}

/// Convert a typed model within the standalone validator's default byte bound.
pub(super) fn wire_value<T: Serialize>(value: &T) -> crate::Result<serde_json::Value> {
    let bytes=crate::engine::serialize_bounded(value,crate::validation::ValidationLimits::default().max_input_bytes)?;
    serde_json::from_slice(&bytes).map_err(|_|crate::Error::new(crate::ErrorKind::Validation))
}

/// Merge compatible independently normalized intersection representations.
fn merge_intersection(left:serde_json::Value,right:serde_json::Value,depth:usize,work:&mut usize)->crate::Result<serde_json::Value>{
    use serde_json::Value;
    if depth>crate::validation::ValidationLimits::default().max_depth||*work==0{return Err(crate::Error::new(crate::ErrorKind::Limit));}
    *work-=1;
    match(left,right){
        (Value::Object(mut left),Value::Object(right))=>{
            for(key,right)in right{
                let value=if let Some(old)=left.remove(&key){merge_intersection(old,right,depth+1,work)?}else{right};
                left.insert(key,value);
            }
            Ok(Value::Object(left))
        }
        (Value::Array(left),Value::Array(right))if left.len()==right.len()=>Ok(Value::Array(left.into_iter().zip(right).map(|(left,right)|merge_intersection(left,right,depth+1,work)).collect::<crate::Result<_>>()?)),
        (left,right)if match(left.as_f64(),right.as_f64()){(Some(left),Some(right))=>left==right,_=>left==right}=>Ok(left),
        _=>Err(crate::Error::new(crate::ErrorKind::Validation)),
    }
}

fn bound_intersection_value(value:&serde_json::Value,limits:&crate::validation::ValidationLimits)->crate::Result<()>{
    use serde_json::Value;
    let mut pending=vec![(value,0usize)];let mut nodes=0usize;
    while let Some((value,depth))=pending.pop(){
        nodes+=1;
        if depth>limits.max_depth||nodes>limits.max_nodes{return Err(crate::Error::new(crate::ErrorKind::Limit));}
        match value{
            Value::Array(values)=>{
                if nodes.saturating_add(pending.len()).saturating_add(values.len())>limits.max_nodes{return Err(crate::Error::new(crate::ErrorKind::Limit));}
                pending.extend(values.iter().map(|value|(value,depth+1)));
            }
            Value::Object(values)=>{
                if nodes.saturating_add(pending.len()).saturating_add(values.len())>limits.max_nodes{return Err(crate::Error::new(crate::ErrorKind::Limit));}
                pending.extend(values.values().map(|value|(value,depth+1)));
            }
            Value::Number(value)if value.as_f64().is_none_or(|value|!value.is_finite())=>return Err(crate::Error::new(crate::ErrorKind::Validation)),
            _=>{},
        }
    }
    Ok(())
}
fn intersection_wire<A:Serialize,B:Serialize>(left:&A,right:&B)->crate::Result<serde_json::Value>{
    let left=wire_value(left)?;let right=wire_value(right)?;
    let limits=crate::validation::ValidationLimits::default();
    bound_intersection_value(&left,&limits)?;bound_intersection_value(&right,&limits)?;
    let mut work=limits.max_work;
    let merged=merge_intersection(left,right,0,&mut work)?;
    bound_intersection_value(&merged,&limits)?;
    // Independently bounded halves can merge into a larger object.
    wire_value(&merged)
}

/// A wire value satisfying both halves of an intersection. Compatible objects
/// merge fields recursively; arrays merge elementwise and must have equal length.
#[derive(Clone, PartialEq)]
pub struct Intersection<A,B> { left:A, right:B }
impl<A,B> Intersection<A,B> {
    pub fn left(&self)->&A {&self.left}
    pub fn right(&self)->&B {&self.right}
}
impl<A:Serialize,B:Serialize> Intersection<A,B> {
    pub fn new(left:A,right:B)->crate::Result<Self> {
        intersection_wire(&left,&right)?;
        Ok(Self{left,right})
    }
}
impl<A:Serialize,B:Serialize> Serialize for Intersection<A,B> {
    fn serialize<S:Serializer>(&self,s:S)->Result<S::Ok,S::Error>{
        intersection_wire(&self.left,&self.right).map_err(|_|serde::ser::Error::custom("intersection representations conflict or exceed limits"))?.serialize(s)
    }
}
impl<'de,A:serde::de::DeserializeOwned+Serialize,B:serde::de::DeserializeOwned+Serialize> Deserialize<'de> for Intersection<A,B>{
    fn deserialize<D:Deserializer<'de>>(d:D)->Result<Self,D::Error>{
        let value=serde_json::Value::deserialize(d)?;
        let value=wire_value(&value).map_err(|_|serde::de::Error::custom("intersection input exceeds limits"))?;
        bound_intersection_value(&value,&crate::validation::ValidationLimits::default()).map_err(|_|serde::de::Error::custom("intersection input exceeds limits"))?;
        let left=serde_json::from_value(value.clone()).map_err(|_|serde::de::Error::custom("left intersection invalid"))?;
        let right=serde_json::from_value(value).map_err(|_|serde::de::Error::custom("right intersection invalid"))?;
        Self::new(left,right).map_err(|_|serde::de::Error::custom("intersection representations conflict or exceed limits"))
    }
}
impl<A,B> fmt::Debug for Intersection<A,B>{fn fmt(&self,f:&mut fmt::Formatter<'_>)->fmt::Result{f.write_str("Intersection([REDACTED])")}}

/// A contract with no accepted JSON values.
#[derive(Clone,PartialEq,Serialize,Deserialize)]
pub enum Never {}
impl fmt::Debug for Never{fn fmt(&self,_:&mut fmt::Formatter<'_>)->fmt::Result{match *self{}}}

/// Owned binary input. JSON encoding is deliberately rejected; multipart adapters own the wire representation.
#[derive(Clone, PartialEq)]
pub struct BinaryFile { bytes: bytes::Bytes, filename: Option<String>, content_type: Option<String> }
impl BinaryFile {
    pub fn new(bytes: impl Into<bytes::Bytes>) -> crate::Result<Self> { Ok(Self { bytes: bytes.into(), filename: None, content_type: None }) }
    pub fn with_filename(mut self, name: impl Into<String>) -> crate::Result<Self> {
        let name=name.into();
        if name.is_empty()||name.len()>255||name=="."||name==".."||name.chars().any(|c|c.is_control()||matches!(c,'/'|'\\'|'"')) {return Err(crate::Error::new(crate::ErrorKind::Validation));}
        self.filename = Some(name); Ok(self)
    }
    pub fn with_content_type(mut self, content_type: impl Into<String>) -> crate::Result<Self> {
        let content_type=content_type.into();
        if content_type.len()>255||!content_type.is_ascii()||content_type.bytes().any(|c|c<32||c==127)||content_type.parse::<mime::Mime>().is_err(){return Err(crate::Error::new(crate::ErrorKind::Validation));}
        self.content_type = Some(content_type); Ok(self)
    }
    pub fn bytes(&self) -> &bytes::Bytes { &self.bytes }
    pub fn filename(&self) -> Option<&str> { self.filename.as_deref() }
    pub fn content_type(&self) -> Option<&str> { self.content_type.as_deref() }
    pub fn validate(&self,max_bytes:usize)->crate::Result<()> {if self.bytes.len()>max_bytes{Err(crate::Error::new(crate::ErrorKind::Limit))}else{Ok(())}}
}
impl fmt::Debug for BinaryFile { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str("BinaryFile([REDACTED])") } }
impl Serialize for BinaryFile { fn serialize<S: Serializer>(&self, _: S) -> Result<S::Ok,S::Error> { Err(serde::ser::Error::custom("binary input requires a multipart adapter")) } }
impl<'de> Deserialize<'de> for BinaryFile { fn deserialize<D: Deserializer<'de>>(_: D) -> Result<Self,D::Error> { Err(serde::de::Error::custom("binary input is not a JSON value")) } }

/// An arbitrary finite JSON number's model alias.
pub type Number = FiniteNumber;
'''


class Generator:
    def __init__(self, inventory):
        self.inventory = inventory
        self.nodes = dict(inventory["schemas"]["definitions"])
        self.exports = inventory["exports"]
        self.schemas = {entry["name"]: entry for entry in self.exports if entry["kind"] == "schema"}
        self.roots = {entry["name"]: entry["schema"]["$ref"] for entry in self.schemas.values()}
        self.names = {}
        self.used = set()
        self.declarations = {}
        self.dependencies = {}
        self.aliases = {}
        self.family_exports = {family: set() for family in FAMILIES.values()}
        self.schema_models = []
        self.model_rows = []
        self.union_rows = []
        # Root schemas receive stable public names before nested names are allocated.
        for name, entry in self.schemas.items():
            native = pascal(name.removesuffix("Schema"))
            sid = entry["schema"]["$ref"]
            if sid not in self.names:
                self.names[sid] = native
                self.used.add(native)
            else:
                self.aliases[native] = self.names[sid]
                self.used.add(native)
            self.family_exports[FAMILIES[entry["classification"]["namespace"]]].add(native)
            self.schema_models.append({"export": name, "schema": sid, "rustType": native, "namespace": entry["classification"]["namespace"], "status": "typed_model", "validator": "crate::validation::parse"})

    def allocate(self, hint):
        name = pascal(hint)
        if name not in self.used:
            self.used.add(name)
            return name
        number = 2
        while f"{name}{number}" in self.used:
            number += 1
        name += str(number)
        self.used.add(name)
        return name

    def deref(self, ref):
        return ref["$ref"]

    def normalize(self, sid):
        seen = set()
        while sid not in seen:
            seen.add(sid)
            node = self.nodes[sid]
            kind = node["kind"]
            if sid in TRANSFORM_MESSAGE:
                return {"kind": "ZodObject", "properties": {"message": {"schema": {"$ref": self.string_sid()}}}, "unknownFields": "passthrough"}, sid
            if sid == "s006118":
                return {"kind":"ZodObject","properties":{"type":{"schema":{"$ref":self.string_sid()}}},"unknownFields":"passthrough"}, sid
            if sid == "s000041":
                return {"kind":"NativeJsonObject"}, sid
            if sid == "s002084":
                inner = self.nodes[self.deref(node["inner"])]
                sid = self.deref(inner["properties"]["data"]["schema"])
                continue
            if kind in {"ZodEffects", "ZodDefault", "ZodReadonly", "ZodBranded"}:
                source = node.get("semanticSource", {}).get("source", {}).get("text", "")
                if "z.instanceof(Blob)" in source:
                    return {"kind": "NativeBinary"}, sid
                sid = self.deref(node["inner"])
            elif kind == "ZodLazy":
                sid = self.deref(node["target"])
            elif kind == "ZodPipeline":
                sid = self.deref(node["output"])
            else:
                return node, sid
        raise ValueError(f"Unresolvable schema wrapper cycle {sid}")

    def string_sid(self):
        return next(key for key, value in self.nodes.items() if value["kind"] == "ZodString" and not value.get("checks"))

    def presence(self, sid, seen=None):
        seen = set() if seen is None else set(seen)
        if sid in seen:
            return False, False
        seen.add(sid)
        node = self.nodes[sid]
        kind = node["kind"]
        if sid in {"s000041","s006118","s000009","s000014"} or "z.instanceof(Blob)" in node.get("semanticSource",{}).get("source",{}).get("text",""):
            return False,False
        if sid=="s000491":
            return False,True
        if kind in {"ZodOptional", "ZodNullable", "ZodDefault"}:
            optional, nullable = self.presence(self.deref(node["inner"]), seen)
            return optional or kind in {"ZodOptional", "ZodDefault"}, nullable or kind == "ZodNullable"
        if kind in {"ZodEffects", "ZodReadonly", "ZodBranded"}:
            if sid in TRANSFORM_MESSAGE:
                return False, False
            return self.presence(self.deref(node["inner"]), seen)
        if kind == "ZodLazy":
            return self.presence(self.deref(node["target"]), seen)
        if kind == "ZodPipeline":
            return self.presence(self.deref(node["output"]), seen)
        if kind in {"ZodAny", "ZodUnknown"}:
            return True, True
        if kind == "ZodNull":
            return False, True
        if kind in {"ZodUnion", "ZodDiscriminatedUnion"}:
            states = [self.presence(self.deref(ref), seen) for ref in node["variants"]]
            return any(state[0] for state in states), any(state[1] for state in states)
        return False, False

    def strip_presence(self, sid):
        node = self.nodes[sid]
        if node["kind"] in {"ZodOptional", "ZodNullable", "ZodDefault"}:
            return self.strip_presence(self.deref(node["inner"]))
        if node["kind"] == "ZodUnion":
            variants = [ref for ref in node["variants"] if self.normalize(self.deref(ref))[0]["kind"] != "ZodNull"]
            if len(variants) == 1:
                return self.strip_presence(self.deref(variants[0]))
        return sid

    def rust_type(self, sid, hint, root=False):
        original = sid
        node, normalized = self.normalize(sid)
        kind = node["kind"]
        if not root and original in self.names:
            if self.names[original] in self.declarations and not self.declarations[self.names[original]]:
                return f"Box<{self.names[original]}>"
            self.emit(original, self.names[original])
            return self.names[original]
        if kind in {"ZodOptional", "ZodNullable"}:
            inner = self.rust_type(self.deref(node["inner"]), hint + "Value")
            return f"{'Optional' if kind == 'ZodOptional' else 'Nullable'}<{inner}>"
        primitives = {"ZodString": "String", "ZodNumber": "FiniteNumber", "ZodBoolean": "bool", "ZodAny": "JsonValue", "ZodUnknown": "JsonValue", "ZodNull": "()", "ZodNever": "Never", "ZodDate": "crate::validation::NativeDate", "NativeBinary": "BinaryFile", "NativeJsonObject":"JsonObject", "NativeJsonValue":"JsonValue"}
        if kind in primitives:
            return primitives[kind]
        if kind == "ZodArray":
            element = self.rust_type(self.deref(node["element"]), hint + "Item")
            if element.startswith("Box<"):element=element[4:-1]
            return f"Vec<{element}>"
        if kind == "ZodRecord":
            value = self.rust_type(self.deref(node["value"]), hint + "Value")
            if value.startswith("Box<"):value=value[4:-1]
            return f"TypedMap<{value}>"
        if normalized in self.names and not root:
            self.emit(normalized, self.names[normalized])
            return self.names[normalized]
        if not root:
            name = self.allocate(hint)
            self.names[original] = name
            self.emit(original, name)
            return name
        raise ValueError(f"Unhandled root type {original}:{kind}")

    def enum(self, name, values, open_enum=True):
        if not all(isinstance(value, str) for value in values):
            raise ValueError(f"Non-string enum needs an explicit native adapter: {name}")
        variants = []
        seen = set()
        for value in dict.fromkeys(values):
            variant = pascal(value)
            if variant == "Unknown" or variant in seen:
                variant += "Value"
            while variant in seen:
                variant += "Value"
            seen.add(variant)
            variants.append((variant, value))
        body = f"#[derive(Clone, PartialEq, Eq)]\n#[non_exhaustive]\npub enum {name} {{" + ",".join(variant for variant, _ in variants) + (",Unknown(String)}\n" if open_enum else "}\n")
        body += f"impl {name} {{pub fn as_str(&self)->&str {{match self {{" + "".join(f"Self::{variant}=>{quoted(value)}," for variant, value in variants) + ("Self::Unknown(value)=>value" if open_enum else "") + "}}}\n"
        body += f"impl Serialize for {name} {{fn serialize<S:serde::Serializer>(&self,s:S)->std::result::Result<S::Ok,S::Error>{{s.serialize_str(self.as_str())}}}}\n"
        body += f"impl<'de> Deserialize<'de> for {name} {{fn deserialize<D:serde::Deserializer<'de>>(d:D)->std::result::Result<Self,D::Error>{{let value=String::deserialize(d)?;" + ("Ok(match value.as_str(){" if open_enum else "match value.as_str(){") + "".join(f"{quoted(value)}=>" + (f"Self::{variant}," if open_enum else f"Ok(Self::{variant}),") for variant, value in variants) + ("_=>Self::Unknown(value)})}}\n" if open_enum else '_=>Err(serde::de::Error::custom("literal discriminator mismatch"))}}}\n')
        return body

    def literal(self,name,value):
        if isinstance(value,str):return self.enum(name,[value],open_enum=False)
        typed="bool" if isinstance(value,bool) else "FiniteNumber"
        condition=f"value=={str(value).lower()}" if isinstance(value,bool) else f"value.as_f64()==Some({float(value)!r})"
        return f"#[derive(Clone,PartialEq,Serialize)]\n#[serde(transparent)]\npub struct {name}({typed});\nimpl {name}{{pub fn new(value:{typed})->crate::Result<Self>{{if {condition}{{Ok(Self(value))}}else{{Err(crate::Error::new(crate::ErrorKind::Validation))}}}}pub fn as_inner(&self)->&{typed}{{&self.0}}pub fn into_inner(self)->{typed}{{self.0}}}}\nimpl<'de> Deserialize<'de> for {name}{{fn deserialize<D:serde::Deserializer<'de>>(d:D)->std::result::Result<Self,D::Error>{{let value={typed}::deserialize(d)?;Self::new(value).map_err(|_|serde::de::Error::custom(\"literal value mismatch\"))}}}}\n"

    def number(self,name,sid):
        validation=f"crate::validation::parse({quoted(sid)},serde_json::to_value(&value).map_err(|_|crate::Error::new(crate::ErrorKind::Validation))?,crate::validation::Mode::Response)?;" if sid.startswith("s") else ""
        return f"#[derive(Clone,PartialEq,Serialize)]\n#[serde(transparent)]\npub struct {name}(FiniteNumber);\nimpl {name}{{pub fn new(value:FiniteNumber)->crate::Result<Self>{{{validation}Ok(Self(value))}}pub fn as_inner(&self)->&FiniteNumber{{&self.0}}pub fn into_inner(self)->FiniteNumber{{self.0}}}}\nimpl<'de> Deserialize<'de> for {name}{{fn deserialize<D:serde::Deserializer<'de>>(d:D)->std::result::Result<Self,D::Error>{{let value=FiniteNumber::deserialize(d)?;Self::new(value).map_err(|_|serde::de::Error::custom(\"number contract mismatch\"))}}}}\n"

    def object_fields(self, node):
        if node["kind"] == "ZodIntersection":
            left, _ = self.normalize(self.deref(node["left"]))
            right, _ = self.normalize(self.deref(node["right"]))
            if left["kind"] != "ZodObject" or right["kind"] != "ZodObject":
                return None
            return {**self.object_fields(left), **self.object_fields(right)}
        return node.get("properties", {})

    def object(self, name, sid, node):
        fields = []
        used = {"extensions"}
        properties = self.object_fields(node)
        if properties is None:
            return None
        catchall = node.get("catchall")
        extension_type = "JsonValue"
        if catchall and self.normalize(self.deref(catchall))[0]["kind"] not in {"ZodNever","ZodAny","ZodUnknown"}:
            extension_type = f"OpenValue<{self.rust_type(self.deref(catchall),name+'AdditionalValue')}>"
        for wire, entry in properties.items():
            field = field_name(wire)
            while field in used:
                field += "_value"
            used.add(field)
            child = self.deref(entry["schema"])
            optional, nullable = self.presence(child)
            stripped = self.strip_presence(child)
            base = self.rust_type(stripped, name + pascal(wire))
            # Box direct recursive references; containers already provide indirection.
            if base == name:
                base = f"Box<{base}>"
            typed = f"{'Patch' if nullable else 'Optional'}<{base}>" if optional else f"Nullable<{base}>" if nullable else base
            fields.append((wire, field, typed, optional))
        body = f"#[derive(Clone, PartialEq, Deserialize)]\n#[non_exhaustive]\npub struct {name} {{\n"
        for wire, field, typed, optional in fields:
            body += f"#[serde(rename={quoted(wire)}" + (",default" if optional else "") + f")]\npub {field}:{typed},\n"
        body += f"#[serde(flatten)]\npub extensions:Extensions<{extension_type}>,\n}}\n"
        body += f"impl Serialize for {name} {{fn serialize<S:serde::Serializer>(&self,serializer:S)->std::result::Result<S::Ok,S::Error>{{\n"
        body += "const KNOWN:&[&str]=&[" + ",".join(quoted(wire) for wire, _, _, _ in fields) + "];\n"
        body += "if self.extensions.keys().any(|key|KNOWN.contains(&key.as_str())){return Err(serde::ser::Error::custom(\"extension collides with a known model field\"));}\nlet mut map=serializer.serialize_map(None)?;\n"
        for wire, field, _, optional in fields:
            body += (f"if !self.{field}.is_unset(){{" if optional else "") + f"map.serialize_entry({quoted(wire)},&self.{field})?;" + ("}" if optional else "") + "\n"
        body += "for(key,value)in self.extensions.iter(){map.serialize_entry(key,value)?;}map.end()}}\n"
        builder = name + "Builder"
        self.used.add(builder)
        body += f"impl {name} {{pub fn builder()->{builder}{{{builder}::default()}}\n"
        if sid.startswith("s"):
            body += f"pub const SCHEMA:&'static str={quoted(sid)};\npub fn parse(value:serde_json::Value,mode:crate::validation::Mode)->crate::Result<Self>{{let value=crate::validation::parse(Self::SCHEMA,value,mode)?;serde_json::from_value(value).map_err(|_|crate::Error::new(crate::ErrorKind::Validation))}}}}\n"
        else:
            body += "}\n"
        body += f"#[derive(Default)]\npub struct {builder} {{" + "".join(f"{field}:" + (typed if optional else f"Option<{typed}>") + "," for _, field, typed, optional in fields) + f"extensions:Extensions<{extension_type}>}}\n"
        body += f"impl {builder}{{\n"
        for _, field, typed, optional in fields:
            setter = "with_" + field.removeprefix("r#") if field in {"build", "extension", "default"} else field
            body += f"pub fn {setter}(mut self,value:{typed})->Self{{self.{field}=" + ("value" if optional else "Some(value)") + ";self}\n"
        body += f"pub fn extension(mut self,key:impl Into<String>,value:{extension_type})->Self{{self.extensions.insert(key.into(),value);self}}\n"
        body += f"pub fn build(self)->crate::Result<{name}>{{Ok({name}{{" + "".join(f"{field}:self.{field}" + ("" if optional else ".ok_or_else(||crate::Error::new(crate::ErrorKind::Validation))?") + "," for _, field, _, optional in fields) + "extensions:self.extensions})}}\n"
        if sid.startswith("s"):
            body = body.removesuffix("}\n") + f"pub fn build_validated(self,mode:crate::validation::Mode)->crate::Result<{name}>{{let model=self.build()?;let value=super::support::wire_value(&model)?;{name}::parse(value,mode)}}}}\n"
        body += self.debug(builder)
        return body

    def debug(self, name):
        return f"impl fmt::Debug for {name}{{fn fmt(&self,f:&mut fmt::Formatter<'_>)->fmt::Result{{f.write_str({quoted(name+'([REDACTED])')})}}}}\n"

    def emit(self, sid, name):
        if name in self.declarations:
            return
        self.declarations[name] = ""  # Break recursive graph construction.
        node, normalized = self.normalize(sid)
        kind = node["kind"]
        if kind in {"ZodObject", "ZodIntersection"}:
            body = self.object(name, sid, node)
            if body is None:
                left=self.rust_type(self.deref(node["left"]),name+"Left")
                right=self.rust_type(self.deref(node["right"]),name+"Right")
                body=f"#[derive(Clone,PartialEq,Serialize,Deserialize)]\n#[serde(transparent)]\npub struct {name}(Intersection<{left},{right}>);\nimpl {name}{{pub fn new(left:{left},right:{right})->crate::Result<Self>{{Intersection::new(left,right).map(Self)}}pub fn left(&self)->&{left}{{self.0.left()}}pub fn right(&self)->&{right}{{self.0.right()}}}}\n"
        elif kind in {"ZodEnum", "ZodNativeEnum"}:
            body = self.enum(name, node["values"])
        elif kind == "ZodLiteral":
            body = self.literal(name, node["value"])
        elif kind == "ZodNumber":
            body = self.number(name, sid)
        elif kind in {"ZodUnion", "ZodDiscriminatedUnion"}:
            variants = node["variants"]
            strings = []
            for ref in variants:
                child, _ = self.normalize(self.deref(ref))
                if child["kind"] == "ZodLiteral" and isinstance(child["value"], str):
                    strings.append(child["value"])
            body = self.enum(name, strings) if len(strings) == len(variants) else self.union(name, variants, sid)
        else:
            target = self.rust_type(sid, name + "Value", root=True)
            if target.startswith("Vec<"):
                target = "Sequence<" + target[4:]
            body = f"#[derive(Clone, PartialEq, Serialize, Deserialize)]\n#[serde(transparent)]\npub struct {name}({target});\nimpl {name}{{pub fn new(value:{target})->Self{{Self(value)}}pub fn as_inner(&self)->&{target}{{&self.0}}pub fn into_inner(self)->{target}{{self.0}}}}\n"
        self.declarations[name] = f"/// Native typed contract for frozen schema `{sid}`. Validate wire constraints with `validation::parse`.\n" + body + self.debug(name)

    def union(self, name, variants, schema):
        types = []
        branches = []
        for index, ref in enumerate(variants):
            sid=self.deref(ref)
            node,_=self.normalize(sid)
            if node["kind"]=="ZodNumber":
                if sid not in self.names:self.names[sid]=self.allocate(name+f"Variant{index+1}")
                self.emit(sid,self.names[sid])
                typed=self.names[sid]
            else:
                typed = self.rust_type(sid, name + f"Variant{index + 1}")
            if typed == name:
                typed = f"Box<{typed}>"
            if typed not in types:
                types.append(typed)
            branches.append({"sourceIndex":index+1,"schema":sid,"rustVariant":"Variant"+str(types.index(typed)+1),"rustType":typed})
        self.union_rows.append({"rustType":name,"schema":schema,"branches":branches})
        return f"#[derive(Clone, PartialEq, Serialize, Deserialize)]\n#[serde(untagged)]\n#[non_exhaustive]\npub enum {name}{{" + "".join(f"Variant{index+1}({typed})," for index, typed in enumerate(types)) + "Unknown(JsonValue)}\n"

    def generate(self, only=None):
        selected = [entry for entry in self.schema_models if only is None or FAMILIES[entry["namespace"]] in {only, "shared"}]
        for entry in selected:
            name = self.names[entry["schema"]]
            self.emit(entry["schema"], name)
        if only is None:
            self.extra_models()
            self.external_models()
        # All dependencies are public through family exports, while canonical
        # declarations remain generated once and share structural identity.
        all_names = sorted(set(self.declarations) | set(self.aliases))
        body = "//! Generated native models. Regenerate with tools/generate_models.py.\n#![allow(clippy::large_enum_variant)]\nuse super::support::*;\nuse crate::data::{Optional,Nullable,Patch,FiniteNumber,JsonValue,JsonObject};\nuse serde::{Serialize,Deserialize,ser::SerializeMap};\nuse std::fmt;\n"
        body += "\n".join(self.declarations[name] for name in sorted(self.declarations))
        for name, target in sorted(self.aliases.items()):
            if target in self.declarations:
                body += f"/// Capability-preserving alias of the canonical native model.\npub type {name}={target};\n"
        outputs = {OUT / "generated.rs": body, OUT / "support.rs": SUPPORT}
        module = "//! Native typed product and shared model namespaces.\nmod generated;\nmod support;\npub use support::BinaryFile;\n#[cfg(test)]\nmod tests;\n"
        visible_names=set(self.declarations)|set(self.aliases)
        for declaration in self.declarations.values():
            visible_names.update(re.findall(r"pub struct (\w+Builder)\b",declaration))
        dependency_map={name:set(re.findall(r"\b[A-Za-z_][A-Za-z0-9_]*\b",content))&visible_names for name,content in self.declarations.items()}
        dependency_map.update({name:{target} for name,target in self.aliases.items()})
        for family in FAMILIES.values():
            module += f"pub mod {family};\n"
            pending=list(self.family_exports[family]&visible_names)
            reachable=set()
            while pending:
                name=pending.pop()
                if name in reachable:continue
                reachable.add(name)
                pending.extend(dependency_map.get(name,set())-reachable)
            outputs[OUT / f"{family}.rs"] = "//! Native model contracts and their reachable dependencies; client availability is recorded separately.\n"+("pub use super::generated::{"+",".join(sorted(reachable))+"};\n" if reachable else "")+"pub use super::support::*;\npub use crate::data::{Optional,Nullable,Patch,FiniteNumber,JsonValue,JsonObject};\n"
        outputs[OUT / "mod.rs"] = module
        return outputs

    def extra_models(self):
        type_targets={}
        for entry in self.exports:
            if entry.get("type") and entry["name"] in self.used:
                type_targets.setdefault(entry["type"]["$ref"],entry["name"])
        for entry in self.exports:
            if not entry["source"]["file"].startswith("src/models/") or entry["kind"] not in {"type_alias", "interface"}:
                continue
            name = entry["name"]
            self.family_exports[FAMILIES[entry["classification"]["namespace"]]].add(name)
            if name in self.used:
                self.model_rows.append({"export": name, "rustType": name, "status": "typed_model"})
                continue
            declarations = "\n".join(source["text"] for source in entry["declarations"])
            references = re.findall(r"typeof\s+(\w+Schema)\b", declarations)
            direct_alias=re.search(r"export\s+type\s+"+re.escape(name)+r"\s*=\s*(\w+)\s*;",declarations)
            target_by_type=type_targets.get(entry.get("type",{}).get("$ref"))
            if references and references[-1] in self.roots:
                target = self.names[self.roots[references[-1]]]
                self.aliases[name] = target
                self.used.add(name)
                self.model_rows.append({"export": name, "rustType": name, "status": "typed_alias", "target": target})
            elif target_by_type and target_by_type!=name and name.endswith("Type") and entry["source"]["file"] in {"src/models/enums.ts","src/models/red-team-network-broker.ts"}:
                self.aliases[name]=target_by_type
                self.used.add(name)
                self.model_rows.append({"export":name,"rustType":name,"status":"typed_alias","target":target_by_type})
            elif direct_alias and direct_alias[1] in self.used:
                target=direct_alias[1]
                self.aliases[name]=target
                self.used.add(name)
                self.model_rows.append({"export":name,"rustType":name,"status":"typed_alias","target":target})
            elif entry.get("value") and isinstance(entry["value"], dict) and all(isinstance(value, str) for value in entry["value"].values()):
                self.used.add(name)
                self.declarations[name] = self.enum(name, list(entry["value"].values())) + self.debug(name)
                type_targets.setdefault(entry["type"]["$ref"],name)
                self.model_rows.append({"export": name, "rustType": name, "status": "typed_open_enum"})
            else:
                node=self.inventory["types"]["definitions"][entry["type"]["$ref"]]
                variants=[self.inventory["types"]["definitions"][ref["$ref"]] for ref in node.get("variants",[])]
                if variants and all(variant["kind"]=="literal" and isinstance(variant.get("value"),str) for variant in variants):
                    self.used.add(name)
                    self.declarations[name]=self.enum(name,[variant["value"] for variant in variants])+self.debug(name)
                    self.model_rows.append({"export":name,"rustType":name,"status":"typed_open_enum"})
                elif name=="GatewayIntegrationConfiguration":
                    targets=re.findall(r"\|\s*(\w+)",declarations)
                    if not targets or not all(target in self.used for target in targets):raise ValueError("Integration configuration union references missing typed models")
                    self.used.add(name)
                    self.declarations[name]=f"#[derive(Clone,PartialEq,Serialize,Deserialize)]\n#[serde(untagged)]\n#[non_exhaustive]\npub enum {name}{{"+"".join(f"Variant{i+1}({target})," for i,target in enumerate(targets))+"Unknown(JsonValue)}\n"+self.debug(name)
                    self.union_rows.append({"rustType":name,"sourceType":entry["type"]["$ref"],"branches":[{"sourceIndex":index+1,"sourceType":next(row["type"]["$ref"] for row in self.exports if row["name"]==target and row.get("type")),"rustVariant":"Variant"+str(index+1),"rustType":target} for index,target in enumerate(targets)]})
                    self.model_rows.append({"export":name,"rustType":name,"status":"typed_union"})
                elif name in {"JsonNullable","GatewayOpenValue"}:
                    target="Patch<T>" if name=="JsonNullable" else "OpenString<T>"
                    self.used.add(name)
                    self.declarations[name]=f"/// Native generic presence/open-string equivalent.\npub type {name}<T>={target};\n"
                    self.model_rows.append({"export":name,"rustType":name+"<T>","status":"typed_generic"})
                elif name=="Page":
                    self.used.add(name)
                    fields=[("empty","bool"),("first","bool"),("last","bool"),("number","FiniteNumber"),("numberOfElements","FiniteNumber"),("pageable","PageableObject"),("size","FiniteNumber"),("sort","SortObject"),("totalElements","FiniteNumber"),("totalPages","FiniteNumber")]
                    body="#[derive(Clone,PartialEq,Deserialize)]\n#[non_exhaustive]\npub struct Page<T>{pub content:Vec<T>,"+"".join(f"#[serde(rename={quoted(wire)},default)]pub {field_name(wire)}:Optional<{typed}>," for wire,typed in fields)+"#[serde(flatten)]pub extensions:Extensions<JsonValue>}\n"
                    body+="impl<T> Page<T>{pub fn new(content:Vec<T>)->Self{Self{content,"+"".join(f"{field_name(wire)}:Optional::Unset," for wire,_ in fields)+"extensions:Extensions::default()}}}\n"
                    body+="impl<T:Serialize> Serialize for Page<T>{fn serialize<S:serde::Serializer>(&self,s:S)->std::result::Result<S::Ok,S::Error>{const KNOWN:&[&str]=&[\"content\","+",".join(quoted(wire) for wire,_ in fields)+"];if self.extensions.keys().any(|key|KNOWN.contains(&key.as_str())){return Err(serde::ser::Error::custom(\"extension collides with a known model field\"));}let mut map=s.serialize_map(None)?;map.serialize_entry(\"content\",&self.content)?;"+"".join(f"if !self.{field_name(wire)}.is_unset(){{map.serialize_entry({quoted(wire)},&self.{field_name(wire)})?;}}" for wire,_ in fields)+"for(key,value)in self.extensions.iter(){map.serialize_entry(key,value)?;}map.end()}}\n"
                    body+="impl<T> fmt::Debug for Page<T>{fn fmt(&self,f:&mut fmt::Formatter<'_>)->fmt::Result{f.write_str(\"Page([REDACTED])\")}}\n"
                    self.declarations[name]=body
                    self.model_rows.append({"export":name,"rustType":"Page<T>","status":"typed_generic"})
                else:
                    self.model_rows.append({"export": name, "status": "pending_explicit_type_graph_adapter", "sourceType": entry.get("type"), "source": entry["source"]["file"]})

    def source_json_node(self, tid, known):
        if tid in known:
            return known[tid]
        sid="ts_"+tid
        if sid in self.nodes:return sid
        original=self.inventory["types"]["definitions"][tid]
        kind=original["kind"]
        self.nodes[sid]={"kind":"ZodNever"}
        ref=lambda child:{"$ref":self.source_json_node(child["$ref"],known)}
        if kind in {"string","number","null","never"}:
            node={"kind":{"string":"ZodString","number":"ZodNumber","null":"ZodNull","never":"ZodNever"}[kind]}
        elif kind in {"unknown","any"}:
            node={"kind":"ZodNullable","inner":{"$ref":"ts_json_unknown"}}
            self.nodes["ts_json_unknown"]={"kind":"NativeJsonValue"}
        elif kind=="literal":node={"kind":"ZodLiteral","value":original["value"]}
        elif kind=="array":node={"kind":"ZodArray","element":ref(original["element"])}
        elif kind=="union":
            variants=original["variants"]
            undefined=any(self.inventory["types"]["definitions"][v["$ref"]]["kind"]=="undefined" for v in variants)
            variants=[v for v in variants if self.inventory["types"]["definitions"][v["$ref"]]["kind"]!="undefined"]
            literals=[self.inventory["types"]["definitions"][v["$ref"]] for v in variants]
            if len(variants)==2 and {n.get("value") for n in literals}=={True,False} and all(n["kind"]=="literal" for n in literals):node={"kind":"ZodBoolean"}
            elif len(variants)==1:node=self.nodes[self.source_json_node(variants[0]["$ref"],known)]
            else:node={"kind":"ZodUnion","variants":[ref(v) for v in variants]}
            if undefined:
                self.nodes[sid+"_inner"]=node
                node={"kind":"ZodOptional","inner":{"$ref":sid+"_inner"}}
        elif kind=="object":
            if original.get("calls"):raise ValueError("Callable is not a JSON data model")
            properties={}
            for field in original["properties"]:
                child=ref(field["type"])
                if field["optional"]:
                    wrapper=sid+"_optional_"+field["name"]
                    self.nodes[wrapper]={"kind":"ZodOptional","inner":child}
                    child={"$ref":wrapper}
                properties[field["name"]]={"schema":child}
            node={"kind":"ZodObject","properties":properties,"unknownFields":"passthrough"}
            if original.get("indexSignatures"):
                raise ValueError("Source-only indexed object needs an explicit adapter")
        elif kind=="intersection":
            properties={}
            for variant in original["variants"]:
                child=self.nodes[self.source_json_node(variant["$ref"],known)]
                if child["kind"]!="ZodObject":raise ValueError("Source-only nonobject intersection needs an explicit adapter")
                properties.update(child["properties"])
            node={"kind":"ZodObject","properties":properties,"unknownFields":"passthrough"}
        else:raise ValueError(f"Source-only JSON model cannot contain {kind}: {original.get('display')}")
        self.nodes[sid]=node
        return sid

    def external_models(self):
        plain={"GatewayWorkspaceProvisionRequest","GatewayWorkspaceProvisionResult","GatewayDottedValueEntry","GatewaySecretFieldRule","GatewaySecretOperationMetadata","IamScopeCreateInput","IamScopeUpdateInput","TokenInfo"}
        known={}
        for entry in self.exports:
            if entry["kind"] in {"type_alias","interface"} and entry["name"] in self.names.values():
                node=self.inventory["types"]["definitions"][entry["type"]["$ref"]]
                if node["kind"] not in {"any","unknown","string","number","literal"}:
                    sid=next(key for key,value in self.names.items() if value==entry["name"])
                    known.setdefault(entry["type"]["$ref"],sid)
        for entry in self.exports:
            if entry["kind"] not in {"type_alias","interface"} or entry["source"]["file"].startswith("src/models/"):continue
            name=entry["name"]
            declarations="\n".join(row["text"] for row in entry["declarations"])
            direct=re.search(r"export\s+type\s+"+re.escape(name)+r"\s*=\s*(\w+)\s*;",declarations)
            inferred=re.findall(r"typeof\s+(\w+Schema)\b",declarations)
            target=self.names[self.roots[inferred[-1]]] if inferred and inferred[-1] in self.roots else direct[1] if direct and direct[1] in self.used else None
            row={"export":name,"source":entry["source"]["file"],"sourceType":entry["type"],"namespace":entry["classification"]["namespace"]}
            family=FAMILIES[row["namespace"]]
            if target:
                if name!=target:self.aliases[name]=target
                self.used.add(name);self.family_exports[family].add(name)
                row.update(status="typed_alias",rustType=name,target=target)
            elif name in plain:
                sid=self.source_json_node(entry["type"]["$ref"],known)
                self.names[sid]=name;self.used.add(name);known[entry["type"]["$ref"]]=sid
                self.emit(sid,name);self.family_exports[family].add(name)
                row.update(status="typed_source_model",rustType=name,validation="source-only type; owning client validates its operation contract")
            elif family=="dlp":
                mapping={
                    "DlpNamespaceOptions":"crate::dlp::DlpClient","DlpProfilesClientOptions":"crate::Client",
                    "DataPatternsClientOptions":"crate::Client","DataProfilesClientOptions":"crate::Client","DictionariesClientOptions":"crate::Client","DataFilteringProfilesClientOptions":"crate::Client",
                    "DataPatternListParams":"crate::dlp::DataPatternListParams","DataProfileListParams":"crate::dlp::DataProfileListParams","DictionaryListParams":"crate::dlp::DictionaryListParams","DataFilteringProfileListParams":"crate::dlp::DataFilteringProfileListParams",
                    "DataPatternListAllParams":"crate::dlp::DataPatternListParams","DataProfileListAllParams":"crate::dlp::DataProfileListParams","DictionaryListAllParams":"crate::dlp::DictionaryListParams","DataFilteringProfileListAllParams":"crate::dlp::DataFilteringProfileListParams",
                    "DictionaryFileInput":"crate::dlp::models::BinaryFile","DictionaryUploadParams":"crate::dlp::DictionaryUpload","DictionaryGetParams":"Option<bool>",
                }
                if name not in mapping:raise ValueError(f"DLP native behavioral adapter lacks an explicit mapping: {name}")
                row.update(status="native_client_mapping",rustType=mapping[name],reason="Canonical DLP client/input adapter; list-all caps use explicit CollectOptions and request lifecycle uses RequestOptions")
            elif name in {"GetTokenOptions","OAuthManagementClientOptions","IamScopesClientOptions"}:
                row.update(status="native_client_mapping",rustType="crate::airs::management::GetTokenOptions" if name=="GetTokenOptions" else "crate::Client",reason="Implemented management token request or explicit OAuth client configuration")
            elif name in {"ContentOptions","ScanCallOptions","SyncScanOptions"}:
                target={"ContentOptions":"crate::airs::scan::ContentBuilder","ScanCallOptions":"crate::RequestOptions","SyncScanOptions":"crate::airs::scan::SyncScanOptions"}[name]
                row.update(status="native_client_mapping",rustType=target,reason="Implemented native scan builder or per-call deadline/cancellation/retry options")
            elif family=="red_team":
                # Public behavioral options are handwritten client APIs, not JSON models.
                options={
                    "RedTeamListOptions":"RedTeamListOptions",
                    "RedTeamScanListOptions":"RedTeamScanListOptions",
                    "RedTeamScanListAllOptions":"RedTeamScanListOptions",
                    "AttackListOptions":"AttackListOptions","GoalListOptions":"GoalListOptions",
                    "PromptsBySetListOptions":"PromptsBySetListOptions",
                    "PromptSetsReportOptions":"PromptSetsReportOptions",
                    "CustomAttacksReportListOptions":"CustomAttacksReportListOptions",
                    "PromptSetListOptions":"PromptSetListOptions","PromptSetListAllOptions":"PromptSetListOptions",
                    "PromptListOptions":"PromptListOptions","PromptListAllOptions":"PromptListOptions",
                    "TargetListOptions":"TargetListOptions","TargetListAllOptions":"TargetListOptions",
                    "TargetOperationOptions":"TargetOperationOptions",
                    "AdapterListOptions":"AdapterListOptions","AdapterListAllOptions":"AdapterListOptions",
                    "AdapterOperationOptions":"AdapterOperationOptions","ChannelListOptions":"ChannelListOptions",
                }
                configs={
                    "RedTeamClientOptions","RedTeamScansClientOptions","RedTeamReportsClientOptions",
                    "RedTeamInstancesClientOptions","RedTeamCustomAttackReportsClientOptions",
                    "RedTeamCustomAttacksClientOptions","RedTeamTargetsClientOptions",
                    "RedTeamAdaptersClientOptions","RedTeamNetworkBrokerClientOptions","RedTeamEulaClientOptions",
                }
                if name in options:
                    row.update(status="native_client_mapping",rustType="crate::airs::red_team::"+options[name],nativeFeature="red-team",additionalTypes=["crate::RequestOptions"],reason="Implemented native Red Team query/operation options; explicit total deadline and cancellation are separate RequestOptions")
                    if "ListAllOptions" in name:
                        row["additionalTypes"].append("crate::pagination::CollectOptions")
                        row["reason"]="Implemented native list options plus explicit bounded CollectOptions; collection reports truncation/partial failure/resume, with starting skip as a native extension"
                    if name=="ChannelListOptions":row["reason"]="Implemented channel filters; status scalar or array maps to a repeated ordered Vec<String>, including duplicates; empty vector emits no status query"
                elif name in configs:
                    row.update(status="native_client_mapping",rustType="crate::Client",nativeFeature="red-team",reason="Explicit OAuth Client configuration replaces internal options/environment fallback; constructors share provider/policy/session/limits and omit x-tsg-id")
                    if name=="RedTeamClientOptions":
                        row.update(additionalTypes=["crate::ClientBuilder","crate::Endpoint"],reason="RedTeamClient::new(Client, management Endpoint, broker Endpoint) uses explicit admitted destinations and shared immutable OAuth/transport/session configuration")
                else:raise ValueError(f"Red Team native behavioral adapter lacks an explicit mapping: {name}")
            elif family=="model_security":
                options={
                    "ModelSecurityScanListOptions","ModelSecurityScanListAllOptions",
                    "ModelSecurityEvaluationListOptions","ModelSecurityFileListOptions",
                    "ModelSecurityLabelListOptions","ModelSecurityViolationListOptions",
                    "ModelSecurityModelListOptions","ModelSecurityModelListAllOptions",
                    "ModelSecurityModelVersionListOptions","ModelSecurityModelVersionListAllOptions",
                    "ModelSecurityModelVersionFileListOptions","ModelSecurityModelVersionFileListAllOptions",
                    "ModelSecurityGroupListOptions","ModelSecurityGroupListAllOptions",
                    "ModelSecurityRuleInstanceListOptions","ModelSecurityRuleListOptions",
                    "ModelSecurityRuleListAllOptions","CustomRuleListOptions","SnapshotVersionListOptions",
                }
                configs={
                    "ModelSecurityClientOptions","ModelSecurityScansClientOptions",
                    "ModelSecurityModelsClientOptions","ModelSecurityRulesClientOptions",
                    "ModelSecurityGroupsClientOptions","ModelSecurityCustomRulesClientOptions",
                }
                if name in options:
                    row.update(status="native_client_mapping",rustType="crate::airs::model_security::"+name,nativeFeature="model-security",additionalTypes=["crate::RequestOptions"],reason="Implemented typed Model Security query options; per-call total deadline/cancellation/retry controls are separate RequestOptions")
                    if "ListAllOptions" in name:
                        row["reason"]="Implemented native list-all options include bounded pagination::CollectOptions; source filters are retained and collections report truncation/partial failure/resume"
                elif name in configs:
                    row.update(status="native_client_mapping",rustType="crate::Client",nativeFeature="model-security",reason="Explicit OAuth Client configuration replaces internal options/environment fallback; constructors share provider/policy/session/limits and omit x-tsg-id")
                    if name=="ModelSecurityClientOptions":
                        row.update(additionalTypes=["crate::ClientBuilder","crate::Endpoint"],reason="ModelSecurityClient::new(Client, data Endpoint, management Endpoint) selects admitted data/management destinations and shares immutable OAuth/transport/session configuration")
                else:raise ValueError(f"Model Security native behavioral adapter lacks an explicit mapping: {name}")
            elif name in {"AISecSDKExceptionMetadata","AISecSDKProblemDetails"}:
                row.update(status="native_foundation_mapping",rustType="crate::Error",reason="Rust Error exposes bounded sanitized diagnostics; JavaScript exception diagnostic bags are not request data")
            else:
                phase=7 if name.startswith(("GatewayRealtime","GatewayWebSocket")) or name in {"GatewayStream","AIGatewayWorkspaceProvisionOptions"} else 6 if family=="gateway" else 7 if family=="agentguard" else 5 if family in {"red_team","model_security"} else 4 if family in {"management","dlp","iam"} else 3 if family=="scan" else 4 if entry["source"]["file"]=="src/listing.ts" else 7
                row.update(status="deferred_client_adapter",owningPhase=phase,reason="Behavioral/client configuration contract; native adapter belongs to the product client phase")
            self.model_rows.append(row)


def validate_graph_contract(inventory, validator_source=None):
    """Refuse unimplemented nested checks and key normalization at generation time."""
    nodes=inventory["schemas"]["definitions"]
    actual={kind:{check["kind"] for node in nodes.values() if node["kind"]==kind for check in node.get("checks",[])} for kind in ["ZodString","ZodNumber"]}
    if actual["ZodString"]!=STRING_CHECKS or actual["ZodNumber"]!=NUMBER_CHECKS:
        raise ValueError("Schema string/number check kinds differ from the reviewed native implementation")
    regexes=set()
    for sid,node in nodes.items():
        for check in node.get("checks",[]):
            if check["kind"]=="regex":
                regex=check.get("regex",{})
                if regex.get("flags")!="":raise ValueError(f"Unsupported regex flags in {sid}")
                regexes.add(regex.get("$regexp"))
    if regexes!=REGEX_SOURCES:raise ValueError("Regex sources differ from the reviewed native implementation")
    source=validator_source if validator_source is not None else (ROOT/"crates/scm-sdk/src/validation.rs").read_text()
    def implemented(function,next_function):
        body=source.split("fn "+function+"(",1)[1].split("fn "+next_function+"(",1)[0]
        return set(re.findall(r'^\s*"([^"\n]+)"\s*=>',body,re.M))
    if implemented("check_string","check_number")!=STRING_CHECKS or implemented("check_number","multiple_of")!=NUMBER_CHECKS:
        raise ValueError("Native validator check dispatch differs from the reviewed inventory")
    pattern_body=source.split("fn patterns(",1)[1].split("fn email(",1)[0]
    native_regexes={value for value in re.findall(r'r"([^"\n]+)"',pattern_body) if value.startswith("^")}
    if native_regexes!=REGEX_SOURCES:raise ValueError("Native regex implementations differ from the reviewed inventory")
    def refs(value):
        if isinstance(value,dict):
            if "$ref" in value:yield value["$ref"]
            for child in value.values():yield from refs(child)
        elif isinstance(value,list):
            for child in value:yield from refs(child)
    record_keys=0
    for sid,node in nodes.items():
        if node["kind"]!="ZodRecord":continue
        record_keys+=1;pending=[node["key"]["$ref"]];seen=set()
        while pending:
            child=pending.pop()
            if child in seen:continue
            seen.add(child);definition=nodes[child]
            if definition["kind"] in {"ZodEffects","ZodDefault","ZodPipeline","ZodCatch"} or definition.get("coerce"):
                raise ValueError(f"Record key normalization needs an explicit native implementation: {sid}/{child}")
            pending.extend(ref for ref in refs(definition) if ref in nodes)
    return {"stringChecks":sorted(STRING_CHECKS),"numberChecks":sorted(NUMBER_CHECKS),"regexSources":sorted(REGEX_SOURCES),"recordKeys":record_keys,"recordKeyPolicy":"No effects, defaults, pipelines, catches or coercion in record key graphs"}

def union_contract(generator):
    return {"schemaVersion":1,"baselineCommit":generator.inventory["baseline"]["commit"],"unions":sorted(generator.union_rows,key=lambda row:row["rustType"])}

def verify_union_pins(actual, expected):
    if actual!=expected:raise ValueError("Public positional union identity differs from the committed pins; an explicit API migration and reviewed pin update are required")


def compact_graph(inventory):
    validate_graph_contract(inventory)
    roots = {}
    for root in inventory["schemas"]["roots"]:
        roots[root["source"]["file"] + "#" + root["name"]] = root["schema"]["$ref"]
        roots.setdefault(root["name"], root["schema"]["$ref"])
    for entry in inventory["exports"]:
        if entry["kind"] == "schema":
            roots[entry["name"]] = entry["schema"]["$ref"]
    nodes = {}
    for sid, original in inventory["schemas"]["definitions"].items():
        node = {key: value for key, value in original.items() if key not in {"semanticSource", "description", "semanticPortability", "provider", "effect"}}
        if "effect" in original:
            node["effect"] = {"type": original["effect"]["type"]}
        if original["kind"] in {"ZodDefault", "ZodEffects"}:
            node["requiresReviewedSemantics"] = True
        nodes[sid] = node
    return {"schemaVersion": 1, "roots": roots, "nodes": nodes}


def verify_fixtures():
    """Compile a separate native consumer against every public schema model."""
    ledger=json.loads((ROOT/"contracts/native-models.json").read_text())
    inventory=json.loads(SOURCE.read_text())
    definitions=inventory["schemas"]["definitions"]
    binary_nodes={sid for sid,node in definitions.items() if "z.instanceof(Blob)" in node.get("semanticSource",{}).get("source",{}).get("text","")}
    def references(value):
        if isinstance(value,dict):
            if "$ref" in value:yield value["$ref"]
            for child in value.values():yield from references(child)
        elif isinstance(value,list):
            for child in value:yield from references(child)
    def has_binary(sid,seen):
        if sid in seen:return False
        seen.add(sid)
        return sid in binary_nodes or any(has_binary(child,seen) for child in references(definitions[sid]))
    native_exports={row["export"] for row in ledger["schemas"] if has_binary(row["schema"],set())}
    directory=ROOT/".local/typed-model-oracle"
    (directory/"src").mkdir(parents=True,exist_ok=True)
    manifest='[workspace]\n[package]\nname="scm-typed-model-oracle"\nversion="0.0.0"\nedition="2024"\nrust-version="1.88"\n[dependencies]\nscm-sdk={path="../../crates/scm-sdk",default-features=false,features=["models"]}\nserde="=1.0.229"\nserde_json="=1.0.154"\n'
    (directory/"Cargo.toml").write_text(manifest)
    program='use serde::{Serialize,de::DeserializeOwned};\nuse serde_json::Value;\nfn typed<T:Serialize+DeserializeOwned>(value:Value)->Result<Value,String>{let decoded:T=serde_json::from_value(value).map_err(|_|"typed decode failed".to_string())?;serde_json::to_value(decoded).map_err(|_|"typed encode failed".to_string())}\nfn roundtrip(schema:&str,value:Value)->Result<Value,String>{match schema{\n'
    for row in ledger["schemas"]:
        namespace=row["namespace"]
        path="scm_sdk::"+namespace+"::models::"+row["rustType"]
        program+=f'{quoted(row["export"])}=>typed::<{path}>(value),\n'
    program+='_=>Err("schema has no typed model".into())}}\n'
    # Private expiry choices are native client options, not public wire model
    # exports. Compile their actual frozen 7/30/60 graph through the same generator
    # to exercise its literal decoder without inventing a public SDK contract.
    expiry=Generator(inventory)
    expiry.emit("s000076","ExpiryDays")
    program+='mod expiry_contract {use scm_sdk::data::{FiniteNumber,JsonValue};use serde::{Serialize,Deserialize};use std::fmt;\n'
    program+='\n'.join(expiry.declarations[name].replace("crate::","scm_sdk::") for name in sorted(expiry.declarations))+'}\n'
    program+='const EXPECTED:&[&str]=&['+','.join(quoted(row["export"]) for row in ledger["schemas"])+'];\n'
    program+='const NATIVE:&[&str]=&['+','.join(quoted(name) for name in sorted(native_exports))+'];\n'
    program+=r'''fn main(){
        let input=std::fs::read_to_string(std::env::args().nth(1).expect("fixture path required")).expect("read oracle fixtures");
        let corpus:Value=serde_json::from_str(&input).expect("fixture JSON");
        let mut checked=0usize;let mut schemas=std::collections::BTreeSet::new();let mut failures=Vec::new();
        for root in corpus["roots"].as_array().expect("roots"){
            for name in root["names"].as_array().expect("root names"){
                let name=name.as_str().expect("schema name");
                for case in root["jsonCases"].as_array().expect("cases"){
                    for mode in ["zod","response"]{
                        if case[mode]["success"]!=true{continue;}
                        let expected=case[mode]["output"].clone();
                        checked+=1;schemas.insert(name.to_owned());
                        match roundtrip(name,expected.clone()){
                            Ok(actual)if actual==expected=>{},
                            Ok(_)=>failures.push(format!("{} {} {}: typed roundtrip differs",name,case["label"],mode)),
                            Err(reason)=>failures.push(format!("{} {} {}: {}",name,case["label"],mode,reason)),
                        }
                    }
                }
            }
        }
        let mut expiry_identity_checks=0;
        for (number,expected) in [(7,1),(30,2),(60,3)] {
            for value in [serde_json::json!(number),serde_json::json!(number as f64)] {
                let decoded:expiry_contract::ExpiryDays=serde_json::from_value(value).expect("frozen expiry literal accepted");
                let actual=match decoded {
                    expiry_contract::ExpiryDays::Variant1(value)=>{assert_eq!(value.as_inner().as_f64(),Some(7.0));assert_eq!(value.into_inner().as_f64(),Some(7.0));1},
                    expiry_contract::ExpiryDays::Variant2(value)=>{assert_eq!(value.as_inner().as_f64(),Some(30.0));assert_eq!(value.into_inner().as_f64(),Some(30.0));2},
                    expiry_contract::ExpiryDays::Variant3(value)=>{assert_eq!(value.as_inner().as_f64(),Some(60.0));assert_eq!(value.into_inner().as_f64(),Some(60.0));3},_=>0};
                expiry_identity_checks+=1;
                if actual!=expected {failures.push("frozen expiry literal selected wrong native variant".into());}
            }
        }
        if serde_json::from_value::<expiry_contract::ExpiryDaysVariant1>(serde_json::json!(30)).is_ok(){failures.push("expiry literal 7 accepted 30".into());}
        let mut native_only=0;
        for name in EXPECTED {
            if !schemas.contains(*name) {
                if NATIVE.contains(name){native_only+=1;}else{failures.push(format!("{name}: no successful JSON model specimen"));}
            }
        }
        if checked==0{failures.push("typed oracle checked no successful specimens".into());}
        println!("{}",serde_json::json!({"publicSchemas":EXPECTED.len(),"typedCases":checked,"typedSchemas":schemas.len(),"nativeOnlySchemas":native_only,"expiryLiteralIdentityChecks":expiry_identity_checks,"failures":failures.len()}));
        for message in failures.iter().take(40){eprintln!("{message}");}
        if !failures.is_empty(){std::process::exit(1);}
    }
    '''
    (directory/"src/main.rs").write_text(program)
    cargo=shutil.which("cargo") or str(pathlib.Path.home()/".cargo/bin/cargo")
    result=subprocess.run([cargo,"+1.99.0","run","--offline","--manifest-path",str(directory/"Cargo.toml"),"--",str(ROOT/"contracts/native-fixtures/schemas.json")],cwd=ROOT)
    if result.returncode:raise ValueError("Independent typed model oracle failed")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    parser.add_argument("--only", choices=FAMILIES.values(), help="Development-only initial family slice")
    parser.add_argument("--graph-only", action="store_true")
    parser.add_argument("--initialize-union-pins",action="store_true",help="Create the first reviewed positional-union ledger; refuses to overwrite an existing ledger")
    parser.add_argument("--verify-fixtures",action="store_true",help="Build a separate native consumer and roundtrip all successful frozen oracle cases")
    args = parser.parse_args()
    if args.verify_fixtures:
        verify_fixtures()
        return
    inventory = json.loads(SOURCE.read_text())
    if inventory["baseline"]["commit"] != "4a80dbdb3dc14365e1c25d838ac0bd139485d48c":
        raise ValueError("Unexpected model generation baseline")
    outputs = {ROOT / "crates/scm-sdk/src/validation/schema_graph.json": json.dumps(compact_graph(inventory), separators=(",", ":"), ensure_ascii=False) + "\n"}
    if not args.graph_only:
        generator = Generator(inventory)
        rust = generator.generate(args.only)
        if not args.only:
            pins=union_contract(generator)
            if args.initialize_union_pins:
                if args.check or UNION_PINS.exists():raise ValueError("Union pins already exist or initialization requested in check mode")
                UNION_PINS.write_text(json.dumps(pins,indent=2)+"\n")
            if not UNION_PINS.is_file():raise ValueError("Reviewed positional union pins are missing; initialize once before review")
            verify_union_pins(pins,json.loads(UNION_PINS.read_text()))
        formatter = shutil.which("rustfmt") or str(pathlib.Path.home() / ".cargo/bin/rustfmt")
        for file, content in rust.items():
            result = subprocess.run([formatter, "+1.99.0", "--edition", "2024"], input=content, text=True, capture_output=True)
            if result.returncode:
                raise ValueError(f"Generated Rust syntax failure {file.name}: {result.stderr[:2000]}")
            outputs[file] = result.stdout
        unresolved = [row for row in generator.model_rows if row["status"].startswith("pending_")]
        if not args.only and unresolved:
            raise ValueError(f"Known source models lack a typed adapter: {[row['export'] for row in unresolved]}")
        ledger = {"schemaVersion": 1, "baselineCommit": inventory["baseline"]["commit"], "sourceSha256": sha(SOURCE.read_bytes()), "generatorSha256": sha(pathlib.Path(__file__).read_bytes()), "partialFamily": args.only, "structuralChecks":validate_graph_contract(inventory), "unionVariantPins":"contracts/native-union-variants.json", "unionVariantPinsSha256":sha(UNION_PINS.read_bytes()) if UNION_PINS.is_file() else None, "semanticRegistry": "contracts/native-semantics.json", "nativeBoundaries": [{"schema":sid,"rustType":"BinaryFile" if "z.instanceof(Blob)" in node.get("semanticSource",{}).get("source",{}).get("text","") else "crate::validation::NativeDate" if node["kind"]=="ZodDate" else "crate::Transport" if sid=="s000009" else "crate::RequestOptions"} for sid,node in inventory["schemas"]["definitions"].items() if node["kind"]=="ZodDate" or sid in {"s000009","s000014"} or "z.instanceof(Blob)" in node.get("semanticSource",{}).get("source",{}).get("text","")], "transformOutputs": [{"schema":sid,"representation":"object with required message string" if sid in TRANSFORM_MESSAGE else "array unwrapped from data envelope" if sid=="s002084" else "object with required type string and preserved additional JSON fields"} for sid in sorted(TRANSFORM_MESSAGE|{"s002084","s006118"})], "schemas": generator.schema_models, "modelExports": generator.model_rows, "generatedDeclarations": len(generator.declarations), "aliases": len(generator.aliases), "deferred": [row for row in generator.model_rows if row["status"].startswith(("pending_","deferred_"))]}
        outputs[ROOT / "contracts/native-models.json"] = json.dumps(ledger, indent=2) + "\n"
    for file, content in outputs.items():
        if args.check:
            if not file.is_file() or file.read_text() != content:
                raise ValueError(f"Generated output differs: {file.relative_to(ROOT)}")
        else:
            file.parent.mkdir(parents=True, exist_ok=True)
            if file.is_file() and file.read_text()==content:continue
            temporary=file.with_name(file.name+".generating")
            temporary.write_text(content)
            temporary.replace(file)
    print(json.dumps({"checked": args.check, "outputs": len(outputs), "graphNodes": len(inventory["schemas"]["definitions"]), "partialFamily": args.only}))


if __name__ == "__main__":
    main()
