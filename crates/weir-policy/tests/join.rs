//! An attendee joins by its address and a speaker by its name (119, 121;
//! record 0011).

use serde_json::json;
use weir_policy::{Facts, People, Value};

const C9: &str = include_str!("c9.toml");

fn c9() -> People {
    People::read(C9).expect("C.9's people list")
}

fn stated(facts: serde_json::Value) -> Facts {
    Facts::stated(&facts).expect("stated facts")
}

fn read(facts: serde_json::Value) -> Facts {
    Facts::read(&facts).expect("read facts")
}

fn text(text: &str) -> Value {
    Value::Text(text.to_string())
}

#[test]
fn an_attendee_joins_its_person_and_org() {
    let joined = c9().join(
        &stated(json!({"attendee": ["dana@acme.com", "kim@globex.com"]})),
        &Facts::new(),
    );
    assert_eq!(joined.get("attendee.name"), Some(&[text("Dana Reyes")][..]));
    assert_eq!(joined.get("attendee.role"), Some(&[text("executive")][..]));
    assert_eq!(joined.get("attendee.teams"), Some(&[text("web")][..]));
    assert_eq!(joined.get("attendee.org"), Some(&[text("Globex")][..]));
    assert_eq!(
        joined.get("attendee.client"),
        Some(&[Value::Bool(true)][..])
    );
    assert_eq!(joined.iter().count(), 5);

    // An address is compared with its case evened.
    let joined = c9().join(&stated(json!({"attendee": "Dana@ACME.com"})), &Facts::new());
    assert_eq!(joined.get("attendee.name"), Some(&[text("Dana Reyes")][..]));
}

#[test]
fn a_speaker_joins_by_the_name_a_transcript_gives() {
    let joined = c9().join(&Facts::new(), &read(json!({"speaker": "dana"})));
    assert_eq!(joined.get("speaker.name"), Some(&[text("Dana Reyes")][..]));
    assert_eq!(joined.get("speaker.role"), Some(&[text("executive")][..]));
    assert_eq!(joined.get("speaker.teams"), Some(&[text("web")][..]));
    // No org holds acme.com.
    assert_eq!(joined.get("speaker.org"), None);

    // A speaker joins the orgs of its person's addresses.
    let people = People::read(
        r#"
[[person]]
name = "Kim Lee"
addresses = ["kim@globex.com"]
names = ["Kim"]

[[org]]
name = "Globex"
domains = ["globex.com"]
client = true
"#,
    )
    .expect("a people list");
    let joined = people.join(&Facts::new(), &read(json!({"speaker": "kim"})));
    assert_eq!(joined.get("speaker.org"), Some(&[text("Globex")][..]));
    assert_eq!(joined.get("speaker.client"), Some(&[Value::Bool(true)][..]));
}

#[test]
fn a_name_two_persons_hold_joins_neither() {
    let people = People::read(
        r#"
[[person]]
name = "Dana Reyes"
names = ["Dana"]
role = "executive"

[[person]]
name = "Dana Kim"
names = ["Dana"]
"#,
    )
    .expect("a people list");
    let joined = people.join(&Facts::new(), &read(json!({"speaker": "Dana"})));
    assert!(joined.is_empty(), "{joined:?}");
}

#[test]
fn an_attendee_with_no_address_joins_nothing() {
    let joined = c9().join(&stated(json!({"attendee": "Dana Reyes"})), &Facts::new());
    assert!(joined.is_empty(), "{joined:?}");
    for given in ["dana@", "@acme.com", "dana @acme.com", "a@b@acme.com"] {
        let joined = c9().join(&stated(json!({"attendee": given})), &Facts::new());
        assert!(joined.is_empty(), "{given}: {joined:?}");
    }
}

#[test]
fn a_domain_two_orgs_hold_joins_neither() {
    let people = People::read(
        r#"
[[person]]
name = "Lee Park"
addresses = ["lee@acme.com"]

[[org]]
name = "Acme"
domains = ["acme.com"]

[[org]]
name = "Acme Labs"
domains = ["ACME.com"]
client = true
"#,
    )
    .expect("a people list");
    let joined = people.join(&stated(json!({"attendee": "lee@acme.com"})), &Facts::new());
    assert_eq!(joined.get("attendee.name"), Some(&[text("Lee Park")][..]));
    assert_eq!(joined.get("attendee.org"), None);
    assert_eq!(joined.get("attendee.client"), None);
}
