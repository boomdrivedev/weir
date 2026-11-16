//! The tick shape's cases (C.7): each holds, in its folder, `case.toml`
//! alone, naming its clauses, a declaration and the statechart kind of it
//! ticked, the proofs its reader names, one object's state, one snapshot
//! and what the tick must give: the transitions that fire, each with its
//! effects, and the next state. A field the form does not name fails the
//! read, naming the case and the field.
//!
//! The declaration is read by `read-kinds`, the one parser of kinds, and
//! every statechart kind of it but the one ticked is a template, by its
//! name (core.30, core.54). The case is ticked by weir's engine (96 to 99;
//! core.35).

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;

use serde::Deserialize;
use time::format_description::well_known::Rfc3339;
use time::{Date, Month, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset};
use toml::value::{Datetime, Offset};
use toml::{Table, Value};
use weir_engine::{
    Applied, Fired, ObjectState, Phase, Planned, Related, Response, Snapshot, Statechart, Tick,
};

use crate::clauses::Clause;
use crate::pack::Case;

/// The one file a tick case holds.
const CASE_FILE: &str = "case.toml";

/// A tick case, read.
#[derive(Clone, Debug, PartialEq)]
pub struct TickCase {
    /// `tick/<name>`.
    pub id: String,
    pub name: String,
    pub satisfies: Vec<Clause>,
    /// The statechart kind ticked, as `read-kinds` reads it.
    pub chart: Statechart,
    /// Every other statechart kind of the declaration, by its name.
    pub templates: BTreeMap<String, Statechart>,
    /// The evidence that proves each effect, by its atom.
    pub proofs: BTreeMap<String, String>,
    /// The object's state ticked.
    pub state: ObjectState,
    /// The snapshot ticked.
    pub snapshot: Snapshot,
    /// What the tick must give.
    pub expected: Tick,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Written {
    satisfies: Vec<Value>,
    declaration: String,
    chart: String,
    #[serde(default)]
    proofs: BTreeMap<String, String>,
    state: WrittenState,
    snapshot: WrittenSnapshot,
    expect: WrittenExpect,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct WrittenState {
    regions: BTreeMap<String, String>,
    #[serde(default)]
    entered_at: BTreeMap<String, Datetime>,
    #[serde(default)]
    counters: BTreeMap<String, u64>,
    #[serde(default)]
    applied: Vec<String>,
    #[serde(default)]
    set_aside: BTreeMap<String, BTreeMap<String, String>>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct WrittenSnapshot {
    at: Datetime,
    #[serde(default)]
    evidence: BTreeMap<String, Value>,
    #[serde(default)]
    regions: BTreeMap<String, BTreeMap<String, Value>>,
    #[serde(default)]
    responses: Vec<WrittenResponse>,
    owner: Option<WrittenRelated>,
    #[serde(default)]
    children: Vec<WrittenRelated>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct WrittenResponse {
    id: String,
    answer: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct WrittenRelated {
    kind: String,
    states: Vec<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct WrittenExpect {
    fired: Vec<WrittenFired>,
    next: WrittenState,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct WrittenFired {
    region: String,
    from: String,
    to: String,
    bump: Option<String>,
    response: Option<WrittenApplied>,
    #[serde(default)]
    effects: Vec<WrittenPlanned>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct WrittenApplied {
    id: String,
    argument: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct WrittenPlanned {
    #[serde(rename = "do")]
    atom: String,
    region: String,
    phase: String,
    #[serde(default)]
    args: Table,
}

/// Reads the tick case `case`, or names the case and why it does not read:
/// a file it holds that a tick case does not, a field the form does not
/// name or of the wrong shape, a declaration `read-kinds` refuses, with
/// its refusal, a `chart` naming no statechart kind of the declaration, a
/// time with no offset, or a phase that is none of an effect's.
pub fn read_tick_case(case: &Case) -> Result<TickCase, String> {
    let id = case.id();
    let refused = |why: String| format!("{id}: {why}");
    if let Some(stray) = case.files.keys().find(|path| *path != CASE_FILE) {
        return Err(refused(format!(
            "holds `{stray}`, and a tick case holds `{CASE_FILE}` alone"
        )));
    }
    let written: Written = toml::from_str(case.text(CASE_FILE)?)
        .map_err(|error| refused(error.message().to_string()))?;
    let satisfies = written
        .satisfies
        .iter()
        .map(Clause::cited)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|why| refused(format!("`satisfies`: {why}")))?;
    if satisfies.is_empty() {
        return Err(refused("cites no clause in `satisfies`".into()));
    }
    let declaration = weir_kinds::read_kinds(&written.declaration)
        .map_err(|refusal| refused(format!("`declaration` is refused by read-kinds: {refusal}")))?;
    let mut statecharts: BTreeMap<String, Statechart> = declaration
        .statecharts
        .into_iter()
        .map(|(name, kind)| (name, kind.statechart))
        .collect();
    let Some(chart) = statecharts.remove(&written.chart) else {
        let what = if declaration.kinds.contains_key(&written.chart) {
            "a simple kind of the declaration"
        } else {
            "no kind of the declaration"
        };
        return Err(refused(format!(
            "`chart`: `{}` is {what}, and a tick case ticks a statechart kind",
            written.chart
        )));
    };
    let state = object_state(written.state, "state").map_err(&refused)?;
    let snapshot = snapshot(written.snapshot).map_err(&refused)?;
    let fired = written
        .expect
        .fired
        .into_iter()
        .enumerate()
        .map(|(n, fired)| self::fired(fired, &format!("expect.fired[{n}]")))
        .collect::<Result<Vec<_>, _>>()
        .map_err(&refused)?;
    let next = object_state(written.expect.next, "expect.next").map_err(&refused)?;
    Ok(TickCase {
        id: id.clone(),
        name: case.name.clone(),
        satisfies,
        chart,
        templates: statecharts,
        proofs: written.proofs,
        state,
        snapshot,
        expected: Tick { fired, next },
    })
}

/// Ticks `case`'s chart twice with weir's engine, each time from copies
/// of its templates, proofs, state and snapshot, and compares the first
/// tick with what the case expects; or names the case and the field of
/// each difference: a transition or an effect other than expected, a part
/// of the next state other than expected, the two ticks unequal, an input
/// changed by the tick, or the tick refused (65, 96, 98, 99).
pub fn run_tick_case(case: &TickCase) -> Result<(), Vec<String>> {
    let mut said: Vec<String> = Vec::new();
    let mut say = |field: &str, what: String| said.push(format!("{}: {field}: {what}", case.id));
    let tick_copies = || {
        let (chart, templates, proofs, state, snapshot) = (
            case.chart.clone(),
            case.templates.clone(),
            case.proofs.clone(),
            case.state.clone(),
            case.snapshot.clone(),
        );
        let tick = weir_engine::tick(&chart, &templates, &proofs, &state, &snapshot);
        (tick, (chart, templates, proofs, state, snapshot))
    };
    let (first, inputs) = tick_copies();
    let (second, _) = tick_copies();
    let (chart, templates, proofs, state, snapshot) = inputs;
    for (field, changed) in [
        ("chart", chart != case.chart),
        ("templates", templates != case.templates),
        ("proofs", proofs != case.proofs),
        ("state", state != case.state),
        ("snapshot", snapshot != case.snapshot),
    ] {
        if changed {
            say(field, "an input the tick changed".into());
        }
    }
    if first != second {
        say("tick", "the two ticks of one case are unequal".into());
    }
    let tick = match first {
        Ok(tick) => tick,
        Err(refusal) => {
            say("tick", format!("refused: {refusal}"));
            return Err(said);
        }
    };
    let expected = &case.expected;
    if tick.fired.len() != expected.fired.len() {
        say(
            "fired",
            format!(
                "expected {} transitions, ticked {}: {}",
                expected.fired.len(),
                tick.fired.len(),
                moves(&tick.fired)
            ),
        );
    }
    for (n, (fired, wanted)) in tick.fired.iter().zip(&expected.fired).enumerate() {
        let at = |part: &str| format!("fired[{n}].{part}");
        let mut part = |name: &str, ticked: String, wanted: String| {
            if ticked != wanted {
                say(&at(name), format!("expected {wanted}, ticked {ticked}"));
            }
        };
        part("region", fired.region.clone(), wanted.region.clone());
        part("from", fired.from.clone(), wanted.from.clone());
        part("to", fired.to.clone(), wanted.to.clone());
        part(
            "bump",
            format!("{:?}", fired.bump),
            format!("{:?}", wanted.bump),
        );
        part(
            "response",
            format!(
                "{:?}",
                fired.response.as_ref().map(|r| (&r.id, &r.argument))
            ),
            format!(
                "{:?}",
                wanted.response.as_ref().map(|r| (&r.id, &r.argument))
            ),
        );
        part("effects", effects(&fired.effects), effects(&wanted.effects));
    }
    let next = &tick.next;
    let wanted = &expected.next;
    differ(
        &mut say,
        "next.regions",
        &next.regions,
        &wanted.regions,
        |held| format!("`{held}`"),
    );
    differ(
        &mut say,
        "next.entered_at",
        &next.entered_at,
        &wanted.entered_at,
        |time| time.format(&Rfc3339).unwrap_or_else(|_| time.to_string()),
    );
    differ(
        &mut say,
        "next.counters",
        &next.counters,
        &wanted.counters,
        |n| n.to_string(),
    );
    if next.applied != wanted.applied {
        say(
            "next.applied",
            format!("expected {:?}, ticked {:?}", wanted.applied, next.applied),
        );
    }
    differ(
        &mut say,
        "next.set_aside",
        &next.set_aside,
        &wanted.set_aside,
        |instance| format!("{instance:?}"),
    );
    if said.is_empty() {
        Ok(())
    } else {
        Err(said)
    }
}

/// Each key of two maps whose values differ, named under `field`.
fn differ<V: PartialEq + Debug>(
    say: &mut impl FnMut(&str, String),
    field: &str,
    ticked: &BTreeMap<String, V>,
    wanted: &BTreeMap<String, V>,
    show: impl Fn(&V) -> String,
) {
    let keys: BTreeSet<&String> = ticked.keys().chain(wanted.keys()).collect();
    for key in keys {
        let (ticked, wanted) = (ticked.get(key), wanted.get(key));
        if ticked != wanted {
            let show = |value: Option<&V>| value.map(&show).unwrap_or_else(|| "none".into());
            say(
                &format!("{field}.{key}"),
                format!("expected {}, ticked {}", show(wanted), show(ticked)),
            );
        }
    }
}

/// The transitions fired, each `<region>: <from> to <to>`.
fn moves(fired: &[Fired]) -> String {
    let each: Vec<String> = fired
        .iter()
        .map(|fired| format!("{}: {} to {}", fired.region, fired.from, fired.to))
        .collect();
    format!("[{}]", each.join(", "))
}

/// Effects planned, as a case writes them.
fn effects(effects: &[Planned]) -> String {
    let each: Vec<String> = effects
        .iter()
        .map(|effect| {
            let mut written = format!(
                "{{ do = {:?}, region = {:?}, phase = {:?}",
                effect.atom,
                effect.region,
                spelled(effect.phase)
            );
            if !effect.args.is_empty() {
                written.push_str(&format!(
                    ", args = {{ {} }}",
                    effect.args.to_string().trim().replace('\n', ", ")
                ));
            }
            written.push_str(" }");
            written
        })
        .collect();
    format!("[{}]", each.join(", "))
}

/// An object's state as a case writes it, at `at`.
fn object_state(written: WrittenState, at: &str) -> Result<ObjectState, String> {
    let entered_at = written
        .entered_at
        .iter()
        .map(|(path, time)| {
            Ok((
                path.clone(),
                instant(time, &format!("{at}.entered_at.{path}"))?,
            ))
        })
        .collect::<Result<_, String>>()?;
    Ok(ObjectState {
        regions: written.regions,
        entered_at,
        counters: written.counters,
        applied: written.applied,
        set_aside: written.set_aside,
    })
}

/// A snapshot as a case writes it.
fn snapshot(written: WrittenSnapshot) -> Result<Snapshot, String> {
    let related = |related: WrittenRelated| Related {
        kind: related.kind,
        states: related.states,
    };
    Ok(Snapshot {
        at: instant(&written.at, "snapshot.at")?,
        evidence: written.evidence,
        regions: written.regions,
        responses: written
            .responses
            .into_iter()
            .map(|response| Response {
                id: response.id,
                answer: response.answer,
            })
            .collect(),
        owner: written.owner.map(related),
        children: written.children.into_iter().map(related).collect(),
    })
}

/// A fired transition as a case expects it, at `at`.
fn fired(written: WrittenFired, at: &str) -> Result<Fired, String> {
    let effects = written
        .effects
        .into_iter()
        .enumerate()
        .map(|(n, effect)| {
            Ok(Planned {
                phase: phase(&effect.phase, &format!("{at}.effects[{n}].phase"))?,
                atom: effect.atom,
                args: effect.args,
                region: effect.region,
            })
        })
        .collect::<Result<_, String>>()?;
    Ok(Fired {
        region: written.region,
        from: written.from,
        to: written.to,
        bump: written.bump,
        response: written.response.map(|applied| Applied {
            id: applied.id,
            argument: applied.argument,
        }),
        effects,
    })
}

/// An effect's phase, by its spelling.
fn phase(text: &str, at: &str) -> Result<Phase, String> {
    Ok(match text {
        "exit" => Phase::Exit,
        "transition" => Phase::Transition,
        "entry" => Phase::Entry,
        "enter" => Phase::Enter,
        _ => {
            return Err(format!(
                "`{at}`: `{text}` is no phase, which are exit, transition, entry and enter"
            ))
        }
    })
}

/// A phase as a case spells it.
fn spelled(phase: Phase) -> &'static str {
    match phase {
        Phase::Exit => "exit",
        Phase::Transition => "transition",
        Phase::Entry => "entry",
        Phase::Enter => "enter",
    }
}

/// A datetime a case writes, with its date, time and offset, as an
/// instant; one with no offset is refused naming `at`.
fn instant(datetime: &Datetime, at: &str) -> Result<OffsetDateTime, String> {
    let refused = || format!("`{at}`: `{datetime}` is no time with its offset");
    let (Some(date), Some(time), Some(offset)) = (datetime.date, datetime.time, datetime.offset)
    else {
        return Err(refused());
    };
    let date = Month::try_from(date.month)
        .ok()
        .and_then(|month| Date::from_calendar_date(i32::from(date.year), month, date.day).ok());
    let time = Time::from_hms_nano(
        time.hour,
        time.minute,
        time.second.unwrap_or(0),
        time.nanosecond.unwrap_or(0),
    )
    .ok();
    let offset = match offset {
        Offset::Z => Some(UtcOffset::UTC),
        Offset::Custom { minutes } => UtcOffset::from_whole_seconds(i32::from(minutes) * 60).ok(),
    };
    match (date, time, offset) {
        (Some(date), Some(time), Some(offset)) => {
            Ok(PrimitiveDateTime::new(date, time).assume_offset(offset))
        }
        _ => Err(refused()),
    }
}
