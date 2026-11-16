//! An object starts in its initial states (96, 97).

mod common;

use common::*;
use weir_engine::{start, Statechart};

#[test]
fn a_lamp_starts_off() {
    let state = start(&lamp(), &templates(), at(0)).expect("the lamp starts");
    assert_eq!(state, holding(&[("power", "off")], at(0)));
}

#[test]
fn a_state_that_runs_a_template_starts_it() {
    let mut chart: Statechart = lamp();
    chart.regions.get_mut("power").expect("power").initial = "on".into();
    let state = start(&chart, &templates(), at(0)).expect("the lamp starts");
    assert_eq!(
        state,
        holding(
            &[
                ("power", "on"),
                ("power.on.glow", "warming"),
                ("power.on.level", "dim"),
            ],
            at(0)
        )
    );
}
