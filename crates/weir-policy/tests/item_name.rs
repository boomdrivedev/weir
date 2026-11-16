//! A rule's item is named for its rule and its window's close (128, 129;
//! C.5, C.9; record 0010).

use jiff::tz::{TimeZone, TimeZoneDatabase};
use jiff::Timestamp;
use weir_policy::{Policy, Rule};

const C9: &str = include_str!("c9.toml");

fn instant(text: &str) -> Timestamp {
    text.parse().expect(text)
}

fn c9_rule(name: &str) -> Rule {
    Policy::read(C9)
        .expect("C.9's policy")
        .rules()
        .iter()
        .find(|rule| rule.name == name)
        .cloned()
        .expect(name)
}

fn windowed(window: &str) -> Rule {
    let settings = format!(
        "[[rules]]\nname = \"mornings\"\nwhen = {{ all = [] }}\nperson = \"a look\"\npriority = \"normal\"\nwindow = \"{window}\"\n"
    );
    Policy::read(&settings).expect("a policy").rules()[0].clone()
}

#[test]
fn an_item_named_for_its_windows_next_time() {
    assert_eq!(
        c9_rule("point-releases").item_name(instant("2026-11-12T10:05:00Z"), &TimeZone::UTC),
        "point-releases-20261112T103000Z"
    );
}

#[test]
fn an_item_of_a_rule_with_no_window_is_named_for_its_making() {
    assert_eq!(
        c9_rule("a-clients-ask").item_name(instant("2026-11-12T10:05:07Z"), &TimeZone::UTC),
        "a-clients-ask-20261112T100507Z"
    );
}

#[test]
fn a_window_read_in_the_machines_zone() {
    let zone = TimeZoneDatabase::bundled()
        .get("America/New_York")
        .expect("a zone");
    let name = windowed("0 9 * * *").item_name(instant("2026-11-12T15:00:00Z"), &zone);
    assert!(name.ends_with("-20261113T140000Z"), "{name}");
    let name = windowed("0 9 * * *").item_name(instant("2026-11-12T15:00:00Z"), &TimeZone::UTC);
    assert!(name.ends_with("-20261113T090000Z"), "{name}");
}

#[test]
fn a_window_that_names_no_later_time() {
    let name = windowed("0 0 30 2 *").item_name(instant("2026-11-12T10:05:07Z"), &TimeZone::UTC);
    assert_eq!(name, "mornings-20261112T100507Z");
}
