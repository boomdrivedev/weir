//! A rule's move, and the item it makes, written in its capture's write,
//! held to the scenarios of inbox-signals and inbox-items (19, 29 to 32,
//! 35, 126 to 128; C.2; core.13, core.18, core.19, core.26, core.60).

mod common;

use jiff::tz::TimeZone;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;
use weir_capture::rec;
use weir_capture::take::{take, Bank, Inbox, RawFile, Refusal, Take};
use weir_capture::{CameIn, Capture, Key, Nature, OwnSignal, RuleMove, RuleOutcome, Taken};
use weir_policy::{People, Policy, Priority};
use weir_signals::{InboxAt, Item, MadeBy, Refused, Word};

use common::{after, capture, empty, paths, recfix, signal};

const TAKEN_AT: &str = "2026-11-12T10:05:00Z";
const FIRST: &str = "point-releases-20261112T103000Z";
const NEXT: &str = "point-releases-20261112T110000Z";

/// The capture `key`, taken at 10:05 and not unread.
fn taken_capture(key: &str) -> Capture {
    Capture {
        unread: false,
        taken_at: TAKEN_AT.into(),
        ..capture(key)
    }
}

/// A release under `key` with its own excerpt's signal, of nature `ask`,
/// and `rule_move`.
fn release(key: &str, rule_move: Option<RuleMove>) -> Taken {
    Taken {
        capture: taken_capture(key),
        own_signal: Some(OwnSignal {
            signal: format!("{key}#1"),
            kind: Some(Nature::Ask),
            asserted_by: "acme/web-app".into(),
            assertion: format!("{key} is released"),
            subjects: vec![],
            argues_with: vec![],
            position: "release notes".into(),
            excerpt: format!("{key} is out."),
            cut: None,
        }),
        rule_move,
    }
}

/// A meeting under `key` titled `title`, with no excerpt, its rule's own
/// signal of `excerpt`, and `rule_move`.
fn meeting(key: &str, title: &str, excerpt: &str, rule_move: Option<RuleMove>) -> Taken {
    Taken {
        capture: Capture {
            source: "meeting".into(),
            title: Some(title.into()),
            ..taken_capture(key)
        },
        own_signal: Some(OwnSignal {
            signal: format!("{key}#1"),
            kind: None,
            asserted_by: "meeting".into(),
            assertion: excerpt.into(),
            subjects: vec![],
            argues_with: vec![],
            position: "title".into(),
            excerpt: excerpt.into(),
            cut: None,
        }),
        rule_move,
    }
}

fn join(item: &str) -> Option<RuleMove> {
    Some(RuleMove {
        rule: "point-releases".into(),
        outcome: RuleOutcome::Join {
            priority: Priority::Low,
            item: item.into(),
        },
    })
}

fn stand_ups() -> Option<RuleMove> {
    Some(RuleMove {
        rule: "stand-ups".into(),
        outcome: RuleOutcome::Drop {
            reason: "a stand-up".into(),
        },
    })
}

fn refused(inbox: &InboxAt, taken: &Taken) -> Refused {
    match inbox.take(taken) {
        Ok(write) => panic!("written: {write:?}"),
        Err(refused) => refused,
    }
}

/// The inbox after `taken` is taken on `inbox`.
fn took(inbox: &InboxAt, taken: &Taken) -> InboxAt {
    after(
        inbox,
        &inbox
            .take(taken)
            .unwrap_or_else(|refused| panic!("{refused}")),
    )
}

/// `inbox` with its rule's item `name` closed, as the close writes it: the
/// tip's files with `closed_at` added to the item's record.
fn closed(inbox: &InboxAt, name: &str) -> InboxAt {
    let mut files = inbox.files().clone();
    let path = format!("inbox/items/{name}/item.rec");
    let text = files.get_mut(&path).expect("the item's record");
    text.push_str("closed_at: 2026-11-12T10:30:00Z\n");
    recfix(text);
    InboxAt::read(files).unwrap_or_else(|error| panic!("{error}"))
}

