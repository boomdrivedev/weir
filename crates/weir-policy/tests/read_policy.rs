//! A policy is read whole and refused naming the field (122, 123; C.9;
//! record 0010).

use weir_cadence::Schedule;
use weir_policy::{Match, Outcome, Policy, Priority, Test, Value};

const C9: &str = include_str!("c9.toml");

/// The field a policy is refused naming.
fn refused(settings: &str) -> String {
    match Policy::read(settings) {
        Ok(read) => panic!("{settings} was read: {read:?}"),
        Err(error) => error.field.expect("a field"),
    }
}

fn text(text: &str) -> Value {
    Value::Text(text.to_string())
}

fn fact(name: &str, test: Test) -> Match {
    Match::Fact {
        name: name.to_string(),
        test,
    }
}

const LAUNCH: &str = r#"
[[rules]]
name = "point-releases"
when = { fact = "level", in = ["patch", "minor"] }
kind = "update"
repository = "~/Code/acme/web-app"
priority = "low"
"#;

#[test]
fn c9s_policy_read() {
    let policy = Policy::read(C9).expect("C.9's policy");
    let names: Vec<&str> = policy
        .rules()
        .iter()
        .map(|rule| rule.name.as_str())
        .collect();
    assert_eq!(names, ["stand-ups", "point-releases", "a-clients-ask"]);

    let [stand_ups, point_releases, a_clients_ask] = policy.rules() else {
        panic!("three rules");
    };
    assert_eq!(
        stand_ups.when,
        Match::All(vec![
            fact("source", Test::Is(text("meeting"))),
            fact("title", Test::Contains("stand-up".into())),
        ])
    );
    assert_eq!(stand_ups.outcome, Outcome::Drop("a stand-up".into()));
    assert_eq!(stand_ups.priority, None);
    assert_eq!(stand_ups.window, None);

    assert_eq!(
        point_releases.when,
        fact("level", Test::In(vec![text("patch"), text("minor")]))
    );
    assert_eq!(
        point_releases.outcome,
        Outcome::Launch {
            kind: "update".into(),
            repository: "~/Code/acme/web-app".into(),
        }
    );
    assert_eq!(point_releases.priority, Some(Priority::Low));
    assert_eq!(
        point_releases.window,
        Some(Schedule::parse("*/30 * * * *").expect("a schedule"))
    );

    assert_eq!(
        a_clients_ask.outcome,
        Outcome::Person("a client's ask".into())
    );
    assert_eq!(a_clients_ask.priority, Some(Priority::Urgent));
    assert_eq!(a_clients_ask.window, None);
    assert_eq!(
        a_clients_ask.when,
        Match::All(vec![
            fact("says", Test::Is(text("request"))),
            fact("attendee.client", Test::Is(Value::Bool(true))),
            Match::Any(vec![
                fact("speaker.role", Test::Is(text("executive"))),
                fact("speaker.client", Test::Is(Value::Bool(true))),
            ]),
        ])
    );

    // A document with no rules is a policy of none.
    let none = Policy::read("[host]\nbranch = \"inbox\"\n").expect("a policy of none");
    assert!(none.rules().is_empty());
}

#[test]
fn a_rule_with_two_outcomes() {
    assert_eq!(
        refused(&format!("{LAUNCH}drop = \"not wanted\"\n")),
        "rules[0].drop"
    );
    assert_eq!(
        refused("[[rules]]\nname = \"r\"\nwhen = { all = [] }\n"),
        "rules[0]"
    );
}

#[test]
fn a_rule_named_twice() {
    assert_eq!(refused(&format!("{LAUNCH}{LAUNCH}")), "rules[1].name");
}

#[test]
fn a_rule_name_with_an_underscore() {
    assert_eq!(
        refused(&LAUNCH.replace("point-releases", "point_releases")),
        "rules[0].name"
    );
    assert_eq!(
        refused(&LAUNCH.replace("point-releases", "Point-releases")),
        "rules[0].name"
    );
}

