//! The snapshot: its caller's reading of one object's evidence at one
//! moment, read once before the tick, which every guard of the tick reads
//! (96, 98).

use std::collections::BTreeMap;

use time::OffsetDateTime;
use toml::Value;

/// Its caller's reading of one object at one moment (96, 98). The engine
/// serves no evidence of its own: every name is the caller's.
#[derive(Debug, Clone, PartialEq)]
pub struct Snapshot {
    /// The moment it was read: a region entering a state records it, and
    /// an `older` guard measures an age to it.
    pub at: OffsetDateTime,
    /// The evidence read for the object as a whole, by name.
    pub evidence: BTreeMap<String, Value>,
    /// The evidence read for one region path, by path and name. An
    /// evidence is read for a path from the values given at that path,
    /// else at the nearest path above it, else the object's.
    pub regions: BTreeMap<String, BTreeMap<String, Value>>,
    /// The responses addressed to the object, in the order given; which
    /// decision each answers is its caller's (97).
    pub responses: Vec<Response>,
    /// The object's owner, for a `parent` guard; `None` where it has none.
    pub owner: Option<Related>,
    /// The object's children, for a `children` guard.
    pub children: Vec<Related>,
}

/// A response addressed to the object: its id, which its caller gives, and
/// its answer as given (97).
#[derive(Debug, Clone, PartialEq)]
pub struct Response {
    /// Its id, by which the object applies it once (97).
    pub id: String,
    /// Its answer, matched against a `response` guard's pattern.
    pub answer: String,
}

/// An object related to the one ticked, its owner or a child: its kind and
/// the states its caller reads for it.
#[derive(Debug, Clone, PartialEq)]
pub struct Related {
    /// Its kind, by name as its caller gives it.
    pub kind: String,
    /// The states it holds.
    pub states: Vec<String>,
}
