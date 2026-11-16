//! The toy the engine's tests run: a lamp and the bulb it runs, built from
//! the engine's types with no reading of TOML, naming only a lamp's and a
//! bulb's atoms (98).
#![allow(dead_code)]

use std::collections::BTreeMap;

use time::format_description::well_known::Rfc3339;
use time::{Duration, OffsetDateTime};
use weir_engine::toml::{Table, Value};
use weir_engine::{
    ChildrenTest, Decision, Effect, Guard, ObjectState, Region, Related, Response, Snapshot, State,
    Statechart, Test, Transition,
};

/// The lamp: one region, `power`. `off` carries the decision `switch`; a
/// response `mode <name>` keeps it `off` planning `set_mode`, and a
/// response `on`, or `lamp.switch` read `up`, moves it `on`. `on` plans
/// `light` on entry, runs the bulb and holds a region `level`; it moves
/// `off` when the bulb is `burnt`, bumping `burnouts`, when `lamp.faulty`
/// is `true` or a response says `off`, and when `lamp.switch` reads
/// `down`, moving the bulb to `cold`.
pub fn lamp() -> Statechart {
    let off = State {
        decision: Some(Decision {
            kind: "switch".into(),
            answers: vec!["on".into(), "mode <name>".into()],
            shows: Vec::new(),
            sections: Table::new(),
        }),
        transitions: vec![
            Transition {
                effects: vec![effect_with("set_mode", &[("mode", "$response")])],
                ..transition(response("mode <name>"), "off")
            },
            transition(response("on"), "on"),
            transition(is("lamp.switch", "up"), "on"),
        ],
        ..state()
    };
    let level = region(
        "dim",
        vec![
            (
                "dim",
                State {
                    transitions: vec![transition(
                        ev("lamp.level", Test::Gte(Value::Integer(5))),
                        "bright",
                    )],
                    ..state()
                },
            ),
            ("bright", state()),
        ],
    );
    let on = State {
        entry: vec![effect("light")],
        machine: Some("bulb".into()),
        regions: BTreeMap::from([("level".to_string(), level)]),
        transitions: vec![
            Transition {
                bump: Some("burnouts".into()),
                ..transition(Guard::Final("burnt".into()), "off")
            },
            transition(
                Guard::Any(vec![is_true("lamp.faulty"), response("off")]),
                "off",
            ),
            Transition {
                enter: BTreeMap::from([("bulb".to_string(), "cold".to_string())]),
                ..transition(is("lamp.switch", "down"), "off")
            },
        ],
        ..state()
    };
    chart(
        "lamp",
        vec![("power", region("off", vec![("off", off), ("on", on)]))],
    )
}

/// The bulb: one region, `glow`. `warming` moves `lit` when `bulb.warm` is
/// `true`, `warming` or `lit` moves `burnt` when `bulb.burnt` is `true`,
/// and `burnt` and `cold` are final, `cold` planning `cool` on entry.
pub fn bulb() -> Statechart {
    let warming = State {
        transitions: vec![
            transition(is_true("bulb.warm"), "lit"),
            transition(is_true("bulb.burnt"), "burnt"),
        ],
        ..state()
    };
    let lit = State {
        transitions: vec![transition(is_true("bulb.burnt"), "burnt")],
        ..state()
    };
    let burnt = State {
        is_final: true,
        ..state()
    };
    let cold = State {
        is_final: true,
        entry: vec![effect("cool")],
        ..state()
    };
    chart(
        "bulb",
        vec![(
            "glow",
            region(
                "warming",
                vec![
                    ("warming", warming),
                    ("lit", lit),
                    ("burnt", burnt),
                    ("cold", cold),
                ],
            ),
        )],
    )
}