#[test]
fn a_drop_with_a_priority() {
    assert_eq!(
        refused(
            "[[rules]]\nname = \"stand-ups\"\nwhen = { all = [] }\ndrop = \"a stand-up\"\npriority = \"low\"\n"
        ),
        "rules[0].priority"
    );
    assert_eq!(
        refused(
            "[[rules]]\nname = \"stand-ups\"\nwhen = { all = [] }\ndrop = \"a stand-up\"\nwindow = \"0 9 * * *\"\n"
        ),
        "rules[0].window"
    );
}

#[test]
fn a_launch_with_no_repository() {
    assert_eq!(
        refused(&LAUNCH.replace("repository = \"~/Code/acme/web-app\"\n", "")),
        "rules[0].repository"
    );
    assert_eq!(
        refused(
            "[[rules]]\nname = \"r\"\nwhen = { all = [] }\nperson = \"a look\"\nrepository = \"acme/web-app\"\npriority = \"low\"\n"
        ),
        "rules[0].repository"
    );
}

#[test]
fn a_launch_with_no_priority() {
    assert_eq!(
        refused(&LAUNCH.replace("priority = \"low\"\n", "")),
        "rules[0].priority"
    );
    assert_eq!(
        refused(&LAUNCH.replace("\"low\"", "\"lowest\"")),
        "rules[0].priority"
    );
}

#[test]
fn a_window_that_is_not_five_field_cron() {
    assert_eq!(
        refused(&format!("{LAUNCH}window = \"@hourly\"\n")),
        "rules[0].window"
    );
}

#[test]
fn a_field_the_profile_does_not_name() {
    assert_eq!(
        refused(&format!("{LAUNCH}level = \"patch\"\n")),
        "rules[0].level"
    );
}

#[test]
fn a_match_of_a_form_123_does_not_name() {
    let with_when = |when: &str| {
        LAUNCH.replace(
            "when = { fact = \"level\", in = [\"patch\", \"minor\"] }",
            &format!("when = {when}"),
        )
    };
    assert_eq!(
        refused(&with_when("{ fact = \"level\", above = 3 }")),
        "rules[0].when"
    );
    for (when, field) in [
        ("{ fact = \"level\", is = 1, in = [1] }", "rules[0].when"),
        ("{ all = [], any = [] }", "rules[0].when"),
        ("{ is = 1 }", "rules[0].when"),
        ("\"level\"", "rules[0].when"),
        (
            "{ all = [{ all = [] }, { fact = \"Level\", is = 1 }] }",
            "rules[0].when.all[1].fact",
        ),
        ("{ not = { older = \"2w\" } }", "rules[0].when.not.older"),
        ("{ fact = \"level\", in = \"patch\" }", "rules[0].when.in"),
        (
            "{ fact = \"level\", exists = \"yes\" }",
            "rules[0].when.exists",
        ),
        (
            "{ any = [{ fact = \"level\", contains = 1 }] }",
            "rules[0].when.any[0].contains",
        ),
    ] {
        assert_eq!(refused(&with_when(when)), field, "{when}");
    }
}

#[test]
fn a_value_spelled_as_a_toml_date() {
    let settings = LAUNCH.replace(
        "when = { fact = \"level\", in = [\"patch\", \"minor\"] }",
        "when = { fact = \"due\", is = 2026-11-14 }",
    );
    assert_eq!(refused(&settings), "rules[0].when.is");
    // Spelled in a string, as an offer spells it, it is a date.
    let settings = LAUNCH.replace(
        "when = { fact = \"level\", in = [\"patch\", \"minor\"] }",
        "when = { fact = \"due\", is = \"2026-11-14\" }",
    );
    let policy = Policy::read(&settings).expect("a policy");
    assert_eq!(
        policy.rules()[0].when,
        fact("due", Test::Is(Value::from_text("2026-11-14")))
    );
    assert!(matches!(Value::from_text("2026-11-14"), Value::Date(_)));
}
