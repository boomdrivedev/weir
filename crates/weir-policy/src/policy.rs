//! A policy, its rules and their matches, read and checked whole (122,
//! 123; C.9).
//!
//! A policy is its settings document's `[[rules]]`, in order. A rule that
//! breaks 122 or 123, a rule named twice, or a field C.9 does not name
//! refuses the whole policy, naming the field by its path counting from 0,
//! as `rules[1].when.all[0].fact` (record 0011).

use std::fmt;

use jiff::tz::TimeZone;
use jiff::{SignedDuration, Timestamp};
use serde_json::Number;
use toml::{Table, Value as Toml};
use weir_cadence::Schedule;

use crate::fact::{sort, Facts, Value};
use crate::people::{document, path, required_text, tables, text, unnamed, SettingsError};

/// A keeper's rules for one inbox, in order (122).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Policy {
    rules: Vec<Rule>,
}

/// One rule of a policy (122; C.9).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rule {
    /// Its name: lowercase letters, digits and `-`, no other rule's.
    pub name: String,
    /// Its match (123).
    pub when: Match,
    /// Its outcome, one of three.
    pub outcome: Outcome,
    /// Its priority, for a launch or a person's turn, and none for a drop.
    pub priority: Option<Priority>,
    /// Its window (129), none where it has none, and always none for a
    /// drop.
    pub window: Option<Schedule>,
}

/// A rule's outcome, one of three (122), each text never empty.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Outcome {
    /// A kind to launch, named as a launch names it, with the repository it
    /// launches in.
    Launch {
        /// The kind.
        kind: String,
        /// The repository.
        repository: String,
    },
    /// A person's turn, and what it waits on them for.
    Person(String),
    /// A drop, with its reason.
    Drop(String),
}

/// A rule's priority, one of the five levels (122; C.9).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Priority {
    /// `low`.
    Low,
    /// `normal`.
    Normal,
    /// `elevated`.
    Elevated,
    /// `high`.
    High,
    /// `urgent`.
    Urgent,
}

impl Priority {
    /// The five, lowest first.
    pub const ALL: [Priority; 5] = [
        Priority::Low,
        Priority::Normal,
        Priority::Elevated,
        Priority::High,
        Priority::Urgent,
    ];

    /// The level's word.
    pub fn as_str(self) -> &'static str {
        match self {
            Priority::Low => "low",
            Priority::Normal => "normal",
            Priority::Elevated => "elevated",
            Priority::High => "high",
            Priority::Urgent => "urgent",
        }
    }

    /// The level `text` spells, or none.
    pub fn parse(text: &str) -> Option<Priority> {
        Priority::ALL
            .into_iter()
            .find(|priority| priority.as_str() == text)
    }
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// A rule's match (123; C.9).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Match {
    /// `{ all = [...] }`: every match of the list holds.
    All(Vec<Match>),
    /// `{ any = [...] }`: some match of the list holds.
    Any(Vec<Match>),
    /// `{ not = ... }`: the match does not hold.
    Not(Box<Match>),
    /// `{ fact = "<name>", ... }`: a fact compared.
    Fact {
        /// The fact's name, one C.9 gives a sort.
        name: String,
        /// The comparison.
        test: Test,
    },
    /// `{ older = "<span>" }`: the capture's age past a span.
    Older(Span),
}

/// How a fact is compared (123; C.9).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Test {
    /// `is`: a value of the fact is this one.
    Is(Value),
    /// `in`: a value of the fact is one of the list's.
    In(Vec<Value>),
    /// `exists`: the fact has a value, or has none.
    Exists(bool),
    /// `contains`: a text value of the fact holds this text, its case
    /// evened.
    Contains(String),
}

/// A span a capture's age is compared with: a whole number of `m`, `h` or
/// `d` (C.9).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Span {
    text: String,
    duration: SignedDuration,
}

impl Span {
    /// The span `text` spells, or none.
    pub fn parse(text: &str) -> Option<Span> {
        let unit = text.chars().last()?;
        let count = &text[..text.len() - unit.len_utf8()];
        if count.is_empty() || !count.chars().all(|c| c.is_ascii_digit()) {
            return None;
        }
        let seconds: i64 = match unit {
            'm' => 60,
            'h' => 60 * 60,
            'd' => 24 * 60 * 60,
            _ => return None,
        };
        let duration = count.parse::<i64>().ok()?.checked_mul(seconds)?;
        Some(Span {
            text: text.to_string(),
            duration: SignedDuration::from_secs(duration),
        })
    }

    /// The span as a duration, a day being twenty-four hours.
    pub fn duration(&self) -> SignedDuration {
        self.duration
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.text)
    }
}