#[test]
fn a_rules_drop_in_the_captures_write() {
    let inbox = empty();
    let taken = meeting(
        "meeting/file/standup",
        "Daily stand-up",
        "Daily stand-up",
        stand_ups(),
    );
    let write = inbox.take(&taken).unwrap();
    assert_eq!(
        paths(&write),
        [
            "inbox/captures/meeting/file/standup/capture.rec",
            "inbox/captures/meeting/file/standup/signals.rec",
            "inbox/moves/moves.rec",
        ]
    );
    let inbox = after(&inbox, &write);
    let (_, standing) = inbox
        .standing(&signal("meeting/file/standup#1"))
        .expect("a standing move");
    assert_eq!(standing.word, Word::Drop);
    assert_eq!(standing.target, "a stand-up");
    assert_eq!(standing.note.as_deref(), Some("a stand-up"));
    assert_eq!(standing.by, "rule:stand-ups");
    assert!(!inbox.is_unread(&taken.capture.key));
    assert_eq!(
        inbox
            .signal(&signal("meeting/file/standup#1"))
            .unwrap()
            .kind,
        None
    );
}

#[test]
fn a_rules_join_makes_its_item_in_the_captures_write() {
    let inbox = empty();
    let write = inbox
        .take(&release("release/web-app/1.2.1", join(FIRST)))
        .unwrap();
    assert_eq!(
        paths(&write),
        [
            "inbox/captures/release/web-app/1.2.1/capture.rec",
            "inbox/captures/release/web-app/1.2.1/signals.rec",
            &format!("inbox/items/{FIRST}/item.rec"),
            "inbox/moves/moves.rec",
        ]
    );
    let inbox = after(&inbox, &write);
    assert_eq!(
        inbox.item_signals(FIRST),
        [&signal("release/web-app/1.2.1#1")]
    );
}

#[test]
fn a_rules_item_record() {
    let inbox = empty();
    let write = inbox
        .take(&release("release/web-app/1.2.1", join(FIRST)))
        .unwrap();
    let text = &write.files[&format!("inbox/items/{FIRST}/item.rec")];
    assert_eq!(
        text,
        &format!(
            "format: weir-item/1\n\
             item: {FIRST}\n\
             made_by: join\n\
             named_by: rule:point-releases\n\
             named_at: {TAKEN_AT}\n\
             rule: point-releases\n\
             priority: low\n"
        )
    );
    recfix(text);
    let inbox = after(&inbox, &write);
    let item = &inbox.item(FIRST).unwrap().item;
    assert_eq!(
        item,
        &Item {
            item: FIRST.into(),
            made_by: MadeBy::Join,
            named_by: "rule:point-releases".into(),
            named_at: TAKEN_AT.into(),
            rule: Some("point-releases".into()),
            priority: Some(Priority::Low),
            closed_at: None,
        }
    );
    assert_eq!(&rec::write(&[item.to_record()]), text);
}

#[test]
fn a_rules_move_names_its_rule() {
    let inbox = took(&empty(), &release("release/x/1.2.1", join(FIRST)));
    let (number, standing) = inbox
        .standing(&signal("release/x/1.2.1#1"))
        .expect("a standing move");
    assert_eq!(number, 1);
    assert_eq!(standing.word, Word::Join);
    assert_eq!(standing.target, FIRST);
    assert_eq!(standing.by, "rule:point-releases");
    assert_eq!(standing.at, TAKEN_AT);
    assert_eq!(standing.batch, None);
    let moves = &inbox.files()["inbox/moves/moves.rec"];
    assert!(moves.contains("by: rule:point-releases\n"), "{moves}");
    assert!(!moves.contains("batch:"), "{moves}");
    recfix(moves);
}

#[test]
fn a_rules_move_with_no_signal() {
    let taken = Taken {
        capture: taken_capture("meeting/file/standup"),
        own_signal: None,
        rule_move: stand_ups(),
    };
    let refused = refused(&empty(), &taken);
    assert_eq!(refused.0.len(), 1);
    assert_eq!(refused.0[0].what, "meeting/file/standup");
    assert_eq!(refused.0[0].field.as_deref(), Some("signal"));
}

