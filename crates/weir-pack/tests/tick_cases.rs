//! The tick cases, held to tick-cases' "A tick case is read whole", "A
//! tick case's statechart is read by the kind format", "A tick case is
//! ticked twice and compared" and "The pack holds the tick cases" (C.7;
//! 96 to 99; S21).

use weir_pack::{read_tick_case, run_tick_case, Pack, Shape};

/// The case `tick/<name>` of a pack holding `files`, each by its path from
/// the case's folder.
fn case_of(name: &str, files: &[(&str, &str)]) -> weir_pack::Case {
    let pack = Pack::from_files(
        files
            .iter()
            .map(|(path, text)| (format!("tick/{name}/{path}"), text.as_bytes().to_vec())),
    );
    pack.case(Shape::Tick, name).expect("the case")
}

/// S21's first tick: S21's lamp `off`, `lamp.switch` read `up` and
/// `lamp.lit` not read; `off` moves `on`, planning `light` at entry.
const S21: &str = r#"satisfies = [96, 98, 99]
declaration = """
[kinds.lamp]
version = 1
title = "Lamp"

[kinds.lamp.regions.power]
initial = "off"

[kinds.lamp.regions.power.states.off]
transitions = [{ when = { ev = "lamp.switch", is = "up" }, to = "on" }]

[kinds.lamp.regions.power.states.on]
entry = [{ do = "light" }]

[kinds.switch]
version = 1

[[kinds.switch.stages]]
name = "flip"
prompt = "prompts/flip.md"
deliverables = ["commits"]
"""
chart = "lamp"
proofs = { light = "lamp.lit" }

[state]
regions = { power = "off" }
entered_at = { power = 2026-09-04T12:00:00Z }

[snapshot]
at = 2026-09-04T12:01:00Z
evidence = { "lamp.switch" = "up" }

[expect]
fired = [
  { region = "power", from = "off", to = "on", effects = [
    { do = "light", region = "power", phase = "entry" },
  ] },
]
next = { regions = { power = "on" }, entered_at = { power = 2026-09-04T12:01:00Z } }
"#;

#[test]
fn a_well_formed_case_reads() {
    let read = read_tick_case(&case_of("s21", &[("case.toml", S21)])).unwrap();
    assert_eq!(read.id, "tick/s21");
    assert_eq!(read.chart.name, "lamp");
    assert!(read.templates.is_empty(), "a simple kind is no template");
    assert_eq!(read.expected.fired.len(), 1);
}

#[test]
fn a_misspelt_field() {
    let misspelt = S21.replace(
        "evidence = { \"lamp.switch\"",
        "evidense = { \"lamp.switch\"",
    );
    let why = read_tick_case(&case_of("misspelt", &[("case.toml", &misspelt)])).unwrap_err();
    assert!(why.starts_with("tick/misspelt: "), "{why}");
    assert!(why.contains("evidense"), "{why}");
    let top = format!("{S21}\nchrat = \"lamp\"\n");
    assert!(read_tick_case(&case_of("top", &[("case.toml", &top)]))
        .unwrap_err()
        .contains("chrat"));
    let stray = read_tick_case(&case_of(
        "stray",
        &[("case.toml", S21), ("notes.md", "a note")],
    ))
    .unwrap_err();
    assert!(stray.contains("notes.md"), "{stray}");
}

#[test]
fn a_chart_that_is_no_statechart() {
    for (name, chart) in [("simple", "switch"), ("none", "shade")] {
        let toml = S21.replace("chart = \"lamp\"", &format!("chart = \"{chart}\""));
        let why = read_tick_case(&case_of(name, &[("case.toml", &toml)])).unwrap_err();
        assert!(why.starts_with(&format!("tick/{name}: `chart`: ")), "{why}");
        assert!(why.contains(chart), "{why}");
    }
}

#[test]
fn a_declaration_the_kind_format_refuses() {
    let toml = S21.replace(
        "is = \"up\" }, to = \"on\" }",
        "is = \"up\" }, to = \"lit\" }",
    );
    let why = read_tick_case(&case_of("refused", &[("case.toml", &toml)])).unwrap_err();
    assert!(why.starts_with("tick/refused: "), "{why}");
    assert!(
        why.contains("kinds.lamp.regions.power.states.off.transitions[0].to"),
        "{why}"
    );
}

/// What running the case `files` hold says, or that it ran with no
/// difference.
fn run(name: &str, toml: &str) -> Result<(), Vec<String>> {
    let case = read_tick_case(&case_of(name, &[("case.toml", toml)])).unwrap();
    run_tick_case(&case)
}