impl Match {
    /// Whether the match holds over `facts` and a capture's age (123),
    /// none where its event time cannot be read. A fact of several values
    /// holds a comparison when any value does, and a fact absent holds no
    /// `is`, `in` or `contains`; `contains` evens the case of both texts;
    /// `older` holds when the age is past its span, strictly; an `all` of
    /// none holds and an `any` of none does not (record 0011). It reads
    /// nothing but its arguments (core.58).
    pub fn holds(&self, facts: &Facts, age: Option<SignedDuration>) -> bool {
        match self {
            Match::All(matches) => matches.iter().all(|one| one.holds(facts, age)),
            Match::Any(matches) => matches.iter().any(|one| one.holds(facts, age)),
            Match::Not(one) => !one.holds(facts, age),
            Match::Fact { name, test } => {
                let values = facts.get(name).unwrap_or_default();
                match test {
                    Test::Is(value) => values.contains(value),
                    Test::In(list) => values.iter().any(|value| list.contains(value)),
                    Test::Exists(exists) => !values.is_empty() == *exists,
                    Test::Contains(text) => {
                        let text = text.to_lowercase();
                        values.iter().any(|value| {
                            matches!(value, Value::Text(held) if held.to_lowercase().contains(&text))
                        })
                    }
                }
            }
            Match::Older(span) => age.is_some_and(|age| age > span.duration()),
        }
    }
}

impl Rule {
    /// The name of an item the rule makes at `made_at` (128; C.9):
    /// `<rule>-<time>`, the time the first its window names after
    /// `made_at`, read in `zone`, or `made_at` itself for a rule with no
    /// window or a window that names no later time, written in UTC as
    /// `YYYYMMDDTHHMMSSZ` (C.5; record 0011).
    pub fn item_name(&self, made_at: Timestamp, zone: &TimeZone) -> String {
        let close = self
            .window
            .as_ref()
            .and_then(|window| window.first_after(made_at, zone))
            .map_or(made_at, |close| close.timestamp());
        format!("{}-{}", self.name, close.strftime("%Y%m%dT%H%M%SZ"))
    }
}

/// Every field a rule may hold (C.9).
const RULE_FIELDS: [&str; 8] = [
    "name",
    "when",
    "kind",
    "repository",
    "person",
    "drop",
    "priority",
    "window",
];

const NOT_A_MATCH: &str = "is not a match: a table of exactly one of `all`, `any`, `not` and \
                           `older`, or of `fact` with exactly one of `is`, `in`, `exists` and \
                           `contains` (123)";

impl Policy {
    /// Reads the policy of a settings document (122; C.9): its `[[rules]]`,
    /// in order, each checked whole; no `rules` is a policy of none. The
    /// document's other tables are not read.
    pub fn read(settings: &str) -> Result<Policy, SettingsError> {
        let document = document(settings)?;
        let mut rules: Vec<Rule> = Vec::new();
        for (at, table) in tables(&document, "rules")? {
            let rule = read_rule(table, &at)?;
            if let Some(n) = rules.iter().position(|held| held.name == rule.name) {
                return Err(SettingsError::new(
                    path(&at, "name"),
                    format!("is `{}`, which `rules[{n}]` is named too", rule.name),
                ));
            }
            rules.push(rule);
        }
        Ok(Policy { rules })
    }

    /// The rules, in the policy's order.
    pub fn rules(&self) -> &[Rule] {
        &self.rules
    }
}

/// One rule, its fields checked in the order name, match, outcome,
/// priority and window, after any field C.9 does not name (122).
fn read_rule(table: &Table, at: &str) -> Result<Rule, SettingsError> {
    unnamed(table, &RULE_FIELDS, at)?;
    let name = required_text(table, "name", at)?;
    let spelled = !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-');
    if !spelled {
        return Err(SettingsError::new(
            path(at, "name"),
            format!("is `{name}`, not lowercase letters, digits and `-`"),
        ));
    }
    let when = match table.get("when") {
        None => return Err(SettingsError::new(path(at, "when"), "is missing")),
        Some(when) => read_match(when, &path(at, "when"))?,
    };
    let outcome = read_outcome(table, at)?;
    let is_drop = matches!(outcome, Outcome::Drop(_));
    let priority = match (text(table, "priority", at)?, is_drop) {
        (Some(_), true) => {
            return Err(SettingsError::new(
                path(at, "priority"),
                "is given with a drop, which has none (122)",
            ));
        }
        (None, true) => None,
        (None, false) => {
            return Err(SettingsError::new(
                path(at, "priority"),
                "is missing: a launch or a person's turn has one (122)",
            ));
        }
        (Some(word), false) => Some(Priority::parse(&word).ok_or_else(|| {
            SettingsError::new(
                path(at, "priority"),
                format!(
                    "is `{word}`, not one of {}",
                    Priority::ALL.map(Priority::as_str).join(", ")
                ),
            )
        })?),
    };
    let window = match text(table, "window", at)? {
        Some(_) if is_drop => {
            return Err(SettingsError::new(
                path(at, "window"),
                "is given with a drop, which has none (122)",
            ));
        }
        Some(window) => Some(Schedule::parse(&window).map_err(|error| {
            SettingsError::new(path(at, "window"), format!("is `{window}`: {error}"))
        })?),
        None => None,
    };
    Ok(Rule {
        name,
        when,
        outcome,
        priority,
        window,
    })
}

