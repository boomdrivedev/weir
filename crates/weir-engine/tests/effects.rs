//! Firing plans its effects, each only while its proof is absent, and
//! records the move (96).

mod common;

use common::*;
use weir_engine::toml::{Table, Value};
use weir_engine::{tick, Phase, Planned, State, Statechart, Transition};

/// A planned effect's atom, region and phase.
fn planned(effects: &[Planned]) -> Vec<(&str, &str, Phase)> {
    effects
        .iter()
        .map(|e| (e.atom.as_str(), e.region.as_str(), e.phase))
        .collect()
}

/// A lamp of one region whose `off` clicks on exit, moves `on` when
/// `lamp.switch` reads `up` with a tally of its own, or to itself when it
/// reads `again` with a chime of its own, and whose `off` and `on` light on
/// entry.
fn a_lamp_with_every_phase() -> Statechart {
    let off = State {
        exit: vec![effect("click")],
        entry: vec![effect("light")],
        transitions: vec![
            Transition {
                effects: vec![effect("tally")],
                ..transition(is("lamp.switch", "up"), "on")
            },
            Transition {
                effects: vec![effect("chime")],
                ..transition(is("lamp.switch", "again"), "off")
            },
        ],
        ..state()
    };
    let on = State {
        entry: vec![effect("light")],
        ..state()
    };
    chart(
        "lamp",
        vec![("power", region("off", vec![("off", off), ("on", on)]))],
    )
}

#[test]
fn exit_transition_and_entry_effects_in_order() {
    let state = holding(&[("power", "off")], at(0));
    let snapshot = with_evidence(snapshot(at(1)), &[("lamp.switch", "up".into())]);
    let tick = tick(
        &a_lamp_with_every_phase(),
        &templates(),
        &proofs(),
        &state,
        &snapshot,
    )
    .expect("a tick");
    assert_eq!(tick.fired.len(), 1);
    assert_eq!(
        planned(&tick.fired[0].effects),
        vec![
            ("click", "power", Phase::Exit),
            ("tally", "power", Phase::Transition),
            ("light", "power", Phase::Entry),
        ]
    );
}

#[test]
fn an_effect_whose_proof_is_present() {
    let state = holding(&[("power", "off")], at(0));
    let snapshot = with_evidence(
        snapshot(at(1)),
        &[
            ("lamp.switch", "up".into()),
            ("lamp.lit", Value::Boolean(true)),
        ],
    );
    let tick = tick(&s21_lamp(), &templates(), &proofs(), &state, &snapshot).expect("a tick");
    assert_eq!(tick.fired.len(), 1);
    assert_eq!(
        (tick.fired[0].from.as_str(), tick.fired[0].to.as_str()),
        ("off", "on")
    );
    assert_eq!(tick.fired[0].effects, Vec::new());
    assert_eq!(tick.next, holding(&[("power", "on")], at(1)));
}

#[test]
fn an_effect_with_no_proof_named() {
    assert!(!proofs().contains_key("set_mode"));
    let state = holding(&[("power", "off")], at(0));
    let snapshot = with_responses(snapshot(at(1)), &[("r1", "mode warm")]);
    let tick = tick(&lamp(), &templates(), &proofs(), &state, &snapshot).expect("a tick");
    assert_eq!(tick.fired.len(), 1);
    let mut args = Table::new();
    args.insert("mode".into(), "warm".into());
    assert_eq!(
        tick.fired[0].effects,
        vec![Planned {
            atom: "set_mode".into(),
            args,
            region: "power".into(),
            phase: Phase::Transition,
        }]
    );
}

#[test]
fn a_transition_to_its_own_state_runs_its_own_effects_alone() {
    let state = holding(&[("power", "off")], at(0));
    let snapshot = with_evidence(snapshot(at(1)), &[("lamp.switch", "again".into())]);
    let tick = tick(
        &a_lamp_with_every_phase(),
        &templates(),
        &proofs(),
        &state,
        &snapshot,
    )
    .expect("a tick");
    assert_eq!(tick.fired.len(), 1);
    assert_eq!(
        planned(&tick.fired[0].effects),
        vec![("chime", "power", Phase::Transition)]
    );
}

#[test]
fn firing_records_the_move() {
    let mut state = holding(
        &[
            ("power", "on"),
            ("power.on.glow", "burnt"),
            ("power.on.level", "dim"),
        ],
        at(0),
    );
    state.counters.insert("burnouts".into(), 2);
    let tick = tick(&lamp(), &templates(), &proofs(), &state, &snapshot(at(9))).expect("a tick");
    assert_eq!(tick.fired.len(), 1);
    assert_eq!(tick.fired[0].bump.as_deref(), Some("burnouts"));
    assert_eq!(tick.next.regions["power"], "off");
    assert_eq!(tick.next.entered_at["power"], at(9));
    assert_eq!(tick.next.counters["burnouts"], 3);
}

#[test]
fn a_transition_to_its_own_state_keeps_its_time() {
    let state = holding(&[("power", "off")], at(0));
    let snapshot = with_responses(snapshot(at(4)), &[("r1", "mode warm")]);
    let tick = tick(&lamp(), &templates(), &proofs(), &state, &snapshot).expect("a tick");
    assert_eq!(tick.fired.len(), 1);
    assert_eq!(tick.next.regions["power"], "off");
    assert_eq!(tick.next.entered_at["power"], at(0));
}
