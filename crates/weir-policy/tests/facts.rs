//! A fact is a name with one value or several, and each name has one sort
//! (117, 118; C.1, C.9; record 0010).

use jiff::civil::date;
use serde_json::{json, Number};
use weir_policy::{sort, FactError, Facts, Sort, Value};

fn text(text: &str) -> Value {
    Value::Text(text.to_string())
}

fn refused(facts: serde_json::Value) -> FactError {
    match Facts::stated(&facts) {
        Ok(read) => panic!("{facts} was read: {read:?}"),
        Err(error) => error,
    }
}

#[test]
fn a_fact_of_several_values() {
    let facts = Facts::stated(&json!({"attendee": ["dana@acme.com", "kim@globex.com"]}))
        .expect("stated facts");
    assert_eq!(
        facts.get("attendee"),
        Some(&[text("dana@acme.com"), text("kim@globex.com")][..])
    );
}

#[test]
fn a_fact_of_no_value() {
    let error = refused(json!({"level": []}));
    assert_eq!(error.field, "facts.level");
    assert!(error.reason.contains("no values"), "{error}");
}

#[test]
fn a_value_of_an_object() {
    assert_eq!(refused(json!({"level": {"major": 2}})).field, "facts.level");
    assert_eq!(refused(json!({"level": null})).field, "facts.level");
    assert_eq!(
        refused(json!({"level": ["patch", ["minor"]]})).field,
        "facts.level[1]"
    );
}

#[test]
fn a_date_in_a_string() {
    let facts = Facts::stated(&json!({"released": "2026-11-14"})).expect("stated facts");
    assert_eq!(
        facts.get("released"),
        Some(&[Value::Date(date(2026, 11, 14))][..])
    );
    // A policy spells a value as an offer does.
    assert_eq!(
        facts.get("released"),
        Some(&[Value::from_text("2026-11-14")][..])
    );
    // A string not of that shape, or a day no calendar holds, is a text.
    for other in ["2026-11-14T09:00:00Z", "2026-02-30", "20261114"] {
        assert_eq!(Value::from_text(other), text(other));
    }
}

#[test]
fn one_number_two_spellings() {
    let whole = Value::Number(Number::from(900));
    let fraction = Value::Number(Number::from_f64(900.0).expect("a number"));
    assert_eq!(whole, fraction);
    assert_eq!(
        Facts::stated(&json!({"length": 900})).expect("stated facts"),
        Facts::stated(&json!({"length": 900.0})).expect("stated facts")
    );
    assert_ne!(whole, text("900"));
}

#[test]
fn a_fact_named_in_capitals() {
    let error = refused(json!({"Level": "patch"}));
    assert_eq!(error.field, "facts.Level");
    assert_eq!(sort("Level"), None);
}

#[test]
fn a_stated_fact_under_a_read_facts_name() {
    assert_eq!(refused(json!({"says": "request"})).field, "facts.says");
    assert_eq!(sort("says"), Some(Sort::Read));
}

#[test]
fn a_stated_fact_under_a_joined_facts_name() {
    assert_eq!(
        refused(json!({"attendee.client": true})).field,
        "facts.attendee.client"
    );
    assert_eq!(sort("attendee.client"), Some(Sort::Joined));
    assert_eq!(sort("attendee.email"), None);
}

#[test]
fn a_stated_fact_under_an_offers_own_field() {
    let error = refused(json!({"title": "Daily stand-up"}));
    assert_eq!(error.field, "facts.title");
    assert!(error.reason.contains("own `title`"), "{error}");
    // A rule reads the offer's own fields as stated facts (117).
    assert_eq!(sort("title"), Some(Sort::Stated));
}

#[test]
fn read_facts_are_of_their_sort_alone() {
    let read = Facts::read(&json!({"says": "request", "due": "2026-11-20"})).expect("read facts");
    assert_eq!(read.get("says"), Some(&[text("request")][..]));
    assert_eq!(
        read.get("due"),
        Some(&[Value::Date(date(2026, 11, 20))][..])
    );
    let error = Facts::read(&json!({"level": "patch"})).expect_err("a stated fact");
    assert_eq!(error.field, "facts.level");
}

#[test]
fn facts_written_as_they_were_read() {
    let given = json!({"attendee": ["a@acme.com", "b@acme.com"], "length": 900, "released": "2026-11-14", "recorded": true});
    let facts = Facts::stated(&given).expect("stated facts");
    assert_eq!(facts.to_json(), given);
    assert_eq!(serde_json::to_value(&facts).expect("JSON"), given);
}
