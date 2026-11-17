//! In each region, innermost first, the first transition whose guard holds
//! fires, every guard reading the one snapshot and the state as given
//! (96).

mod common;

use common::*;
use weir_engine::toml::{Table, Value};
use weir_engine::{tick, Applied, Fired, Guard, Phase, Planned, State};

#[test]
fn the_first_transition_whose_guard_holds() {
    let state = holding(&[("power", "off")], at(0));
    let snapshot = with_responses(
        with_evidence(snapshot(at(1)), &[("lamp.switch", "up".into())]),
        &[("r1", "on")],
    );
    let tick = tick(&lamp(), &templates(), &proofs(), &state, &snapshot).expect("a tick");
    assert_eq!(
        tick.fired,
        vec![Fired {
            region: "power".into(),
            from: "off".into(),
            to: "on".into(),
            bump: None,
            response: Some(Applied {
                id: "r1".into(),
                argument: String::new(),
            }),
            effects: vec![Planned {
                atom: "light".into(),
                args: Table::new(),
                region: "power".into(),
                phase: Phase::Entry,
            }],
        }]
    );
}

#[test]
fn innermost_first_at_most_one_per_region() {
    let state = holding(
        &[
            ("power", "on"),
            ("power.on.glow", "warming"),
            ("power.on.level", "dim"),
        ],
        at(0),
    );
    let snapshot = with_evidence(
        snapshot(at(1)),
        &[
            ("lamp.level", Value::Integer(7)),
            ("lamp.faulty", Value::Boolean(true)),
        ],
    );
    let tick = tick(&lamp(), &templates(), &proofs(), &state, &snapshot).expect("a tick");
    let moves: Vec<(&str, &str, &str)> = tick
        .fired
        .iter()
        .map(|fired| {
            (
                fired.region.as_str(),
                fired.from.as_str(),
                fired.to.as_str(),
            )
        })
        .collect();
    assert_eq!(
        moves,
        vec![("power.on.level", "dim", "bright"), ("power", "on", "off")]
    );
}

/// A lamp of two regions: `power`, `off` moving `on` when `lamp.switch`
/// reads `up`, and `shade`, `down` moving `up` when `power` is `on`.
fn lamp_with_a_shade() -> weir_engine::Statechart {
    let mut chart = one_region(is("lamp.switch", "up"));
    let down = State {
        transitions: vec![transition(
            Guard::Region {
                name: "power".into(),
                within: vec!["on".into()],
            },
            "up",
        )],
        ..state()
    };
    chart.regions.insert(
        "shade".into(),
        region("down", vec![("down", down), ("up", state())]),
    );
    chart
}

#[test]
fn a_sibling_sees_a_move_at_the_next_tick() {
    let chart = lamp_with_a_shade();
    let state = holding(&[("power", "off"), ("shade", "down")], at(0));
    let snapshot = with_evidence(snapshot(at(1)), &[("lamp.switch", "up".into())]);
    let first = tick(&chart, &templates(), &proofs(), &state, &snapshot).expect("a tick");
    let moved: Vec<&str> = first.fired.iter().map(|f| f.region.as_str()).collect();
    assert_eq!(moved, vec!["power"]);
    assert_eq!(first.next.regions["shade"], "down");
    let second = tick(&chart, &templates(), &proofs(), &first.next, &snapshot).expect("a tick");
    let moved: Vec<&str> = second.fired.iter().map(|f| f.region.as_str()).collect();
    assert_eq!(moved, vec!["shade"]);
    assert_eq!(second.next.regions["shade"], "up");
}

#[test]
fn a_transition_to_its_own_state_with_nothing_to_do() {
    let mut chart = one_region(is("lamp.switch", "up"));
    let off = chart
        .regions
        .get_mut("power")
        .and_then(|power| power.states.get_mut("off"))
        .expect("off");
    off.transitions
        .insert(0, transition(is("lamp.switch", "up"), "off"));
    let state = holding(&[("power", "off")], at(0));
    let snapshot = with_evidence(snapshot(at(1)), &[("lamp.switch", "up".into())]);
    let tick = tick(&chart, &templates(), &proofs(), &state, &snapshot).expect("a tick");
    assert_eq!(tick.fired, Vec::new());
    assert_eq!(tick.next, state);
}
