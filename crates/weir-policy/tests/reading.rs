//! Reading a policy gives the first rule whose match holds, and a capture
//! as it lands is read by rules on stated facts alone (117, 124, 125; I10).

use jiff::Timestamp;
use serde_json::json;
use weir_policy::{Facts, People, Policy, Priority, Stated};

const NOW: &str = "2026-11-12T10:05:00Z";

fn now() -> Timestamp {
    NOW.parse().expect("a time")
}

fn policy(settings: &str) -> Policy {
    Policy::read(settings).unwrap_or_else(|error| panic!("{error}"))
}

fn stated(facts: serde_json::Value) -> Facts {
    Facts::stated(&facts).expect("stated facts")
}

fn read(facts: serde_json::Value) -> Facts {
    Facts::read(&facts).expect("read facts")
}

fn capture<'a>(facts: &'a Facts, source: &'a str, title: Option<&'a str>) -> Stated<'a> {
    Stated {
        facts,
        source,
        title,
        event_at: "2026-11-12T09:00:00+01:00",
    }
}

fn name(rule: Option<&weir_policy::Rule>) -> Option<&str> {
    rule.map(|rule| rule.name.as_str())
}

const TWO: &str = r#"
[[rules]]
name = "patches"
when = { fact = "level", is = "patch" }
drop = "a patch"

[[rules]]
name = "everything"
when = { all = [] }
person = "a look"
priority = "normal"
"#;

#[test]
fn the_first_rule_in_order() {
    let policy = policy(TWO);
    let facts = stated(json!({"level": "patch"}));
    let capture = capture(&facts, "release", Some("x 1.2.1"));
    let people = People::default();
    assert_eq!(
        name(policy.for_signal(&capture, &Facts::new(), &people, now())),
        Some("patches")
    );
    assert_eq!(
        name(policy.for_capture(&capture, &people, now())),
        Some("patches")
    );
    // The same policy, people list, facts and time give the same rule.
    for _ in 0..3 {
        assert_eq!(
            name(policy.for_signal(&capture, &Facts::new(), &people, now())),
            Some("patches")
        );
    }
    let other = stated(json!({"level": "major"}));
    assert_eq!(
        name(policy.for_capture(&capture_of(&other), &people, now())),
        Some("everything")
    );
}

fn capture_of(facts: &Facts) -> Stated<'_> {
    capture(facts, "release", None)
}

#[test]
fn no_rule_matches() {
    let policy = policy(&TWO.replace("when = { all = [] }", "when = { older = \"1d\" }"));
    let facts = stated(json!({"level": "major"}));
    let capture = capture(&facts, "release", None);
    // Its event time is an hour and five minutes before now.
    assert_eq!(
        policy.for_signal(&capture, &Facts::new(), &People::default(), now()),
        None
    );
    let later: Timestamp = "2026-11-13T09:00:00Z".parse().expect("a time");
    assert_eq!(
        name(policy.for_signal(&capture, &Facts::new(), &People::default(), later)),
        Some("everything")
    );
}

#[test]
fn a_facts_own_priority_is_matched_like_any_other() {
    let policy = policy(
        r#"
[[rules]]
name = "said-urgent"
when = { fact = "priority", is = "urgent" }
person = "a look"
priority = "low"
"#,
    );
    let facts = stated(json!({"priority": "urgent"}));
    let rule = policy
        .for_signal(
            &capture(&facts, "alert", None),
            &Facts::new(),
            &People::default(),
            now(),
        )
        .expect("a rule");
    assert_eq!(rule.name, "said-urgent");
    assert_eq!(rule.priority, Some(Priority::Low));
}

#[test]
fn a_speaker_two_persons_name_matches_no_rule() {
    let policy = policy(
        r#"
[[rules]]
name = "an-executive"
when = { fact = "speaker.role", is = "executive" }
person = "an executive's word"
priority = "high"
"#,
    );
    let people = People::read(
        r#"
[[person]]
name = "Dana Reyes"
names = ["Dana"]
role = "executive"

[[person]]
name = "Dana Kim"
names = ["Dana", "Dana Kim"]
role = "executive"
"#,
    )
    .expect("a people list");
    let facts = Facts::new();
    let capture = capture(&facts, "meeting", Some("Planning"));
    assert_eq!(
        policy.for_signal(&capture, &read(json!({"speaker": "Dana"})), &people, now()),
        None
    );
    assert_eq!(
        name(policy.for_signal(
            &capture,
            &read(json!({"speaker": "Dana Kim"})),
            &people,
            now()
        )),
        Some("an-executive")
    );
}

#[test]
fn a_rule_on_a_read_fact_is_passed_over_as_a_capture_lands() {
    let policy = policy(
        r#"
[[rules]]
name = "nothing-said"
when = { not = { fact = "says", exists = true } }
drop = "nothing said"

[[rules]]
name = "stand-ups"
when = { fact = "title", contains = "stand-up" }
drop = "a stand-up"
"#,
    );
    let facts = stated(json!({"length": 900}));
    let capture = capture(&facts, "meeting", Some("Daily stand-up"));
    let people = People::default();
    assert!(policy.rules()[0].names_a_read_fact());
    assert!(!policy.rules()[1].names_a_read_fact());
    assert_eq!(
        name(policy.for_capture(&capture, &people, now())),
        Some("stand-ups")
    );
    assert_eq!(
        name(policy.for_signal(&capture, &Facts::new(), &people, now())),
        Some("nothing-said")
    );
}

#[test]
fn a_rule_on_a_speakers_joined_fact_is_passed_over() {
    let policy = policy(
        r#"
[[rules]]
name = "a-clients-word"
when = { any = [{ fact = "speaker.client", is = true }, { fact = "source", is = "meeting" }] }
person = "a client's word"
priority = "high"
"#,
    );
    assert!(policy.rules()[0].names_a_read_fact());
    let facts = stated(json!({"attendee": ["dana@acme.com"]}));
    let capture = capture(&facts, "meeting", Some("Planning"));
    let people = People::default();
    assert_eq!(policy.for_capture(&capture, &people, now()), None);
    assert_eq!(
        name(policy.for_signal(&capture, &Facts::new(), &people, now())),
        Some("a-clients-word")
    );
}
