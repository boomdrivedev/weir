//! The tick answers from its inputs alone (69, 96, 98, 99; S21).

mod common;

use common::*;
use weir_engine::toml::{Table, Value};
use weir_engine::{tick, Fired, Phase, Planned};

#[test]
fn the_lamp_switched_on() {
    let state = holding(&[("power", "off")], at(0));
    let snapshot = with_evidence(snapshot(at(1)), &[("lamp.switch", "up".into())]);
    let tick = tick(&s21_lamp(), &templates(), &proofs(), &state, &snapshot).expect("a tick");
    assert_eq!(
        tick.fired,
        vec![Fired {
            region: "power".into(),
            from: "off".into(),
            to: "on".into(),
            bump: None,
            response: None,
            effects: vec![Planned {
                atom: "light".into(),
                args: Table::new(),
                region: "power".into(),
                phase: Phase::Entry,
            }],
        }]
    );
    assert_eq!(tick.next, holding(&[("power", "on")], at(1)));
}

#[test]
fn the_lamp_on_with_its_proof_present() {
    let state = holding(&[("power", "on")], at(1));
    let snapshot = with_evidence(snapshot(at(2)), &[("lamp.lit", Value::Boolean(true))]);
    let tick = tick(&s21_lamp(), &templates(), &proofs(), &state, &snapshot).expect("a tick");
    assert_eq!(tick.fired, Vec::new());
    assert_eq!(tick.next, state);
}

#[test]
fn a_tick_given_twice() {
    let state = holding(
        &[
            ("power", "on"),
            ("power.on.glow", "lit"),
            ("power.on.level", "dim"),
        ],
        at(0),
    );
    let snapshot = with_responses(
        with_evidence(
            snapshot(at(5)),
            &[
                ("lamp.level", Value::Integer(7)),
                ("bulb.burnt", Value::Boolean(true)),
            ],
        ),
        &[("r1", "off")],
    );
    let first = tick(&lamp(), &templates(), &proofs(), &state, &snapshot).expect("a tick");
    let second = tick(&lamp(), &templates(), &proofs(), &state, &snapshot).expect("a tick");
    assert!(!first.fired.is_empty());
    assert_eq!(first, second);
}

#[test]
fn ticking_changes_nothing() {
    let (chart, templates, proofs) = (lamp(), templates(), proofs());
    let state = holding(&[("power", "off")], at(0));
    let snapshot = with_responses(
        with_evidence(snapshot(at(1)), &[("lamp.switch", "up".into())]),
        &[("r1", "mode warm")],
    );
    let copies = (
        chart.clone(),
        templates.clone(),
        proofs.clone(),
        state.clone(),
        snapshot.clone(),
    );
    let tick = tick(&chart, &templates, &proofs, &state, &snapshot).expect("a tick");
    assert!(!tick.fired.is_empty());
    assert_eq!((chart, templates, proofs, state, snapshot), copies);
}
