//! A decision stands while its state is active (97).

mod common;

use common::*;
use weir_engine::{decisions, Decision, Standing};

#[test]
fn a_decision_stands_while_its_state_is_active() {
    let state = holding(&[("power", "off")], at(3));
    let standing = decisions(&lamp(), &templates(), &state).expect("decisions");
    let switch = lamp().regions["power"].states["off"]
        .decision
        .clone()
        .expect("a decision");
    assert_eq!(
        standing,
        vec![Standing {
            region: "power".into(),
            state: "off".into(),
            decision: switch,
            since: Some(at(3)),
        }]
    );
}

#[test]
fn a_decision_whose_state_was_left() {
    let state = holding(
        &[
            ("power", "on"),
            ("power.on.glow", "lit"),
            ("power.on.level", "dim"),
        ],
        at(0),
    );
    assert_eq!(
        decisions(&lamp(), &templates(), &state).expect("decisions"),
        Vec::new()
    );

    let mut chart = lamp();
    let dim = chart
        .regions
        .get_mut("power")
        .and_then(|power| power.states.get_mut("on"))
        .and_then(|on| on.regions.get_mut("level"))
        .and_then(|level| level.states.get_mut("dim"))
        .expect("dim");
    dim.decision = Some(Decision {
        kind: "brighten".into(),
        answers: vec!["yes".into()],
        shows: Vec::new(),
        sections: Default::default(),
    });
    let left = holding(&[("power", "off"), ("power.on.level", "dim")], at(0));
    let standing: Vec<String> = decisions(&chart, &templates(), &left)
        .expect("decisions")
        .into_iter()
        .map(|standing| standing.region)
        .collect();
    assert_eq!(standing, vec!["power".to_string()]);
}