#[test]
fn a_signal_of_no_nature_with_no_rules_move() {
    let taken = meeting(
        "meeting/file/standup",
        "Daily stand-up",
        "Daily stand-up",
        None,
    );
    let refused = refused(&empty(), &taken);
    assert_eq!(refused.0[0].what, "meeting/file/standup");
    assert_eq!(refused.0[0].field.as_deref(), Some("signal"));
}

#[test]
fn a_rules_signal_whose_excerpt_is_not_the_title() {
    let taken = meeting(
        "meeting/file/standup",
        "Daily stand-up",
        "Weekly stand-up",
        stand_ups(),
    );
    let refused = refused(&empty(), &taken);
    assert_eq!(refused.0[0].field.as_deref(), Some("excerpt"), "{refused}");
}

#[test]
fn a_closed_item_takes_no_more_joins() {
    let inbox = took(&empty(), &release("release/x/1.2.1", join(FIRST)));
    let inbox = closed(&inbox, FIRST);
    let write = inbox.take(&release("release/x/1.2.2", join(NEXT))).unwrap();
    assert!(
        write
            .files
            .contains_key(&format!("inbox/items/{NEXT}/item.rec")),
        "{:?}",
        paths(&write)
    );
    let inbox = after(&inbox, &write);
    assert_eq!(inbox.item_signals(NEXT), [&signal("release/x/1.2.2#1")]);
    assert_eq!(inbox.item_signals(FIRST), [&signal("release/x/1.2.1#1")]);
    assert_eq!(
        inbox
            .rule_item("point-releases")
            .map(|at| at.item.item.as_str()),
        Some(NEXT)
    );
}

#[test]
fn two_writers_of_one_rule_meet_one_item() {
    // Another writer made the rule's item at the tip this join lands on.
    let inbox = took(&empty(), &release("release/x/1.2.1", join(FIRST)));
    let write = inbox.take(&release("release/x/1.2.2", join(NEXT))).unwrap();
    assert!(!paths(&write)
        .iter()
        .any(|path| path.starts_with("inbox/items/")));
    let inbox = after(&inbox, &write);
    assert!(inbox.item(NEXT).is_none());
    assert_eq!(
        inbox.item_signals(FIRST),
        [&signal("release/x/1.2.1#1"), &signal("release/x/1.2.2#1")]
    );
}

#[test]
fn a_rules_item_under_a_used_name() {
    let inbox = took(&empty(), &release("release/x/1.2.1", join(FIRST)));
    let inbox = closed(&inbox, FIRST);
    let not_taken = refused(&inbox, &release("release/x/1.2.2", join(FIRST)));
    assert_eq!(not_taken.0.len(), 1);
    assert_eq!(not_taken.0[0].what, FIRST);
    // And an item another made under that name.
    let concluded = after(
        &empty(),
        &empty()
            .propose_item(FIRST, "inquiry/x#1", "operator", TAKEN_AT)
            .unwrap(),
    );
    let not_taken = refused(&concluded, &release("release/x/1.2.2", join(FIRST)));
    assert_eq!(not_taken.0[0].what, FIRST);
}

const C9: &str = include_str!("../../weir-policy/tests/c9.toml");

/// An inbox a host's `take` writes through, its tip the files each write
/// leaves, read again after each write.
struct Tip(InboxAt);

impl Inbox for Tip {
    fn holds(&self, key: &Key) -> bool {
        self.0.holds(key)
    }

    fn write(&mut self, taken: &Taken) -> Result<(), Refusal> {
        let write = self.0.take(taken).map_err(|r| Refusal(r.to_string()))?;
        self.0 = after(&self.0, &write);
        Ok(())
    }
}

/// A bank no offer here reaches: none names raw material or is cut.
struct NoBank;

impl Bank for NoBank {
    fn bank(&mut self, key: &Key, _: &[RawFile], _: Option<&str>) -> Result<String, Refusal> {
        panic!("{key} was banked")
    }

    fn discard(&mut self, key: &Key) {
        panic!("{key} was discarded")
    }
}

