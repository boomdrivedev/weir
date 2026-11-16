//! A transition's guard, read into the engine's [`Guard`]: a table of
//! exactly one form C.4 names, a guard of no form refused naming its path
//! (94, 95; C.4; core.54).

use toml::{Table, Value};
use weir_engine::{ChildrenTest, Guard, Test};

use crate::fields::field;
use crate::quiet;
use crate::refusal::Problems;

/// Why a guard of no form C.4 names is refused (95).
const NO_FORM: &str = "is not a guard of a form the format names";

/// The tests an evidence is put to, one beside its `ev` (94, C.4).
const EVIDENCE_TESTS: [&str; 13] = [
    "is", "in", "exists", "gt", "gte", "lt", "lte", "older", "eq_ev", "ne_ev", "gt_ev", "gte_ev",
    "lt_ev",
];

/// The tests a `children` guard puts its children to, one beside its
/// `kind` (C.4).
const CHILDREN_TESTS: [&str; 4] = ["all", "any", "none", "count_gte"];

/// The guard at `at`, its path built from its transition's `when` and,
/// within it, `all[<n>]`, `any[<n>]` and `not`. A value that is no table,
/// or a table whose keys are those of no form C.4 names, is refused naming
/// `at`; a form's own field of the wrong shape is refused naming that field
/// (95). What a guard's names name is the reader's, and no guard is
/// evaluated (98; core.32).
pub(crate) fn read_guard(guard: &Value, at: &str, problems: &mut Problems) -> Guard {
    let Some(table) = guard.as_table() else {
        problems.refuse(at, NO_FORM);
        return Guard::Always(false);
    };
    let Some(form) = form(table) else {
        problems.refuse(at, NO_FORM);
        return Guard::Always(false);
    };
    let value = &table[form];
    let here = field(at, form);
    match form {
        "all" => Guard::All(guards(value, &here, problems)),
        "any" => Guard::Any(guards(value, &here, problems)),
        "not" => Guard::Not(Box::new(read_guard(value, &here, problems))),
        "response" => Guard::Response(name(value, &here, problems)),
        "final" => Guard::Final(name(value, &here, problems)),
        "always" => Guard::Always(flag(value, &here, problems)),
        "region" => {
            let inner = inner(value);
            Guard::Region {
                name: name(&inner["name"], &field(&here, "name"), problems),
                within: states(&inner["in"], &field(&here, "in"), problems),
            }
        }
        "parent" => Guard::Parent {
            within: states(&inner(value)["in"], &field(&here, "in"), problems),
        },
        "children" => {
            let inner = inner(value);
            let kind = name(&inner["kind"], &field(&here, "kind"), problems);
            let (test, value) = inner
                .iter()
                .find(|(key, _)| key.as_str() != "kind")
                .map(|(key, value)| (key.as_str(), value))
                .expect("a children guard's test");
            let at = field(&here, test);
            let test = match test {
                "all" => ChildrenTest::All(states(value, &at, problems)),
                "any" => ChildrenTest::Any(states(value, &at, problems)),
                "none" => ChildrenTest::None(states(value, &at, problems)),
                _ => ChildrenTest::CountGte(count(value, &at, problems)),
            };
            Guard::Children { kind, test }
        }
        _ => evidence(table, at, problems),
    }
}

/// The form a guard's table is of, by its key, or `None` where its keys
/// are those of no form: one key of a form, or `ev` beside exactly one of
/// its tests; and for `region`, `parent` and `children`, a table holding
/// exactly the keys of the form, a `children` with exactly one of its
/// tests (C.4).
fn form(table: &Table) -> Option<&'static str> {
    let keys: Vec<&str> = table.keys().map(String::as_str).collect();
    let within = |value: &Value, required: &[&str], one_of: &[&str]| {
        value.as_table().is_some_and(|inner| {
            required.iter().all(|key| inner.contains_key(*key))
                && inner
                    .keys()
                    .filter(|key| one_of.contains(&key.as_str()))
                    .count()
                    == usize::from(!one_of.is_empty())
                && inner.len() == required.len() + usize::from(!one_of.is_empty())
        })
    };
    match keys.as_slice() {
        [one] => {
            let value = &table[*one];
            match *one {
                "all" => Some("all"),
                "any" => Some("any"),
                "not" => Some("not"),
                "response" => Some("response"),
                "final" => Some("final"),
                "always" => Some("always"),
                "region" if within(value, &["name", "in"], &[]) => Some("region"),
                "parent" if within(value, &["in"], &[]) => Some("parent"),
                "children" if within(value, &["kind"], &CHILDREN_TESTS) => Some("children"),
                _ => None,
            }
        }
        [_, _]
            if table.contains_key("ev") && keys.iter().any(|key| EVIDENCE_TESTS.contains(key)) =>
        {
            Some("ev")
        }
        _ => None,
    }
}

