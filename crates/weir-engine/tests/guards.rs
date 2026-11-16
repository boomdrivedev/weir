//! Each guard tried through the tick, over a lamp of one region whose one
//! transition is under the guard (94, 96, 98).

mod common;

use common::*;
use time::format_description::well_known::Rfc3339;
use weir_engine::toml::value::Datetime;
use weir_engine::toml::Value;
use weir_engine::{tick, ChildrenTest, Guard, ObjectState, Snapshot, Statechart, Test};

/// Whether the lamp of one region, in `off`, moves under `guard` over
/// `snapshot`.
fn fires(guard: Guard, snapshot: &Snapshot) -> bool {
    fires_in(
        &one_region(guard),
        &holding(&[("power", "off")], at(0)),
        snapshot,
    )
}

/// Whether `chart`, in `state`, moves `power` over `snapshot`.
fn fires_in(chart: &Statechart, state: &ObjectState, snapshot: &Snapshot) -> bool {
    tick(chart, &templates(), &proofs(), state, snapshot)
        .expect("a tick")
        .fired
        .iter()
        .any(|fired| fired.region == "power")
}

/// A snapshot read at minute 1 holding the object's evidence given.
fn reading(evidence: &[(&str, Value)]) -> Snapshot {
    with_evidence(snapshot(at(1)), evidence)
}

fn int(n: i64) -> Value {
    Value::Integer(n)
}

fn text(s: &str) -> Value {
    Value::String(s.into())
}

#[test]
fn is_and_in() {
    let read = reading(&[("lamp.mode", text("warm"))]);
    assert!(fires(ev("lamp.mode", Test::Is(text("warm"))), &read));
    assert!(!fires(ev("lamp.mode", Test::Is(text("cool"))), &read));
    assert!(fires(
        ev("lamp.mode", Test::In(vec![text("cool"), text("warm")])),
        &read
    ));
    assert!(!fires(ev("lamp.mode", Test::In(vec![text("cool")])), &read));
}

#[test]
fn an_evidence_read_or_not() {
    let read = reading(&[("lamp.mode", text("warm"))]);
    assert!(fires(ev("lamp.mode", Test::Exists(true)), &read));
    assert!(!fires(ev("lamp.mode", Test::Exists(false)), &read));
    assert!(fires(ev("lamp.hue", Test::Exists(false)), &read));
    assert!(!fires(ev("lamp.hue", Test::Exists(true)), &read));
    assert!(!fires(ev("lamp.hue", Test::Is(text("warm"))), &read));
    assert!(!fires(
        ev("lamp.hue", Test::Is(Value::Boolean(false))),
        &read
    ));
}

#[test]
fn values_in_order() {
    let read = reading(&[("lamp.level", int(3)), ("lamp.mode", text("b"))]);
    let level = |test: Test| fires(ev("lamp.level", test), &read);
    assert!(level(Test::Gt(int(2))));
    assert!(!level(Test::Gt(int(3))));
    assert!(level(Test::Gte(int(3))));
    assert!(!level(Test::Gte(int(4))));
    assert!(level(Test::Lt(int(4))));
    assert!(!level(Test::Lt(int(3))));
    assert!(level(Test::Lte(int(3))));
    assert!(!level(Test::Lte(int(2))));
    assert!(level(Test::Gt(Value::Float(2.5))));
    assert!(!level(Test::Gt(text("2"))), "a number against a string");
    assert!(!level(Test::Lt(text("4"))), "a number against a string");
    let mode = |test: Test| fires(ev("lamp.mode", test), &read);
    assert!(mode(Test::Gt(text("a"))), "strings by their bytes");
    assert!(!mode(Test::Gt(int(1))), "a string against a number");
}

