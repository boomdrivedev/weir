//! A statechart kind: a `[kinds.<kind>]` table holding `regions` and no
//! `stages`, read by `read-kinds` into the engine's types beside the simple
//! kinds and checked whole, every failing field refused by its path (51,
//! 52, 57, 81, 94, 95; C.4; core.54).

use weir_kinds::toml::Table;
use weir_kinds::{read_kinds, Declaration, Refusal, StatechartKind};

/// The declaration `text` read, which must read.
fn reads(text: &str) -> Declaration {
    read_kinds(text).unwrap_or_else(|refusal| panic!("refused:\n{refusal}"))
}

/// The declaration `text` refused, which must be refused.
fn refused(text: &str) -> Refusal {
    match read_kinds(text) {
        Ok(declaration) => panic!("read: {declaration:?}"),
        Err(refusal) => refusal,
    }
}

/// The fields a refusal names, in its order.
fn fields(refusal: &Refusal) -> Vec<&str> {
    refusal.problems.iter().map(|p| p.field.as_str()).collect()
}

/// The one statechart kind `name` of the declaration `text`.
fn statechart(text: &str, name: &str) -> StatechartKind {
    reads(text).statecharts[name].clone()
}

fn table(text: &str) -> Table {
    text.parse().unwrap()
}

/// C.4's statechart kind `persona`, as C.4 writes it.
const PERSONA: &str = r#"
[kinds.persona]
version = 1
title = "Persona test"
branch = "persona"

[kinds.persona.regions.stages]
initial = "build"

[kinds.persona.regions.stages.states.build]
machine = "stage"
params = { agents = [{ model = "opus" }], join = "all", on_fail = "build", max_send_backs = 2, deliverables = ["commits"] }
transitions = [
  { when = { final = "passed" }, to = "test" },
  { when = { final = "stopped" }, to = "stopped" },
]

[kinds.persona.regions.stages.states.test]
machine = "stage"
params = { agents = { rule = "every-persona-definition", pattern = "personas/*.md" }, join = "all", on_fail = "build", max_send_backs = 2, deliverables = ["committed:tests/personas.md"] }
transitions = [
  { when = { final = "passed" }, to = "passed" },
  { when = { final = "sent-back" }, to = "build" },
  { when = { final = "stopped" }, to = "stopped" },
]

[kinds.persona.regions.stages.states.passed]
final = true

[kinds.persona.regions.stages.states.stopped]
decision = { kind = "stalled", answers = ["retry", "drop"] }
transitions = [
  { when = { response = "retry" }, to = "build" },
  { when = { response = "drop" }, to = "dropped" },
]

[kinds.persona.regions.stages.states.dropped]
final = true
"#;

/// C.4's simple kind `bump`, its stages as C.4 writes them.
const BUMP: &str = r#"
[kinds.bump]
version = 2
branch = "bump"
starts_unready = false
lands = true

[kinds.bump.agents]
model = "sonnet"

[[kinds.bump.stages]]
name = "code"
prompt = "prompts/bump-code.md"
standing = "prompts/bump-code-standing.md"
deliverables = ["commits"]
agents = [{ program = "claude", model = "opus" }]
quiet = "45m"
makes = ["preview"]
hibernate = true

[[kinds.bump.stages]]
name = "build"
prompt = "prompts/bump-build.md"
deliverables = ["tasks"]
repeat = "per-task-group"

[[kinds.bump.stages]]
name = "review"
prompt = "prompts/bump-review.md"
conversation = true
wait = true
agents = [{ beside = "operator" }]
on_fail = "code"
max_send_backs = 3
"#;

/// One region `main` starting at, and holding, the final state `done`.
const ONE_REGION: &str = r#"
regions.main.initial = "done"
regions.main.states.done.final = true
"#;

/// A statechart kind `name`, its table's own `fields` beside one region.
fn kind(name: &str, fields: &str) -> String {
    format!("[kinds.{name}]\n{fields}\n{ONE_REGION}")
}

// A kind's table holding regions is a statechart kind.

#[test]
fn read_kinds_gives_the_engines_types() {
    let persona = statechart(PERSONA, "persona");
    let chart: &weir_kinds::weir_engine::Statechart = &persona.statechart;
    assert_eq!(chart.name, "persona");
    assert_eq!(chart.version, 1);
    assert_eq!(persona.name(), "persona");
    assert_eq!(persona.version(), 1);
    let region: &weir_kinds::weir_engine::Region = &chart.regions["stages"];
    assert_eq!(region.initial, "build");
}