/// Exactly one of `kind`, with `repository`, `person` and `drop`, the
/// second of several refused (122).
fn read_outcome(table: &Table, at: &str) -> Result<Outcome, SettingsError> {
    let mut given = Vec::new();
    for name in ["kind", "person", "drop"] {
        if let Some(value) = text(table, name, at)? {
            if value.is_empty() {
                return Err(SettingsError::new(path(at, name), "is empty"));
            }
            given.push((name, value));
        }
    }
    let repository = text(table, "repository", at)?;
    let mut given = given.into_iter();
    let (name, value) = given
        .next()
        .ok_or_else(|| SettingsError::new(at, "holds none of `kind`, `person` and `drop` (122)"))?;
    if let Some((second, _)) = given.next() {
        return Err(SettingsError::new(
            path(at, second),
            format!("is a second outcome beside `{name}`: a rule holds exactly one (122)"),
        ));
    }
    match (name, repository) {
        ("kind", Some(repository)) if !repository.is_empty() => Ok(Outcome::Launch {
            kind: value,
            repository,
        }),
        ("kind", Some(_)) => Err(SettingsError::new(path(at, "repository"), "is empty")),
        ("kind", None) => Err(SettingsError::new(
            path(at, "repository"),
            "is missing: a launch names the repository it launches in (122)",
        )),
        (_, Some(_)) => Err(SettingsError::new(
            path(at, "repository"),
            "goes with `kind` alone (122)",
        )),
        ("person", None) => Ok(Outcome::Person(value)),
        _ => Ok(Outcome::Drop(value)),
    }
}

/// The match `value` spells at `at` (123; C.9).
fn read_match(value: &Toml, at: &str) -> Result<Match, SettingsError> {
    let Toml::Table(table) = value else {
        return Err(SettingsError::new(at, NOT_A_MATCH));
    };
    let only = |name: &str| table.len() == 1 && table.contains_key(name);
    if only("all") || only("any") {
        let name = if only("all") { "all" } else { "any" };
        let list = path(at, name);
        let Toml::Array(items) = &table[name] else {
            return Err(SettingsError::new(list, "is not a list of matches"));
        };
        let matches = items
            .iter()
            .enumerate()
            .map(|(n, item)| read_match(item, &format!("{list}[{n}]")))
            .collect::<Result<Vec<_>, _>>()?;
        return Ok(if name == "all" {
            Match::All(matches)
        } else {
            Match::Any(matches)
        });
    }
    if only("not") {
        return Ok(Match::Not(Box::new(read_match(
            &table["not"],
            &path(at, "not"),
        )?)));
    }
    if only("older") {
        let field = path(at, "older");
        return match &table["older"] {
            Toml::String(text) => Span::parse(text).map(Match::Older).ok_or_else(|| {
                SettingsError::new(
                    field,
                    format!("is `{text}`, not a whole number of `m`, `h` or `d`"),
                )
            }),
            _ => Err(SettingsError::new(field, "is not a span")),
        };
    }
    let test = table.keys().find(|key| *key != "fact").map(String::as_str);
    let (true, 2, Some(test @ ("is" | "in" | "exists" | "contains"))) =
        (table.contains_key("fact"), table.len(), test)
    else {
        return Err(SettingsError::new(at, NOT_A_MATCH));
    };
    let name = match &table["fact"] {
        Toml::String(name) if sort(name).is_some() => name.clone(),
        Toml::String(name) => {
            return Err(SettingsError::new(
                path(at, "fact"),
                format!("is `{name}`, not a fact's name C.9 spells"),
            ));
        }
        _ => return Err(SettingsError::new(path(at, "fact"), "is not a text")),
    };
    let field = path(at, test);
    let test = match (test, &table[test]) {
        ("is", value) => Test::Is(read_value(value, &field)?),
        ("in", Toml::Array(items)) => Test::In(
            items
                .iter()
                .enumerate()
                .map(|(n, item)| read_value(item, &format!("{field}[{n}]")))
                .collect::<Result<Vec<_>, _>>()?,
        ),
        ("in", _) => return Err(SettingsError::new(field, "is not a list of values")),
        ("exists", Toml::Boolean(flag)) => Test::Exists(*flag),
        ("exists", _) => return Err(SettingsError::new(field, "is not true or false")),
        (_, Toml::String(text)) => Test::Contains(text.clone()),
        _ => return Err(SettingsError::new(field, "is not a text")),
    };
    Ok(Match::Fact { name, test })
}

/// A value spelled as an offer spells it (C.9): a TOML date or time is
/// refused, since an offer's JSON spells none (record 0011).
fn read_value(value: &Toml, at: &str) -> Result<Value, SettingsError> {
    match value {
        Toml::String(text) => Ok(Value::from_text(text)),
        Toml::Integer(number) => Ok(Value::Number(Number::from(*number))),
        Toml::Float(number) => Number::from_f64(*number)
            .map(Value::Number)
            .ok_or_else(|| SettingsError::new(at, "is not a number JSON spells")),
        Toml::Boolean(flag) => Ok(Value::Bool(*flag)),
        Toml::Datetime(_) => Err(SettingsError::new(
            at,
            "is a TOML date or time: a date is spelled as an offer spells it, in a string",
        )),
        _ => Err(SettingsError::new(
            at,
            "is not a text, a number, true or false, or a date",
        )),
    }
}