#[test]
fn a_case_whose_next_state_differs() {
    assert_eq!(run("s21", S21), Ok(()));
    let toml = S21.replace(
        "next = { regions = { power = \"on\" }",
        "next = { regions = { power = \"off\" }",
    );
    let said = run("differs", &toml).unwrap_err();
    assert_eq!(
        said,
        ["tick/differs: next.regions.power: expected `off`, ticked `on`"]
    );
}

#[test]
fn a_case_whose_effects_differ() {
    let toml = S21.replace(
        "{ do = \"light\", region = \"power\", phase = \"entry\" },",
        "",
    );
    let said = run("effects", &toml).unwrap_err();
    assert_eq!(said.len(), 1, "{said:?}");
    assert!(
        said[0].starts_with("tick/effects: fired[0].effects: expected [], ticked ["),
        "{said:?}"
    );
    assert!(said[0].contains("\"light\""), "{said:?}");
}

#[test]
fn a_case_read_from_its_folder() {
    let dir = tempfile::tempdir().unwrap();
    let folder = dir.path().join("tick").join("the-lamp-switched-on");
    std::fs::create_dir_all(&folder).unwrap();
    std::fs::write(folder.join("case.toml"), S21).unwrap();
    let pack = Pack::read(dir.path()).unwrap();
    assert_eq!(pack.files().len(), 1, "its folder alone");
    let cases = pack.cases(Shape::Tick);
    assert_eq!(cases.len(), 1);
    let case = read_tick_case(&cases[0]).unwrap_or_else(|why| panic!("{why}"));
    assert_eq!(case.id, "tick/the-lamp-switched-on");
    assert_eq!(run_tick_case(&case), Ok(()));
}

/// The cases design 17 names, each with the clauses it cites.
const THE_CASES: [(&str, &[u32]); 15] = [
    ("the-lamp-switched-on", &[96, 98, 99]),
    ("the-lamp-on-with-its-proof-present", &[96, 98, 99]),
    ("an-effect-whose-proof-is-present", &[96]),
    ("the-first-transition-whose-guard-holds", &[96]),
    ("innermost-first", &[96]),
    ("a-sibling-sees-a-move-at-the-next-tick", &[96]),
    ("a-transition-to-its-own-state-with-nothing-to-do", &[96]),
    ("a-template-started-afresh", &[97]),
    ("a-final-state-of-what-a-state-runs", &[96, 97]),
    ("an-enter-moves-what-a-state-ran", &[96, 97]),
    ("a-response-applied-once", &[97]),
    ("a-response-already-applied", &[97]),
    ("a-response-binds-its-argument", &[96, 97]),
    ("a-guard-by-age", &[96]),
    ("evidence-read-at-its-region", &[96]),
];

#[test]
fn the_cases_named() {
    use std::collections::BTreeSet;
    use weir_pack::Clause;

    let pack = Pack::embedded();
    let names: BTreeSet<String> = pack
        .cases(Shape::Tick)
        .into_iter()
        .map(|case| case.name)
        .collect();
    let named: BTreeSet<String> = THE_CASES.iter().map(|(name, _)| name.to_string()).collect();
    assert_eq!(names, named, "the pack's tick cases are design 17's");
    for (name, cites) in THE_CASES {
        let case = pack.case(Shape::Tick, name).expect("the case");
        let read = read_tick_case(&case).unwrap_or_else(|why| panic!("{why}"));
        let satisfies: Vec<Clause> = cites.iter().copied().map(Clause::number).collect();
        assert_eq!(read.satisfies, satisfies, "{name}");
        assert_eq!(read.chart.name, "lamp", "{name}");
        let atoms_of = |chart: &weir_engine::Statechart| {
            let names = weir_engine::names(chart);
            names
                .evidence
                .into_iter()
                .chain(names.effects)
                .chain(names.machines)
                .collect::<Vec<_>>()
        };
        for atom in std::iter::once(&read.chart)
            .chain(read.templates.values())
            .flat_map(atoms_of)
        {
            assert!(
                atom.starts_with("lamp.")
                    || atom.starts_with("bulb.")
                    || ["light", "set_mode", "cool", "bulb"].contains(&atom.as_str()),
                "{name} names `{atom}`, no atom of the lamp or the bulb"
            );
        }
    }
}

#[test]
fn the_tick_cases() {
    let pack = Pack::embedded();
    let cases = pack.cases(Shape::Tick);
    assert_eq!(cases.len(), THE_CASES.len());
    let failed: Vec<String> = cases
        .iter()
        .flat_map(|case| match read_tick_case(case) {
            Err(why) => vec![why],
            Ok(read) => run_tick_case(&read).err().unwrap_or_default(),
        })
        .collect();
    assert_eq!(failed, Vec::<String>::new());
}