// A table holding both stages and regions is refused.

#[test]
fn a_table_holding_both_stages_and_regions() {
    let both = r#"
[kinds.both]
version = 0
title = ""

[[kinds.both.stages]]
name = "code"
prompt = "prompts/code.md"
deliverables = ["commits"]

[kinds.both.regions.main]
initial = "nowhere"
states.a.final = "yes"
"#;
    let refusal = refused(&format!("{BUMP}\n{PERSONA}\n{both}"));
    assert_eq!(fields(&refusal), ["kinds.both.regions"]);
    assert!(
        refusal.problems[0].reason.contains("beside `stages`"),
        "{refusal}"
    );
}

// A table holding agents and regions is a statechart kind.

#[test]
fn agents_beside_regions() {
    let declaration = reads(&format!(
        "{}\n[kinds.persona.agents]\nmodel = \"opus\"",
        kind("persona", "title = \"Persona test\"")
    ));
    assert!(declaration.settings.is_empty());
    assert!(declaration.kinds.is_empty());
    let persona = &declaration.statecharts["persona"];
    assert_eq!(persona.sections, table("[agents]\nmodel = \"opus\""));
}

// A simple kind reads as it did.

#[test]
fn a_simple_kinds_title_and_label_are_sections() {
    let declaration = reads(
        r#"
[kinds.bump]
title = "Bump"
label = "construction"

[[kinds.bump.stages]]
name = "code"
prompt = "prompts/code.md"
deliverables = ["commits"]
"#,
    );
    assert!(declaration.statecharts.is_empty());
    let bump = &declaration.kinds["bump"];
    assert_eq!(bump.stages.len(), 1);
    assert_eq!(
        bump.sections,
        table("title = \"Bump\"\nlabel = \"construction\"")
    );
}

// A statechart kind reads its own fields.

#[test]
fn a_statechart_kind_reads_with_every_field() {
    let persona = statechart(
        &kind(
            "persona",
            r#"
version = 2
title = "Persona test"
label = "construction"
branch = "persona"
lands = true
landing_fix = "acme/fix@1"
max_landing_fixes = 2
owner = "release"
"#,
        ),
        "persona",
    );
    assert_eq!(persona.version(), 2);
    assert_eq!(persona.title, "Persona test");
    assert_eq!(persona.label.as_deref(), Some("construction"));
    assert_eq!(persona.branch, "persona");
    assert!(persona.lands);
    assert_eq!(
        persona.landing_fix.as_ref().map(ToString::to_string),
        Some("acme/fix@1".to_owned())
    );
    assert_eq!(persona.max_landing_fixes, Some(2));
    assert_eq!(persona.sections, table("owner = \"release\""));
}

#[test]
fn a_statechart_kinds_fields_left_out() {
    let persona = statechart(&kind("persona", "title = \"Persona test\""), "persona");
    assert_eq!(persona.version(), 1);
    assert_eq!(persona.branch, "persona");
    assert!(!persona.lands);
    assert_eq!(persona.label, None);
    assert_eq!(persona.landing_fix, None);
    assert_eq!(persona.max_landing_fixes, None);
    assert!(persona.sections.is_empty());
}

// A statechart kind holds its title.

#[test]
fn a_statechart_kind_with_no_title() {
    let refusal = refused(&format!(
        "{}\n{}",
        kind("untitled", ""),
        kind("blank", "title = \"\"")
    ));
    assert_eq!(
        fields(&refusal),
        ["kinds.blank.title", "kinds.untitled.title"]
    );
}

// Only a statechart kind that lands names its landing fix.

#[test]
fn a_landing_fix_on_a_statechart_kind_that_does_not_land() {
    let refusal = refused(&kind(
        "persona",
        "title = \"Persona test\"\nlanding_fix = \"acme/fix@1\"\nmax_landing_fixes = 2",
    ));
    assert_eq!(
        fields(&refusal),
        [
            "kinds.persona.landing_fix",
            "kinds.persona.max_landing_fixes"
        ]
    );
    for problem in &refusal.problems {
        assert!(problem.reason.contains("does not land"), "{problem}");
    }
}

// A statechart holds regions and each region states.

#[test]
fn a_statechart_kind_with_no_region() {
    let refusal = refused("[kinds.persona]\ntitle = \"Persona test\"\nregions = {}");
    assert_eq!(fields(&refusal), ["kinds.persona.regions"]);
}

