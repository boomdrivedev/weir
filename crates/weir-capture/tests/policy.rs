//! A take given a policy reads it in the capture's write, held to the
//! scenarios of capture-take, through the fakes of a host's inbox and bank
//! (125 to 128; C.1 step 5, C.9; core.13, core.14, core.16, core.58).

mod fakes;

use std::io::Write as _;
use std::process::Command;

use jiff::tz::TimeZone;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;
use weir_capture::take::{take, OwnSignal, Take, Taken};
use weir_capture::{CameIn, Capture, Nature, RuleMove, RuleOutcome};
use weir_policy::{People, Policy, Priority, Value};

use fakes::host;

const C9: &str = include_str!("../../weir-policy/tests/c9.toml");

const TAKEN_AT: &str = "2026-11-12T10:05:00Z";

fn bare_take(came_in: CameIn) -> Take {
    let at = OffsetDateTime::parse(TAKEN_AT, &Rfc3339).unwrap();
    Take::new("deneb", at, 8192, came_in).unwrap()
}

/// A take on deneb at 10:05, given `settings`' policy and people list, read
/// in UTC.
fn take_with(settings: &str, came_in: CameIn) -> Take {
    bare_take(came_in).with_policy(
        Policy::read(settings).expect("a policy"),
        People::read(settings).expect("a people list"),
        TimeZone::UTC,
    )
}

/// What one write hands the inbox of a take of `lines`.
fn written(lines: &[String], how: &Take) -> Vec<Taken> {
    let (mut inbox, mut bank, _) = host();
    let report = take(lines, None, how, &mut inbox, &mut bank);
    assert_eq!(report.refused, [], "{report:?}");
    inbox.written
}

/// A release's offer with an own excerpt, stating `level`.
fn release(key: &str, level: &str) -> String {
    format!(
        r#"{{"capture": {{"key": "{key}", "source": "release", "event_at": "2026-11-12T09:58:00Z", "title": "{key}", "facts": {{"level": "{level}"}}, "excerpt": {{"text": "{key} is out.", "kind": "ask", "asserted_by": "acme/web-app", "assertion": "{key} is released", "subjects": [], "argues_with": [], "position": "release notes"}}}}}}"#
    )
}

/// A meeting's offer titled `title`, with no excerpt, stating `facts`.
fn meeting(key: &str, title: &str, facts: &str) -> String {
    format!(
        r#"{{"capture": {{"key": "{key}", "source": "meeting", "event_at": "2026-11-12T09:00:00Z", "title": "{title}", "facts": {facts}}}}}"#
    )
}

fn stand_up() -> String {
    meeting(
        "meeting/file/standup",
        "Daily stand-up",
        r#"{"length": 900, "attendee": ["dana@acme.com", "kim@globex.com", "lee@acme.com"]}"#,
    )
}

#[test]
fn a_patch_release_joins_its_rules_item() {
    let written = written(
        &[release("release/web-app/1.2.1", "patch")],
        &take_with(C9, CameIn::Run("releases".into())),
    );
    let [taken] = written.as_slice() else {
        panic!("one write: {written:?}");
    };
    assert!(!taken.capture.unread);
    let own = taken.own_signal.as_ref().expect("its own signal");
    assert_eq!(own.signal, "release/web-app/1.2.1#1");
    assert_eq!(own.kind, Some(Nature::Ask));
    assert_eq!(
        taken.rule_move,
        Some(RuleMove {
            rule: "point-releases".into(),
            outcome: RuleOutcome::Join {
                priority: Priority::Low,
                item: "point-releases-20261112T103000Z".into(),
            },
        })
    );
}

#[test]
fn a_signal_no_rule_matches_stays_unmoved() {
    let written = written(
        &[release("release/web-app/2.0.0", "major")],
        &take_with(C9, CameIn::Run("releases".into())),
    );
    let [taken] = written.as_slice() else {
        panic!("one write: {written:?}");
    };
    assert!(!taken.capture.unread);
    assert_eq!(
        taken.own_signal.as_ref().map(|own| own.kind),
        Some(Some(Nature::Ask))
    );
    assert_eq!(taken.rule_move, None);
}

#[test]
fn a_take_given_no_policy() {
    let written = written(&[stand_up()], &bare_take(CameIn::Run("meetings".into())));
    let [taken] = written.as_slice() else {
        panic!("one write: {written:?}");
    };
    assert!(taken.capture.unread);
    assert_eq!(taken.own_signal, None);
    assert_eq!(taken.rule_move, None);
}

