//! A guard: what must hold for a transition to fire, one of the forms C.4
//! names (94).

use std::time::Duration;

use toml::Value;

/// A transition's `when`: a table of exactly one form C.4 names (94, 95).
#[derive(Debug, Clone, PartialEq)]
pub enum Guard {
    /// `{ all = [...] }`: every guard holds (94).
    All(Vec<Guard>),
    /// `{ any = [...] }`: one guard or more holds (94).
    Any(Vec<Guard>),
    /// `{ not = ... }`: the guard does not hold (94).
    Not(Box<Guard>),
    /// `{ ev = "<atom>", ... }`: an evidence, by the name its reader binds,
    /// put to one test (94, 98).
    Evidence {
        /// Its `ev`, the evidence's name as written; which names there are
        /// is the reader's (95, 98).
        ev: String,
        /// The one test it is put to (94, C.4).
        test: Test,
    },
    /// `{ response = "<answer>" }`: a response that answers a decision
    /// (94, 97).
    Response(String),
    /// `{ final = "<state>" }`: a final state of a statechart its state
    /// runs, `<state>` or `<region>.<state>`, as written (94, C.4).
    Final(String),
    /// `{ region = { name, in } }`: a sibling region's state (94, C.4).
    Region {
        /// Its `name`, the region's, a dotted path where the region lies
        /// in a statechart a sibling runs (C.4).
        name: String,
        /// Its `in`, the states of which the region's one must be (C.4).
        within: Vec<String>,
    },
    /// `{ parent = { in } }`: its owner's state (94, C.4).
    Parent {
        /// Its `in`, the states of which the owner's one must be (C.4).
        within: Vec<String>,
    },
    /// `{ children = { kind, ... } }`: its children's states (94, C.4).
    Children {
        /// Its `kind`, the children's, by name as written (C.4).
        kind: String,
        /// The one test the children are put to (C.4).
        test: ChildrenTest,
    },
    /// `{ always = <bool> }`: always, as written (94, C.4).
    Always(bool),
}

/// An evidence's one test, the key beside its `ev` (94, C.4).
#[derive(Debug, Clone, PartialEq)]
pub enum Test {
    /// `is`: equal to the value (94).
    Is(Value),
    /// `in`: one of the values (94).
    In(Vec<Value>),
    /// `exists`: whether the evidence exists (94).
    Exists(bool),
    /// `gt`: greater than the value, by order (94).
    Gt(Value),
    /// `gte`: greater than or equal to the value, by order (94).
    Gte(Value),
    /// `lt`: less than the value, by order (94).
    Lt(Value),
    /// `lte`: less than or equal to the value, by order (94).
    Lte(Value),
    /// `older`: older than the duration, by age, written as a whole number
    /// of `s`, `m` or `h` (94, C.4).
    Older(Duration),
    /// `eq_ev`: equal to another evidence, by its name (94).
    EqEv(String),
    /// `ne_ev`: not equal to another evidence, by its name (94).
    NeEv(String),
    /// `gt_ev`: greater than another evidence, by its name (94).
    GtEv(String),
    /// `gte_ev`: greater than or equal to another evidence, by its name
    /// (94).
    GteEv(String),
    /// `lt_ev`: less than another evidence, by its name (94).
    LtEv(String),
}

/// A `children` guard's one test, the key beside its `kind` (94, C.4).
#[derive(Debug, Clone, PartialEq)]
pub enum ChildrenTest {
    /// `all`: every child is in one of the states (C.4).
    All(Vec<String>),
    /// `any`: a child or more is in one of the states (C.4).
    Any(Vec<String>),
    /// `none`: no child is in any of the states (C.4).
    None(Vec<String>),
    /// `count_gte`: at least that many children (C.4).
    CountGte(u64),
}
