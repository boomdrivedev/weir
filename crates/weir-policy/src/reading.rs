//! A policy read for a signal and for a capture as it lands (124, 125).
//!
//! A rule reads a capture's stated facts, with its `source`, `title` and
//! `event_at` as facts its source stated under their own names (117), the
//! read facts a reader gave where there are some, and the facts joined from
//! them both by the people list as it stands (119, 121). Neither reading
//! reads a clock or changes anything: the time is its caller's (core.1,
//! core.56).

use jiff::{SignedDuration, Timestamp};

use crate::fact::{sort, Facts, Sort, Value};
use crate::people::People;
use crate::policy::{Match, Policy, Rule};

/// A capture's stated facts, with the offer's own fields a rule reads as
/// stated facts (117).
#[derive(Clone, Copy, Debug)]
pub struct Stated<'a> {
    /// The facts its offer stated.
    pub facts: &'a Facts,
    /// Its source.
    pub source: &'a str,
    /// Its title, where the offer gave one.
    pub title: Option<&'a str>,
    /// Its event time, in RFC 3339 with its offset, as offered.
    pub event_at: &'a str,
}

impl Stated<'_> {
    /// The stated facts with `source`, `title` and `event_at` added as
    /// texts (117).
    fn with_own_fields(&self) -> Facts {
        let mut facts = self.facts.clone();
        facts.push("source", Value::Text(self.source.to_string()));
        if let Some(title) = self.title {
            facts.push("title", Value::Text(title.to_string()));
        }
        facts.push("event_at", Value::Text(self.event_at.to_string()));
        facts
    }

    /// The capture's age at `now`, read from its event time (core.8), none
    /// where that cannot be read.
    fn age(&self, now: Timestamp) -> Option<SignedDuration> {
        let event_at: Timestamp = self.event_at.parse().ok()?;
        Some(now.duration_since(event_at))
    }
}

impl Policy {
    /// The first rule, in the policy's order, whose match holds over a
    /// signal's stated, joined and read facts at `now`, or none (125).
    pub fn for_signal(
        &self,
        capture: &Stated,
        read: &Facts,
        people: &People,
        now: Timestamp,
    ) -> Option<&Rule> {
        let mut facts = capture.with_own_fields();
        facts.extend(read);
        facts.extend(&people.join(capture.facts, read));
        let age = capture.age(now);
        self.rules()
            .iter()
            .find(|rule| rule.when.holds(&facts, age))
    }

    /// The first rule, in the policy's order, that names no read fact nor
    /// one joined from one and whose match holds over a capture's stated
    /// facts and those joined from them at `now`, as the capture lands, or
    /// none (125).
    pub fn for_capture(&self, capture: &Stated, people: &People, now: Timestamp) -> Option<&Rule> {
        let mut facts = capture.with_own_fields();
        facts.extend(&people.join(capture.facts, &Facts::new()));
        let age = capture.age(now);
        self.rules()
            .iter()
            .filter(|rule| !rule.names_a_read_fact())
            .find(|rule| rule.when.holds(&facts, age))
    }
}

impl Rule {
    /// Whether the rule's match, at any depth, names a read fact or a fact
    /// joined from one, `speaker.*` (125).
    pub fn names_a_read_fact(&self) -> bool {
        names_a_read_fact(&self.when)
    }
}

fn names_a_read_fact(when: &Match) -> bool {
    match when {
        Match::All(matches) | Match::Any(matches) => matches.iter().any(names_a_read_fact),
        Match::Not(one) => names_a_read_fact(one),
        Match::Fact { name, .. } => sort(name) == Some(Sort::Read) || name.starts_with("speaker."),
        Match::Older(_) => false,
    }
}