/// The table of a `region`, `parent` or `children` guard, which [`form`]
/// found a table.
fn inner(value: &Value) -> &Table {
    value.as_table().expect("the form's table")
}

/// An evidence, `{ ev = "<atom>", <test> = ... }`, put to its one test
/// (94, C.4).
fn evidence(table: &Table, at: &str, problems: &mut Problems) -> Guard {
    let ev = name(&table["ev"], &field(at, "ev"), problems);
    let (test, value) = table
        .iter()
        .find(|(key, _)| key.as_str() != "ev")
        .map(|(key, value)| (key.as_str(), value))
        .expect("an evidence's test");
    let here = field(at, test);
    let test = match test {
        "is" => Test::Is(value.clone()),
        "gt" => Test::Gt(value.clone()),
        "gte" => Test::Gte(value.clone()),
        "lt" => Test::Lt(value.clone()),
        "lte" => Test::Lte(value.clone()),
        "in" => match value {
            Value::Array(values) => Test::In(values.clone()),
            _ => {
                problems.refuse(&here, "must be an array of values");
                Test::In(Vec::new())
            }
        },
        "exists" => Test::Exists(flag(value, &here, problems)),
        "older" => Test::Older(match value {
            Value::String(text) => quiet::parse(text).unwrap_or_else(|| {
                problems.refuse(
                    &here,
                    format!("`{text}` is not one whole number followed by s, m or h, such as 2h"),
                );
                Default::default()
            }),
            _ => {
                problems.refuse(
                    &here,
                    "must be a duration written as a string, such as \"2h\"",
                );
                Default::default()
            }
        }),
        "eq_ev" => Test::EqEv(name(value, &here, problems)),
        "ne_ev" => Test::NeEv(name(value, &here, problems)),
        "gt_ev" => Test::GtEv(name(value, &here, problems)),
        "gte_ev" => Test::GteEv(name(value, &here, problems)),
        _ => Test::LtEv(name(value, &here, problems)),
    };
    Guard::Evidence { ev, test }
}

/// The guards of an `all` or `any`, each at `<here>[<n>]`.
fn guards(value: &Value, here: &str, problems: &mut Problems) -> Vec<Guard> {
    match value {
        Value::Array(items) => items
            .iter()
            .enumerate()
            .map(|(n, item)| read_guard(item, &format!("{here}[{n}]"), problems))
            .collect(),
        _ => {
            problems.refuse(here, "must be an array of guards");
            Vec::new()
        }
    }
}

/// A name a guard gives as written, a non-empty string.
fn name(value: &Value, at: &str, problems: &mut Problems) -> String {
    match value {
        Value::String(text) if !text.is_empty() => text.clone(),
        _ => {
            problems.refuse(at, "must be a non-empty string");
            String::new()
        }
    }
}

/// The states a guard names, an array of strings.
fn states(value: &Value, at: &str, problems: &mut Problems) -> Vec<String> {
    match value.as_array().and_then(|items| {
        items
            .iter()
            .map(|item| item.as_str().map(str::to_owned))
            .collect()
    }) {
        Some(states) => states,
        None => {
            problems.refuse(at, "must be an array of strings");
            Vec::new()
        }
    }
}

/// A guard's flag, true or false.
fn flag(value: &Value, at: &str, problems: &mut Problems) -> bool {
    match value {
        Value::Boolean(flag) => *flag,
        _ => {
            problems.refuse(at, "must be true or false");
            false
        }
    }
}

/// A `children` guard's count, an integer of at least 0.
fn count(value: &Value, at: &str, problems: &mut Problems) -> u64 {
    match value {
        Value::Integer(count) if *count >= 0 => count.unsigned_abs(),
        _ => {
            problems.refuse(at, "must be an integer of at least 0");
            0
        }
    }
}
