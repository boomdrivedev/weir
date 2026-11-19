//! A rule's item closed at its window, with its brief of its signals'
//! excerpts as they stand, held to the scenarios of inbox-items (36, 128,
//! 129; C.2; core.20, core.21).

mod common;

use weir_capture::{Capture, Nature, OwnSignal, RuleMove, RuleOutcome, Taken};
use weir_policy::Priority;
use weir_signals::{InboxAt, Write};

use common::{after, capture, empty, paths, recfix};

const FIRST: &str = "point-releases-20261112T103000Z";
const NEXT: &str = "point-releases-20261112T110000Z";
const CLOSED_AT: &str = "2026-11-12T10:30:04Z";

/// A release under `key`, released at `event_at`, its own excerpt
/// `excerpt`, joined by `point-releases` into `item` where it has no
/// current item.
fn release(key: &str, event_at: &str, excerpt: &str, item: &str) -> Taken {
    Taken {
        capture: Capture {
            unread: false,
            event_at: event_at.into(),
            taken_at: "2026-11-12T10:05:00Z".into(),
            ..capture(key)
        },
        own_signal: Some(OwnSignal {
            signal: format!("{key}#1"),
            kind: Some(Nature::Ask),
            asserted_by: "acme/web-app".into(),
            assertion: format!("{key} is released"),
            subjects: vec![],
            argues_with: vec![],
            position: "release notes".into(),
            excerpt: excerpt.into(),
            cut: None,
        }),
        rule_move: Some(RuleMove {
            rule: "point-releases".into(),
            outcome: RuleOutcome::Join {
                priority: Priority::Low,
                item: item.into(),
            },
        }),
    }
}

fn took(inbox: &InboxAt, taken: &Taken) -> InboxAt {
    after(
        inbox,
        &inbox
            .take(taken)
            .unwrap_or_else(|refused| panic!("{refused}")),
    )
}

/// Two patch releases in the rule's item, the later-released taken first.
fn two_releases() -> InboxAt {
    let inbox = took(
        &empty(),
        &release(
            "release/mise/2026.11.2",
            "2026-11-12T09:00:00Z",
            "mise 2026.11.2\n\n- fixes the shim\n",
            FIRST,
        ),
    );
    took(
        &inbox,
        &release(
            "release/mise/2026.11.1",
            "2026-11-11T09:00:00Z",
            "mise 2026.11.1",
            FIRST,
        ),
    )
}

#[test]
fn a_rules_item_closes_with_its_brief() {
    let inbox = two_releases();
    let write = inbox.close_rule_item(FIRST, CLOSED_AT).unwrap();
    assert_eq!(
        paths(&write),
        [
            format!("inbox/items/{FIRST}/brief.md"),
            format!("inbox/items/{FIRST}/item.rec"),
        ]
    );
    let record = &write.files[&format!("inbox/items/{FIRST}/item.rec")];
    recfix(record);
    assert!(record.ends_with(&format!("closed_at: {CLOSED_AT}\n")));
    assert_eq!(
        write.files[&format!("inbox/items/{FIRST}/brief.md")],
        "<!-- format: weir-brief/1 -->\nmise 2026.11.1\n\nmise 2026.11.2\n\n- fixes the shim\n"
    );
    let closed = after(&inbox, &write);
    let item = closed.item(FIRST).unwrap();
    assert_eq!(item.item.closed_at.as_deref(), Some(CLOSED_AT));
    assert_eq!(
        item.brief.as_deref(),
        Some("mise 2026.11.1\n\nmise 2026.11.2\n\n- fixes the shim\n")
    );
    assert!(closed.rule_item("point-releases").is_none());
    assert_eq!(closed.item_signals(FIRST).len(), 2);
}

#[test]
fn a_closed_items_rule_makes_its_next_item() {
    let inbox = two_releases();
    let closed = after(&inbox, &inbox.close_rule_item(FIRST, CLOSED_AT).unwrap());
    let next = took(
        &closed,
        &release(
            "release/mise/2026.11.3",
            "2026-11-12T10:40:00Z",
            "mise 2026.11.3",
            NEXT,
        ),
    );
    assert_eq!(next.rule_item("point-releases").unwrap().item.item, NEXT);
    assert_eq!(next.item_signals(FIRST).len(), 2);
    assert_eq!(next.item_signals(NEXT).len(), 1);
}

#[test]
fn an_item_closes_once() {
    let inbox = two_releases();
    let closed = after(&inbox, &inbox.close_rule_item(FIRST, CLOSED_AT).unwrap());
    let refused = closed
        .close_rule_item(FIRST, "2026-11-12T11:00:00Z")
        .unwrap_err();
    assert_eq!(refused.0[0].field.as_deref(), Some("closed_at"));
}

#[test]
fn an_item_no_rule_made_does_not_close() {
    let inbox = empty();
    let inbox = after(
        &inbox,
        &inbox
            .propose_item("pin-mise", "inquiry/toolchain#3", "operator", CLOSED_AT)
            .unwrap(),
    );
    let refused = inbox.close_rule_item("pin-mise", CLOSED_AT).unwrap_err();
    assert_eq!(refused.0[0].field.as_deref(), Some("rule"));
    let refused = inbox
        .close_rule_item("no-such-item", CLOSED_AT)
        .unwrap_err();
    assert_eq!(refused.0[0].what, "no-such-item");
}

#[test]
fn a_close_at_no_time() {
    let refused = two_releases()
        .close_rule_item(FIRST, "half past ten")
        .unwrap_err();
    assert_eq!(refused.0[0].field.as_deref(), Some("closed_at"));
}

#[test]
fn an_item_record_changed_otherwise_is_refused() {
    let inbox = two_releases();
    let path = format!("inbox/items/{FIRST}/item.rec");
    let text = inbox.files()[&path].replace("priority: low", "priority: high");
    let mut write = Write::new();
    write.put(&path, format!("{text}closed_at: {CLOSED_AT}\n"));
    let refused = inbox.check(&write).unwrap_err();
    assert_eq!(refused.0[0].what, path);
}