#[test]
fn a_guard_by_age() {
    let older = || {
        ev(
            "lamp.since",
            Test::Older(std::time::Duration::from_secs(600)),
        )
    };
    let since = |minutes: i64| text(&at(-minutes).format(&Rfc3339).expect("a time"));
    let read_at = |value: Value| with_evidence(snapshot(at(0)), &[("lamp.since", value)]);
    assert!(!fires(older(), &read_at(since(10))));
    assert!(fires(older(), &read_at(since(11))));
    let datetime: Datetime = "2026-09-04T11:49:00Z".parse().expect("a datetime");
    assert!(fires(older(), &read_at(Value::Datetime(datetime))));
    let offset: Datetime = "2026-09-04T13:49:00+02:00".parse().expect("a datetime");
    assert!(fires(older(), &read_at(Value::Datetime(offset))));
    let local: Datetime = "2026-09-04T11:49:00".parse().expect("a datetime");
    assert!(
        !fires(older(), &read_at(Value::Datetime(local))),
        "a datetime with no offset is no time"
    );
    assert!(!fires(older(), &read_at(text("yesterday"))));
    assert!(!fires(older(), &read_at(int(11))));
    assert!(!fires(older(), &snapshot(at(0))), "an evidence not read");
}

#[test]
fn against_another_evidence() {
    let read = reading(&[
        ("lamp.level", int(5)),
        ("lamp.target", int(5)),
        ("lamp.other", int(6)),
    ]);
    let level = |test: Test| fires(ev("lamp.level", test), &read);
    assert!(level(Test::EqEv("lamp.target".into())));
    assert!(!level(Test::EqEv("lamp.other".into())));
    assert!(level(Test::NeEv("lamp.other".into())));
    assert!(!level(Test::NeEv("lamp.target".into())));
    assert!(level(Test::GteEv("lamp.target".into())));
    assert!(!level(Test::GteEv("lamp.other".into())));
    assert!(level(Test::LtEv("lamp.other".into())));
    assert!(!level(Test::LtEv("lamp.target".into())));
    assert!(!level(Test::GtEv("lamp.target".into())));
    assert!(fires(
        ev("lamp.other", Test::GtEv("lamp.level".into())),
        &read
    ));
    assert!(
        level(Test::NeEv("lamp.unread".into())),
        "ne_ev against one not read"
    );
    assert!(!level(Test::EqEv("lamp.unread".into())));
    assert!(!level(Test::GtEv("lamp.unread".into())));
    assert!(!fires(
        ev("lamp.unread", Test::NeEv("lamp.level".into())),
        &read
    ));
}

#[test]
fn values_of_two_types() {
    let read = reading(&[("lamp.level", int(5))]);
    assert!(fires(ev("lamp.level", Test::Is(Value::Float(5.0))), &read));
    assert!(!fires(ev("lamp.level", Test::Is(text("5"))), &read));
    assert!(fires(
        ev("lamp.level", Test::In(vec![text("5"), Value::Float(5.0)])),
        &read
    ));
    let truth = reading(&[("lamp.faulty", text("true"))]);
    assert!(!fires(
        ev("lamp.faulty", Test::Is(Value::Boolean(true))),
        &truth
    ));
    let instant = reading(&[("lamp.since", text("2026-09-04T12:00:00Z"))]);
    assert!(fires(
        ev("lamp.since", Test::Is(text("2026-09-04T14:00:00+02:00"))),
        &instant
    ));
}

#[test]
fn evidence_read_at_its_region() {
    let state = holding(
        &[
            ("power", "on"),
            ("power.on.glow", "warming"),
            ("power.on.level", "dim"),
        ],
        at(0),
    );
    let level_moves = |snapshot: &Snapshot| {
        tick(&lamp(), &templates(), &proofs(), &state, snapshot)
            .expect("a tick")
            .next
            .regions["power.on.level"]
            == "bright"
    };
    let mut snapshot = reading(&[("lamp.level", int(1))]);
    assert!(!level_moves(&snapshot));
    snapshot.regions.insert(
        "power.on.level".into(),
        [("lamp.level".to_string(), int(7))].into(),
    );
    assert!(
        level_moves(&snapshot),
        "read at its path before the object's"
    );
    let mut above = reading(&[("lamp.level", int(1))]);
    above
        .regions
        .insert("power".into(), [("lamp.level".to_string(), int(7))].into());
    assert!(level_moves(&above), "read at the nearest path above it");
    let mut beside = reading(&[("lamp.level", int(1))]);
    beside.regions.insert(
        "power.on.glow".into(),
        [("lamp.level".to_string(), int(7))].into(),
    );
    assert!(!level_moves(&beside), "never at a path beside it");
}

