//! The engine lists the names a statechart reads and runs (98).

mod common;

use std::collections::BTreeSet;

use common::*;
use weir_engine::{names, Guard, Test};

fn set(names: &[&str]) -> BTreeSet<String> {
    names.iter().map(|name| name.to_string()).collect()
}

#[test]
fn the_names_the_lamp_reads_and_runs() {
    let lamp = names(&lamp());
    assert_eq!(
        lamp.evidence,
        set(&["lamp.faulty", "lamp.level", "lamp.switch"])
    );
    assert_eq!(lamp.effects, set(&["light", "set_mode"]));
    assert_eq!(lamp.machines, set(&["bulb"]));

    let bulb = names(&bulb());
    assert_eq!(bulb.evidence, set(&["bulb.burnt", "bulb.warm"]));
    assert_eq!(bulb.effects, set(&["cool"]));
    assert_eq!(bulb.machines, BTreeSet::new());

    let compared = names(&one_region(Guard::Not(Box::new(ev(
        "lamp.level",
        Test::GteEv("lamp.target".into()),
    )))));
    assert_eq!(compared.evidence, set(&["lamp.level", "lamp.target"]));
}
