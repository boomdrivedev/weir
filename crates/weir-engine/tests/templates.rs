//! A state that runs a statechart starts it afresh each time it is
//! entered, keeps it when left, reads it by its final states and moves it
//! by an enter (96, 97).

mod common;

use std::collections::BTreeMap;

use common::*;
use weir_engine::toml::Value;
use weir_engine::{tick, Guard, Phase, Statechart};

/// The lamp's `on`'s transitions, to be changed.
fn on_transitions(chart: &mut Statechart) -> &mut Vec<weir_engine::Transition> {
    &mut chart
        .regions
        .get_mut("power")
        .and_then(|power| power.states.get_mut("on"))
        .expect("on")
        .transitions
}

#[test]
fn a_template_started_afresh_each_time_its_state_is_entered() {
    let state = holding(&[("power", "off"), ("power.on.glow", "lit")], at(0));
    let snapshot = with_evidence(snapshot(at(1)), &[("lamp.switch", "up".into())]);
    let first = tick(&lamp(), &templates(), &proofs(), &state, &snapshot).expect("a tick");
    assert_eq!(
        first.next.regions,
        holding(
            &[
                ("power", "on"),
                ("power.on.glow", "warming"),
                ("power.on.level", "dim"),
            ],
            at(1)
        )
        .regions
    );
    assert_eq!(first.next.entered_at["power.on.glow"], at(1));
    assert_eq!(
        first.next.set_aside,
        BTreeMap::from([(
            "power.on#1".to_string(),
            BTreeMap::from([("glow".to_string(), "lit".to_string())])
        )])
    );

    let mut off = first.next.clone();
    off.regions.insert("power".into(), "off".into());
    let second = tick(&lamp(), &templates(), &proofs(), &off, &snapshot).expect("a tick");
    assert_eq!(
        second.next.set_aside["power.on#2"],
        BTreeMap::from([
            ("glow".to_string(), "warming".to_string()),
            ("level".to_string(), "dim".to_string()),
        ])
    );
    assert_eq!(
        second.next.set_aside["power.on#1"],
        first.next.set_aside["power.on#1"]
    );
}

#[test]
fn leaving_a_state_keeps_what_it_ran() {
    let state = holding(
        &[
            ("power", "on"),
            ("power.on.glow", "lit"),
            ("power.on.level", "bright"),
        ],
        at(0),
    );
    let snapshot = with_evidence(snapshot(at(1)), &[("lamp.faulty", Value::Boolean(true))]);
    let tick = tick(&lamp(), &templates(), &proofs(), &state, &snapshot).expect("a tick");
    assert_eq!(tick.fired.len(), 1);
    assert_eq!(tick.next.regions["power"], "off");
    assert_eq!(tick.next.regions["power.on.glow"], "lit");
    assert_eq!(tick.next.regions["power.on.level"], "bright");
    assert!(!tick.next.is_live("power.on.glow"));
}

#[test]
fn a_final_state_of_what_a_state_runs() {
    let state = holding(
        &[
            ("power", "on"),
            ("power.on.glow", "burnt"),
            ("power.on.level", "dim"),
        ],
        at(0),
    );
    let tick = tick(&lamp(), &templates(), &proofs(), &state, &snapshot(at(1))).expect("a tick");
    assert_eq!(tick.fired.len(), 1);
    assert_eq!(
        (tick.fired[0].region.as_str(), tick.fired[0].to.as_str()),
        ("power", "off")
    );
    assert_eq!(tick.fired[0].bump.as_deref(), Some("burnouts"));
    assert_eq!(tick.next.counters["burnouts"], 1);
}

#[test]
fn a_state_of_that_name_that_is_not_final() {
    let mut chart = lamp();
    on_transitions(&mut chart)[0].when = Guard::Final("lit".into());
    let state = holding(
        &[
            ("power", "on"),
            ("power.on.glow", "lit"),
            ("power.on.level", "dim"),
        ],
        at(0),
    );
    let tick = tick(&chart, &templates(), &proofs(), &state, &snapshot(at(1))).expect("a tick");
    assert_eq!(tick.fired, Vec::new());
}

#[test]
fn an_enter_moves_what_a_state_ran() {
    let state = holding(
        &[
            ("power", "on"),
            ("power.on.glow", "warming"),
            ("power.on.level", "dim"),
        ],
        at(0),
    );
    let snapshot = with_evidence(snapshot(at(1)), &[("lamp.switch", "down".into())]);
    for key in ["bulb", "glow", "on"] {
        let mut chart = lamp();
        on_transitions(&mut chart)[2].enter =
            BTreeMap::from([(key.to_string(), "cold".to_string())]);
        let tick = tick(&chart, &templates(), &proofs(), &state, &snapshot).expect("a tick");
        assert_eq!(tick.fired.len(), 1, "{key}");
        assert_eq!(tick.fired[0].to, "off", "{key}");
        let effects: Vec<(&str, &str, Phase)> = tick.fired[0]
            .effects
            .iter()
            .map(|e| (e.atom.as_str(), e.region.as_str(), e.phase))
            .collect();
        assert_eq!(
            effects,
            vec![("cool", "power.on.glow", Phase::Enter)],
            "{key}"
        );
        assert_eq!(tick.next.regions["power"], "off", "{key}");
        assert_eq!(tick.next.regions["power.on.glow"], "cold", "{key}");
        assert_eq!(tick.next.entered_at["power.on.glow"], at(1), "{key}");
        assert_eq!(tick.next.regions["power.on.level"], "dim", "{key}");
    }
}