/// Takes `line` at `at` through `tip`, given C.9's policy and people list,
/// read in UTC.
fn take_c9(tip: &mut Tip, line: &str, at: &str) {
    let how = Take::new(
        "deneb",
        OffsetDateTime::parse(at, &Rfc3339).unwrap(),
        8192,
        CameIn::Run("feeds".into()),
    )
    .unwrap()
    .with_policy(
        Policy::read(C9).expect("C.9's policy"),
        People::read(C9).expect("C.9's people list"),
        TimeZone::UTC,
    );
    let report = take([line], None, &how, tip, &mut NoBank);
    assert_eq!(report.refused, [], "{report:?}");
    assert_eq!(report.taken.len(), 1, "{report:?}");
}

/// A patch release's offer under `key`, with its notes as its own excerpt.
fn patch_release(key: &str) -> String {
    format!(
        r#"{{"capture": {{"key": "{key}", "source": "release", "event_at": "2026-11-12T10:00:00Z", "title": "{key}", "facts": {{"level": "patch"}}, "excerpt": {{"text": "{key} is out.", "kind": "ask", "asserted_by": "acme/web-app", "assertion": "{key} is released", "subjects": [], "argues_with": [], "position": "release notes"}}}}}}"#
    )
}

#[test]
fn a_patch_release_then_another_ten_minutes_later() {
    let mut tip = Tip(empty());
    take_c9(
        &mut tip,
        &patch_release("release/web-app/1.2.1"),
        "2026-11-12T10:05:00Z",
    );
    let item_path = format!("inbox/items/{FIRST}/item.rec");
    let made = tip.0.files()[&item_path].clone();
    let item = &tip.0.item(FIRST).expect("the rule's item").item;
    assert_eq!(item.rule.as_deref(), Some("point-releases"));
    assert_eq!(item.priority, Some(Priority::Low));
    assert_eq!(item.named_by, "rule:point-releases");

    take_c9(
        &mut tip,
        &patch_release("release/web-app/1.2.2"),
        "2026-11-12T10:15:00Z",
    );
    assert_eq!(tip.0.files()[&item_path], made);
    assert_eq!(
        tip.0.item_signals(FIRST),
        [
            &signal("release/web-app/1.2.1#1"),
            &signal("release/web-app/1.2.2#1")
        ]
    );
    let items: Vec<&str> = tip
        .0
        .files()
        .keys()
        .filter(|path| path.ends_with("/item.rec"))
        .map(String::as_str)
        .collect();
    assert_eq!(items, [item_path.as_str()]);
}

#[test]
fn a_daily_stand_up_dropped_as_it_lands() {
    let mut tip = Tip(empty());
    let line = r#"{"capture": {"key": "meeting/file/standup", "source": "meeting", "event_at": "2026-11-12T09:00:00Z", "title": "Daily stand-up", "facts": {"length": 900, "attendee": ["dana@acme.com", "kim@globex.com", "lee@acme.com"]}}}"#;
    let before = tip.0.files().clone();
    take_c9(&mut tip, line, "2026-11-12T10:05:00Z");
    let added: Vec<&str> = tip
        .0
        .files()
        .keys()
        .filter(|path| !before.contains_key(*path))
        .map(String::as_str)
        .collect();
    assert_eq!(
        added,
        [
            "inbox/captures/meeting/file/standup/capture.rec",
            "inbox/captures/meeting/file/standup/signals.rec",
            "inbox/moves/moves.rec",
        ]
    );
    let key = Key::parse("meeting/file/standup").unwrap();
    // No reader is charged with the capture: it is not unread.
    assert!(!tip.0.is_unread(&key));
    assert!(!tip.0.capture(&key).unwrap().unread);
    let [own] = tip.0.signals(&key).expect("its own signal") else {
        panic!("one signal");
    };
    assert_eq!(own.kind, None);
    assert_eq!(own.excerpt, "Daily stand-up");
    assert_eq!(own.assertion, "Daily stand-up");
    let (_, standing) = tip
        .0
        .standing(&signal("meeting/file/standup#1"))
        .expect("the rule's drop");
    assert_eq!(standing.word, Word::Drop);
    assert_eq!(standing.by, "rule:stand-ups");
    assert_eq!(standing.target, "a stand-up");
    assert!(tip.0.unmoved().is_empty());
}
