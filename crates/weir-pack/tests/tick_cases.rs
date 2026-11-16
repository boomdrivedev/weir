//! The tick cases, held to tick-cases' "A tick case is read whole", "A
//! tick case's statechart is read by the kind format", "A tick case is
//! ticked twice and compared" and "The pack holds the tick cases" (C.7;
//! 96 to 99; S21).

use weir_pack::{read_tick_case, Pack, Shape};

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
