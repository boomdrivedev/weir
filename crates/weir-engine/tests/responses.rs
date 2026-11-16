//! A response is applied once, and its pattern binds the argument its
//! answer gives (97).

mod common;

use common::*;
use weir_engine::toml::{Table, Value};
use weir_engine::{tick, Applied, Phase, Planned};

#[test]
fn a_response_applied_once() {
    let state = holding(&[("power", "off")], at(0));
    let snapshot = with_responses(snapshot(at(1)), &[("r1", "on")]);
    let tick = tick(&lamp(), &templates(), &proofs(), &state, &snapshot).expect("a tick");
    assert_eq!(tick.fired.len(), 1);
    assert_eq!(tick.fired[0].to, "on");
    assert_eq!(
        tick.fired[0].response,
        Some(Applied {
            id: "r1".into(),
            argument: String::new(),
        })
    );
    assert_eq!(tick.next.applied, vec!["r1".to_string()]);
}

#[test]
fn a_response_already_applied() {
    let mut state = holding(&[("power", "off")], at(0));
    state.applied.push("r1".into());
    let snapshot = with_responses(snapshot(at(1)), &[("r1", "on")]);
    let tick = tick(&lamp(), &templates(), &proofs(), &state, &snapshot).expect("a tick");
    assert_eq!(tick.fired, Vec::new());
    assert_eq!(tick.next, state);
}

#[test]
fn a_response_binds_its_argument() {
    let state = holding(&[("power", "off")], at(0));
    let snapshot = with_responses(snapshot(at(1)), &[("r1", "mode warm")]);
    let tick = tick(&lamp(), &templates(), &proofs(), &state, &snapshot).expect("a tick");
    assert_eq!(tick.fired.len(), 1);
    let fired = &tick.fired[0];
    assert_eq!((fired.from.as_str(), fired.to.as_str()), ("off", "off"));
    assert_eq!(
        fired.response,
        Some(Applied {
            id: "r1".into(),
            argument: "warm".into(),
        })
    );
    let mut args = Table::new();
    args.insert("mode".into(), Value::String("warm".into()));
    assert_eq!(
        fired.effects,
        vec![Planned {
            atom: "set_mode".into(),
            args,
            region: "power".into(),
            phase: Phase::Transition,
        }]
    );
    assert_eq!(tick.next.applied, vec!["r1".to_string()]);
}

#[test]
fn two_regions_take_one_response_in_one_tick() {
    let mut chart = one_region(response("go"));
    let shade = region(
        "down",
        vec![
            (
                "down",
                weir_engine::State {
                    transitions: vec![transition(response("go"), "up")],
                    ..state()
                },
            ),
            ("up", state()),
        ],
    );
    chart.regions.insert("shade".into(), shade);
    let state = holding(&[("power", "off"), ("shade", "down")], at(0));
    let snapshot = with_responses(snapshot(at(1)), &[("r1", "go")]);
    let tick = tick(&chart, &templates(), &proofs(), &state, &snapshot).expect("a tick");
    let taken: Vec<(&str, Option<&str>)> = tick
        .fired
        .iter()
        .map(|f| {
            (
                f.region.as_str(),
                f.response.as_ref().map(|r| r.id.as_str()),
            )
        })
        .collect();
    assert_eq!(taken, vec![("power", Some("r1")), ("shade", Some("r1"))]);
    assert_eq!(tick.next.applied, vec!["r1".to_string()]);
}

#[test]
fn the_evidence_branch_of_any_takes_no_response() {
    let state = holding(
        &[
            ("power", "on"),
            ("power.on.glow", "warming"),
            ("power.on.level", "dim"),
        ],
        at(0),
    );
    let snapshot = with_responses(
        with_evidence(snapshot(at(1)), &[("lamp.faulty", Value::Boolean(true))]),
        &[("r1", "off")],
    );
    let tick = tick(&lamp(), &templates(), &proofs(), &state, &snapshot).expect("a tick");
    assert_eq!(tick.fired.len(), 1);
    assert_eq!(tick.fired[0].to, "off");
    assert_eq!(tick.fired[0].response, None);
    assert_eq!(tick.next.applied, Vec::<String>::new());
}