#[test]
fn a_region_with_no_states() {
    let refusal = refused(
        r#"
[kinds.persona]
title = "Persona test"
regions.a.initial = "a"
regions.b.initial = "a"
regions.b.states = {}
"#,
    );
    assert_eq!(
        fields(&refusal),
        [
            "kinds.persona.regions.a.states",
            "kinds.persona.regions.b.states"
        ]
    );
}

// A region's initial state is one it holds.

#[test]
fn an_initial_state_the_region_does_not_hold() {
    let refusal = refused(
        r#"
[kinds.persona]
title = "Persona test"
regions.main.initial = "c"
regions.main.states.a = {}
regions.main.states.b.final = true
"#,
    );
    assert_eq!(fields(&refusal), ["kinds.persona.regions.main.initial"]);
    assert!(refusal.problems[0].reason.contains("`c`"), "{refusal}");
}

// A region's and a state's key is a name.

#[test]
fn a_state_named_by_a_key_that_is_not_a_name() {
    let refusal = refused(
        r#"
[kinds.persona]
title = "Persona test"
regions.main.initial = "a"
regions.main.states.a.final = true
regions.main.states."a.b".final = "yes"
regions.main.states."a.b".regions = 1
"#,
    );
    assert_eq!(
        fields(&refusal),
        [r#"kinds.persona.regions.main.states."a.b""#]
    );
    assert!(
        refusal.problems[0]
            .reason
            .contains("a state's name is one or more ASCII letters, digits, `_` and `-`"),
        "{refusal}"
    );
}

// A state's own regions are checked as any region.

#[test]
fn a_nested_region_is_checked_as_any() {
    let nested = r#"
[kinds.persona]
title = "Persona test"
regions.main.initial = "build"

[kinds.persona.regions.main.states.build]
machine = "checks"
params = { lint = true }
enter = { inner = "start" }
regions.checks.initial = "lint"
regions.checks.states.lint.final = true
"#;
    let persona = statechart(nested, "persona");
    let build = &persona.statechart.regions["main"].states["build"];
    assert_eq!(build.machine.as_deref(), Some("checks"));
    assert_eq!(build.params, Some(table("lint = true")));
    assert_eq!(build.enter["inner"], "start");
    let checks = &build.regions["checks"];
    assert_eq!(checks.initial, "lint");
    assert!(checks.states["lint"].is_final);

    let refusal = refused(&nested.replace(
        "regions.checks.initial = \"lint\"",
        "regions.checks.initial = \"test\"",
    ));
    assert_eq!(
        fields(&refusal),
        ["kinds.persona.regions.main.states.build.regions.checks.initial"]
    );
}

// A transition goes to a state its region holds.

#[test]
fn a_transition_to_a_state_its_region_does_not_hold() {
    let refusal = refused(&PERSONA.replace(
        r#"{ when = { final = "passed" }, to = "passed" }"#,
        r#"{ when = { final = "passed" }, to = "tested" }"#,
    ));
    assert_eq!(
        fields(&refusal),
        ["kinds.persona.regions.stages.states.test.transitions[0].to"]
    );
    assert!(refusal.problems[0].reason.contains("`tested`"), "{refusal}");
}

#[test]
fn a_transition_with_no_guard_or_target() {
    let refusal = refused(
        r#"
[kinds.persona]
title = "Persona test"
regions.main.initial = "a"
regions.main.states.b.final = true

[kinds.persona.regions.main.states.a]
transitions = [{ to = "b" }, { when = { always = true } }]
"#,
    );
    assert_eq!(
        fields(&refusal),
        [
            "kinds.persona.regions.main.states.a.transitions[0].when",
            "kinds.persona.regions.main.states.a.transitions[1].to",
        ]
    );
}

/// A statechart kind `guards` whose one state `a` has a transition to
/// itself for each of `guards`, in order.
fn guarded(guards: &[&str]) -> String {
    let transitions: Vec<String> = guards
        .iter()
        .map(|guard| format!("  {{ when = {guard}, to = \"a\" }},"))
        .collect();
    format!(
        "[kinds.guards]\ntitle = \"Guards\"\nregions.main.initial = \"a\"\n\n\
         [kinds.guards.regions.main.states.a]\ntransitions = [\n{}\n]\n",
        transitions.join("\n")
    )
}

// A guard is one of the forms C.4 names.

#[test]
fn every_form_of_guard_reads() {
    use std::time::Duration;
    use weir_kinds::toml::Value;
    use weir_kinds::weir_engine::{ChildrenTest, Guard, Test};

    let text = guarded(&[
        r#"{ all = [{ always = true }] }"#,
        r#"{ any = [{ response = "yes" }] }"#,
        r#"{ not = { final = "done" } }"#,
        r#"{ ev = "switch", is = "up" }"#,
        r#"{ ev = "n", in = [1, 2] }"#,
        r#"{ ev = "lock", exists = false }"#,
        r#"{ ev = "n", gt = 1 }"#,
        r#"{ ev = "n", gte = 1 }"#,
        r#"{ ev = "n", lt = 1 }"#,
        r#"{ ev = "n", lte = 1 }"#,
        r#"{ ev = "seen", older = "2h" }"#,
        r#"{ ev = "a", eq_ev = "b" }"#,
        r#"{ ev = "a", ne_ev = "b" }"#,
        r#"{ ev = "a", gt_ev = "b" }"#,
        r#"{ ev = "a", gte_ev = "b" }"#,
        r#"{ ev = "a", lt_ev = "b" }"#,
        r#"{ region = { name = "lamp", in = ["on"] } }"#,
        r#"{ parent = { in = ["open"] } }"#,
        r#"{ children = { kind = "task", all = ["done"] } }"#,
        r#"{ children = { kind = "task", count_gte = 2 } }"#,
    ]);
    let evidence = |ev: &str, test: Test| Guard::Evidence {
        ev: ev.to_owned(),
        test,
    };
    let one = Value::Integer(1);
    let read: Vec<Guard> = statechart(&text, "guards").statechart.regions["main"].states["a"]
        .transitions
        .iter()
        .map(|transition| transition.when.clone())
        .collect();
    assert_eq!(
        read,
        [
            Guard::All(vec![Guard::Always(true)]),
            Guard::Any(vec![Guard::Response("yes".to_owned())]),
            Guard::Not(Box::new(Guard::Final("done".to_owned()))),
            evidence("switch", Test::Is(Value::String("up".to_owned()))),
            evidence("n", Test::In(vec![one.clone(), Value::Integer(2)])),
            evidence("lock", Test::Exists(false)),
            evidence("n", Test::Gt(one.clone())),
            evidence("n", Test::Gte(one.clone())),
            evidence("n", Test::Lt(one.clone())),
            evidence("n", Test::Lte(one)),
            evidence("seen", Test::Older(Duration::from_secs(2 * 3600))),
            evidence("a", Test::EqEv("b".to_owned())),
            evidence("a", Test::NeEv("b".to_owned())),
            evidence("a", Test::GtEv("b".to_owned())),
            evidence("a", Test::GteEv("b".to_owned())),
            evidence("a", Test::LtEv("b".to_owned())),
            Guard::Region {
                name: "lamp".to_owned(),
                within: vec!["on".to_owned()],
            },
            Guard::Parent {
                within: vec!["open".to_owned()],
            },
            Guard::Children {
                kind: "task".to_owned(),
                test: ChildrenTest::All(vec!["done".to_owned()]),
            },
            Guard::Children {
                kind: "task".to_owned(),
                test: ChildrenTest::CountGte(2),
            },
        ]
    );
}

// A guard of no form is refused naming its path.

#[test]
fn a_guard_of_a_form_the_format_does_not_name() {
    let refusal = refused(&guarded(&[
        r#"{ ev = "switch" }"#,
        r#"{ ev = "n", is = 1, gt = 2 }"#,
        r#"{ all = [{ always = true }], any = [{ always = true }] }"#,
        r#"{ always = true, response = "yes" }"#,
        r#"{ children = { kind = "task" } }"#,
        r#""always""#,
        r#"{ any = [{ always = true }, { when = 1 }] }"#,
    ]));
    let at = "kinds.guards.regions.main.states.a.transitions";
    assert_eq!(
        fields(&refusal),
        [
            format!("{at}[0].when"),
            format!("{at}[1].when"),
            format!("{at}[2].when"),
            format!("{at}[3].when"),
            format!("{at}[4].when"),
            format!("{at}[5].when"),
            format!("{at}[6].when.any[1]"),
        ]
    );
    for problem in &refusal.problems {
        assert!(
            problem
                .reason
                .contains("is not a guard of a form the format names"),
            "{problem}"
        );
    }
}

// An older is a duration.

#[test]
fn an_older_that_is_no_duration() {
    let refusal = refused(&guarded(&[r#"{ ev = "seen", older = "2 days" }"#]));
    assert_eq!(
        fields(&refusal),
        ["kinds.guards.regions.main.states.a.transitions[0].when.older"]
    );
}