/// S21's lamp: `power`'s two states alone, `off` moving `on` when
/// `lamp.switch` reads `up`, and `on` planning `light` on entry.
pub fn s21_lamp() -> Statechart {
    let off = State {
        transitions: vec![transition(is("lamp.switch", "up"), "on")],
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

/// A lamp of one region, `power`, whose `off` moves `on` under `when`.
pub fn one_region(when: Guard) -> Statechart {
    let off = State {
        transitions: vec![transition(when, "on")],
        ..state()
    };
    chart(
        "lamp",
        vec![("power", region("off", vec![("off", off), ("on", state())]))],
    )
}

/// The templates the lamp runs: the bulb.
pub fn templates() -> BTreeMap<String, Statechart> {
    BTreeMap::from([("bulb".to_string(), bulb())])
}

/// The proofs the toy's reader names: `light` proved by `lamp.lit`, `cool`
/// by `bulb.cool`.
pub fn proofs() -> BTreeMap<String, String> {
    BTreeMap::from([
        ("light".to_string(), "lamp.lit".to_string()),
        ("cool".to_string(), "bulb.cool".to_string()),
    ])
}

/// The time `minutes` after 2026-09-04T12:00:00Z.
pub fn at(minutes: i64) -> OffsetDateTime {
    OffsetDateTime::parse("2026-09-04T12:00:00Z", &Rfc3339).expect("a time")
        + Duration::minutes(minutes)
}

/// A snapshot read at `at` holding nothing else.
pub fn snapshot(at: OffsetDateTime) -> Snapshot {
    Snapshot {
        at,
        evidence: BTreeMap::new(),
        regions: BTreeMap::new(),
        responses: Vec::new(),
        owner: None,
        children: Vec::new(),
    }
}

/// `snapshot` with the object's evidence given.
pub fn with_evidence(snapshot: Snapshot, evidence: &[(&str, Value)]) -> Snapshot {
    let mut snapshot = snapshot;
    for (name, value) in evidence {
        snapshot.evidence.insert(name.to_string(), value.clone());
    }
    snapshot
}

/// `snapshot` with responses, each an id and its answer, in order.
pub fn with_responses(snapshot: Snapshot, responses: &[(&str, &str)]) -> Snapshot {
    let mut snapshot = snapshot;
    for (id, answer) in responses {
        snapshot.responses.push(Response {
            id: id.to_string(),
            answer: answer.to_string(),
        });
    }
    snapshot
}

/// A related object of `kind` holding `states`.
pub fn related(kind: &str, states: &[&str]) -> Related {
    Related {
        kind: kind.into(),
        states: states.iter().map(|s| s.to_string()).collect(),
    }
}

/// An object's state holding each region path in its state, each entered
/// at `at`.
pub fn holding(regions: &[(&str, &str)], at: OffsetDateTime) -> ObjectState {
    let mut state = ObjectState::default();
    for (path, held) in regions {
        state.regions.insert(path.to_string(), held.to_string());
        state.entered_at.insert(path.to_string(), at);
    }
    state
}

/// A statechart of version 1 holding `regions`.
pub fn chart(name: &str, regions: Vec<(&str, Region)>) -> Statechart {
    Statechart {
        name: name.into(),
        version: 1,
        regions: regions
            .into_iter()
            .map(|(name, region)| (name.to_string(), region))
            .collect(),
    }
}

/// A region starting in `initial`, holding `states`.
pub fn region(initial: &str, states: Vec<(&str, State)>) -> Region {
    Region {
        initial: initial.into(),
        states: states
            .into_iter()
            .map(|(name, state)| (name.to_string(), state))
            .collect(),
        sections: Table::new(),
    }
}

/// A state holding nothing.
pub fn state() -> State {
    State {
        is_final: false,
        decision: None,
        entry: Vec::new(),
        exit: Vec::new(),
        machine: None,
        params: None,
        enter: BTreeMap::new(),
        regions: BTreeMap::new(),
        transitions: Vec::new(),
        sections: Table::new(),
    }
}

/// A transition to `to` under `when`, holding nothing else.
pub fn transition(when: Guard, to: &str) -> Transition {
    Transition {
        when,
        to: to.into(),
        effects: Vec::new(),
        bump: None,
        enter: BTreeMap::new(),
        sections: Table::new(),
    }
}

/// An effect of `atom` with no arguments.
pub fn effect(atom: &str) -> Effect {
    Effect {
        atom: atom.into(),
        args: Table::new(),
        sections: Table::new(),
    }
}

/// An effect of `atom` with string arguments.
pub fn effect_with(atom: &str, args: &[(&str, &str)]) -> Effect {
    Effect {
        args: args
            .iter()
            .map(|(k, v)| (k.to_string(), Value::String(v.to_string())))
            .collect(),
        ..effect(atom)
    }
}

/// `{ ev = name, <test> }`.
pub fn ev(name: &str, test: Test) -> Guard {
    Guard::Evidence {
        ev: name.into(),
        test,
    }
}

/// `{ ev = name, is = "<word>" }`.
pub fn is(name: &str, word: &str) -> Guard {
    ev(name, Test::Is(Value::String(word.into())))
}

/// `{ ev = name, is = true }`.
pub fn is_true(name: &str) -> Guard {
    ev(name, Test::Is(Value::Boolean(true)))
}

/// `{ response = pattern }`.
pub fn response(pattern: &str) -> Guard {
    Guard::Response(pattern.into())
}

/// `{ children = { kind, <test> } }`.
pub fn children(kind: &str, test: ChildrenTest) -> Guard {
    Guard::Children {
        kind: kind.into(),
        test,
    }
}