#[test]
fn a_stand_up_dropped_as_it_lands() {
    let written = written(
        &[stand_up()],
        &take_with(C9, CameIn::Run("meetings".into())),
    );
    let [taken] = written.as_slice() else {
        panic!("one write: {written:?}");
    };
    assert!(!taken.capture.unread);
    assert_eq!(
        taken.own_signal,
        Some(OwnSignal {
            signal: "meeting/file/standup#1".into(),
            kind: None,
            asserted_by: "meeting".into(),
            assertion: "Daily stand-up".into(),
            subjects: vec![],
            argues_with: vec![],
            position: "title".into(),
            excerpt: "Daily stand-up".into(),
            cut: None,
        })
    );
    assert_eq!(
        taken.rule_move,
        Some(RuleMove {
            rule: "stand-ups".into(),
            outcome: RuleOutcome::Drop {
                reason: "a stand-up".into(),
            },
        })
    );
}

#[test]
fn a_capture_with_no_title_is_its_own_signal_by_its_key() {
    let settings = r#"
[[rules]]
name = "deploys"
when = { fact = "source", is = "deploy" }
drop = "a deploy"
"#;
    let line = r#"{"capture": {"key": "deploy/web/42", "source": "deploy", "event_at": "2026-11-12T10:00:00Z"}}"#;
    let written = written(&[line.to_string()], &take_with(settings, CameIn::Intake));
    let [taken] = written.as_slice() else {
        panic!("one write: {written:?}");
    };
    let own = taken.own_signal.as_ref().expect("its own signal");
    assert_eq!(own.excerpt, "deploy/web/42");
    assert_eq!(own.assertion, "deploy/web/42");
    assert_eq!(own.asserted_by, "deploy");
    assert_eq!(own.kind, None);
    assert!(!taken.capture.unread);
}

#[test]
fn a_rule_on_a_read_fact_charges_a_reader() {
    let settings = r#"
[[rules]]
name = "requests"
when = { fact = "says", is = "request" }
person = "a request"
priority = "normal"
"#;
    let written = written(
        &[stand_up()],
        &take_with(settings, CameIn::Run("meetings".into())),
    );
    let [taken] = written.as_slice() else {
        panic!("one write: {written:?}");
    };
    assert!(taken.capture.unread);
    assert_eq!(taken.own_signal, None);
    assert_eq!(taken.rule_move, None);
}

#[test]
fn two_hosts_one_policy() {
    let lines = [
        stand_up(),
        release("release/web-app/1.2.1", "patch"),
        release("release/web-app/1.2.2", "patch"),
    ];
    let how = || take_with(C9, CameIn::Run("feeds".into()));
    let one = written(&lines, &how());
    let other = written(&lines, &how());
    assert_eq!(one.len(), 3);
    assert_eq!(one, other);
    let records = |written: &[Taken]| -> Vec<String> {
        written.iter().map(|taken| taken.capture.to_rec()).collect()
    };
    assert_eq!(records(&one), records(&other));
    let moves: Vec<&str> = one
        .iter()
        .map(|taken| {
            taken
                .rule_move
                .as_ref()
                .expect("a rule's move")
                .rule
                .as_str()
        })
        .collect();
    assert_eq!(moves, ["stand-ups", "point-releases", "point-releases"]);
}

#[test]
fn a_captures_stated_facts_are_written_with_it() {
    let line = meeting(
        "meeting/file/planning",
        "Planning",
        r#"{"length": 900, "attendee": ["dana@acme.com"]}"#,
    );
    let written = written(&[line], &take_with(C9, CameIn::Run("meetings".into())));
    let [taken] = written.as_slice() else {
        panic!("one write: {written:?}");
    };
    let record = taken.capture.to_rec();
    assert!(
        record.ends_with("unread: yes\nfact: attendee \"dana@acme.com\"\nfact: length 900\n"),
        "{record}"
    );
    assert!(!record.contains("attendee."), "{record}");
    recfix_check(&record);
    let read = Capture::from_rec(&record).expect("the record read back");
    assert_eq!(read, taken.capture);
    assert_eq!(
        read.facts.get("attendee"),
        Some(&[Value::Text("dana@acme.com".into())][..])
    );
}

/// Holds `record` to `recfix --check`, as a write is held before it lands
/// (core.24).
fn recfix_check(record: &str) {
    let mut file = tempfile::NamedTempFile::new().expect("a temporary file");
    file.write_all(record.as_bytes())
        .expect("the record written");
    file.flush().expect("the record flushed");
    let output = Command::new("recfix")
        .arg("--check")
        .arg(file.path())
        .output()
        .unwrap_or_else(|error| panic!("recfix is not on PATH: {error}"));
    assert!(
        output.status.success(),
        "recfix refused:\n{record}\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}
