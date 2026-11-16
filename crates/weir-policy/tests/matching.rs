//! A rule's match reads facts and the capture's age alone (123; record
//! 0011).

use jiff::SignedDuration;
use serde_json::json;
use weir_policy::{Facts, Match, People, Policy, Value};

/// The match a rule's `when` spells.
fn when(when: &str) -> Match {
    let settings = format!("[[rules]]\nname = \"r\"\nwhen = {when}\ndrop = \"a reason\"\n");
    let policy = Policy::read(&settings).unwrap_or_else(|error| panic!("{when}: {error}"));
    policy.rules()[0].when.clone()
}

fn hours(hours: i64) -> Option<SignedDuration> {
    Some(SignedDuration::from_hours(hours))
}

#[test]
fn any_value_of_a_fact_holds_a_comparison() {
    let people = People::read(
        r#"
[[org]]
name = "Globex"
domains = ["globex.com"]
client = true

[[org]]
name = "Initech"
domains = ["initech.com"]
"#,
    )
    .expect("a people list");
    let stated = Facts::stated(&json!({"attendee": ["lee@initech.com", "kim@globex.com"]}))
        .expect("stated facts");
    let joined = people.join(&stated, &Facts::new());
    assert_eq!(
        joined.get("attendee.client"),
        Some(&[Value::Bool(false), Value::Bool(true)][..])
    );
    assert!(when(r#"{ fact = "attendee.client", is = true }"#).holds(&joined, None));
    assert!(when(r#"{ fact = "attendee.client", is = false }"#).holds(&joined, None));
    assert!(when(r#"{ fact = "attendee.org", in = ["Umbrella", "Globex"] }"#).holds(&joined, None));
    assert!(!when(r#"{ fact = "attendee.org", in = ["Umbrella"] }"#).holds(&joined, None));
}

#[test]
fn contains_evens_case() {
    let mut facts = Facts::new();
    facts.push("title", Value::Text("Daily Stand-up".into()));
    assert!(when(r#"{ fact = "title", contains = "stand-up" }"#).holds(&facts, None));
    assert!(when(r#"{ fact = "title", contains = "STAND-UP" }"#).holds(&facts, None));
    assert!(!when(r#"{ fact = "title", contains = "retro" }"#).holds(&facts, None));
    // A fact absent holds no comparison.
    assert!(!when(r#"{ fact = "level", contains = "" }"#).holds(&facts, None));
    assert!(!when(r#"{ fact = "level", is = "patch" }"#).holds(&facts, None));
}

#[test]
fn exists_false_on_a_fact_not_stated() {
    let facts = Facts::stated(&json!({"length": 900})).expect("stated facts");
    assert!(when(r#"{ fact = "level", exists = false }"#).holds(&facts, None));
    assert!(!when(r#"{ fact = "level", exists = true }"#).holds(&facts, None));
    assert!(when(r#"{ fact = "length", exists = true }"#).holds(&facts, None));
    // 900 and 900.0 are one number.
    assert!(when(r#"{ fact = "length", is = 900.0 }"#).holds(&facts, None));
}

#[test]
fn older_than_a_span() {
    let facts = Facts::new();
    assert!(when(r#"{ older = "1h" }"#).holds(&facts, hours(2)));
    assert!(!when(r#"{ older = "2h" }"#).holds(&facts, hours(2)));
    assert!(when(r#"{ older = "119m" }"#).holds(&facts, hours(2)));
    assert!(!when(r#"{ older = "1d" }"#).holds(&facts, hours(2)));
    // No age, or an event after the time given, is past no span.
    assert!(!when(r#"{ older = "0m" }"#).holds(&facts, None));
    assert!(!when(r#"{ older = "0m" }"#).holds(&facts, hours(-1)));
    assert!(when(r#"{ not = { older = "1h" } }"#).holds(&facts, None));
}

#[test]
fn an_empty_all_holds_and_an_empty_any_does_not() {
    let some = Facts::stated(&json!({"level": "patch"})).expect("stated facts");
    for facts in [Facts::new(), some] {
        assert!(when("{ all = [] }").holds(&facts, None));
        assert!(!when("{ any = [] }").holds(&facts, None));
    }
}
