//! The engine refuses a state or a template it cannot read, naming the
//! path (96, 97).

mod common;

use std::collections::BTreeMap;

use common::*;
use weir_engine::{start, tick, Refusal};

#[test]
fn a_region_the_statechart_does_not_hold() {
    let state = holding(&[("power", "off"), ("shade", "up")], at(0));
    let refused =
        tick(&lamp(), &templates(), &proofs(), &state, &snapshot(at(1))).expect_err("refused");
    assert_eq!(
        refused,
        Refusal {
            path: "shade".into(),
            reason: "names no region of the statechart".into(),
        }
    );
    assert_eq!(
        refused.to_string(),
        "`shade`: names no region of the statechart"
    );
}

#[test]
fn a_state_its_region_does_not_hold() {
    let state = holding(&[("power", "dimmed")], at(0));
    let refused =
        tick(&lamp(), &templates(), &proofs(), &state, &snapshot(at(1))).expect_err("refused");
    assert_eq!(
        refused,
        Refusal {
            path: "power".into(),
            reason: "holds a state its region does not hold".into(),
        }
    );
}

#[test]
fn a_template_no_one_gave() {
    let state = holding(&[("power", "off")], at(0));
    let snapshot = with_evidence(snapshot(at(1)), &[("lamp.switch", "up".into())]);
    let refused =
        tick(&lamp(), &BTreeMap::new(), &proofs(), &state, &snapshot).expect_err("refused");
    assert_eq!(
        refused.to_string(),
        "`power.on`: runs `bulb`, which is no template given"
    );
}

#[test]
fn a_statechart_that_runs_itself() {
    let mut chart = lamp();
    chart
        .regions
        .get_mut("power")
        .and_then(|power| power.states.get_mut("off"))
        .expect("off")
        .machine = Some("lamp".into());
    let mut templates = templates();
    templates.insert("lamp".into(), chart.clone());
    let refused = start(&chart, &templates, at(0)).expect_err("refused");
    assert_eq!(
        refused,
        Refusal {
            path: "power.off".into(),
            reason: "runs `lamp`, a statechart it already lies inside".into(),
        }
    );
}

#[test]
fn a_region_a_state_and_its_template_both_hold() {
    let mut chart = lamp();
    let on = chart
        .regions
        .get_mut("power")
        .and_then(|power| power.states.get_mut("on"))
        .expect("on");
    on.regions
        .insert("glow".into(), region("dark", vec![("dark", state())]));
    let state = holding(&[("power", "off")], at(0));
    let snapshot = with_evidence(snapshot(at(1)), &[("lamp.switch", "up".into())]);
    let refused = tick(&chart, &templates(), &proofs(), &state, &snapshot).expect_err("refused");
    assert_eq!(
        refused,
        Refusal {
            path: "power.on".into(),
            reason: "holds a region `glow` as its own and in the template it runs".into(),
        }
    );
}