#[test]
fn no_evidence_of_the_engines_own() {
    let read = reading(&[]);
    for name in ["state", "entered_at", "now", "seq", "applied_responses"] {
        assert!(fires(ev(name, Test::Exists(false)), &read), "{name}");
        assert!(!fires(ev(name, Test::Exists(true)), &read), "{name}");
        assert!(!fires(ev(name, Test::Is(text("off"))), &read), "{name}");
    }
}

#[test]
fn a_sibling_region() {
    let shade = |guard: Guard| {
        let mut chart = one_region(guard);
        let up = weir_engine::State {
            regions: [(
                "slat".to_string(),
                region("shut", vec![("shut", state()), ("open", state())]),
            )]
            .into(),
            ..state()
        };
        chart.regions.insert(
            "shade".into(),
            region("down", vec![("down", state()), ("up", up)]),
        );
        chart
    };
    let in_states = |name: &str, within: &[&str]| Guard::Region {
        name: name.into(),
        within: within.iter().map(|s| s.to_string()).collect(),
    };
    let read = reading(&[]);
    let up = holding(
        &[("power", "off"), ("shade", "up"), ("shade.up.slat", "open")],
        at(0),
    );
    let down = holding(&[("power", "off"), ("shade", "down")], at(0));
    let beside = shade(in_states("shade", &["up"]));
    assert!(fires_in(&beside, &up, &read));
    assert!(!fires_in(&beside, &down, &read));
    for dotted in ["shade.up.slat", "up.slat"] {
        let chart = shade(in_states(dotted, &["open"]));
        assert!(fires_in(&chart, &up, &read), "{dotted}");
        let mut shut = up.clone();
        shut.regions.insert("shade.up.slat".into(), "shut".into());
        assert!(!fires_in(&chart, &shut, &read), "{dotted}");
    }
    let none = shade(in_states("slat", &["open"]));
    assert!(
        !fires_in(&none, &up, &read),
        "a bare name is neither under its own state nor beside it"
    );
}

#[test]
fn the_owner_and_the_children() {
    let mut read = reading(&[]);
    read.owner = Some(related("lamp", &["open"]));
    read.children = vec![related("bulb", &["lit"]), related("bulb", &["burnt"])];
    let parent = |within: &[&str]| Guard::Parent {
        within: within.iter().map(|s| s.to_string()).collect(),
    };
    let listed = |states: &[&str]| states.iter().map(|s| s.to_string()).collect::<Vec<_>>();
    assert!(fires(parent(&["open"]), &read));
    assert!(!fires(parent(&["shut"]), &read));
    assert!(!fires(parent(&["open"]), &reading(&[])), "no owner given");
    assert!(fires(
        children("bulb", ChildrenTest::Any(listed(&["lit"]))),
        &read
    ));
    assert!(fires(children("bulb", ChildrenTest::CountGte(2)), &read));
    assert!(!fires(children("bulb", ChildrenTest::CountGte(3)), &read));
    assert!(!fires(
        children("bulb", ChildrenTest::All(listed(&["lit"]))),
        &read
    ));
    assert!(fires(
        children("bulb", ChildrenTest::All(listed(&["lit", "burnt"]))),
        &read
    ));
    assert!(!fires(
        children("bulb", ChildrenTest::None(listed(&["burnt"]))),
        &read
    ));
    assert!(fires(
        children("bulb", ChildrenTest::None(listed(&["cold"]))),
        &read
    ));
    assert!(
        !fires(children("lamp", ChildrenTest::All(listed(&["lit"]))), &read),
        "all of a kind with no child"
    );
}

#[test]
fn all_any_not_and_always() {
    let read = reading(&[("lamp.switch", text("up"))]);
    let yes = || is("lamp.switch", "up");
    let no = || is("lamp.switch", "down");
    assert!(!fires(Guard::All(vec![yes(), no()]), &read));
    assert!(fires(Guard::All(vec![yes(), yes()]), &read));
    assert!(fires(Guard::Any(vec![no(), yes()]), &read));
    assert!(!fires(Guard::Any(vec![no(), no()]), &read));
    assert!(!fires(Guard::Not(Box::new(yes())), &read));
    assert!(fires(Guard::Not(Box::new(no())), &read));
    assert!(fires(Guard::Always(true), &read));
    assert!(!fires(Guard::Always(false), &read));
    assert!(fires(Guard::All(Vec::new()), &read));
    assert!(!fires(Guard::Any(Vec::new()), &read));
}
