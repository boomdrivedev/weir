//! The tick and an object's first state: a region path resolved through a
//! statechart, its states' own regions and the templates they run, and
//! what a state runs started when it is entered (96, 97, 99).

use std::collections::BTreeMap;

use time::OffsetDateTime;
use toml::{Table, Value};

use crate::eval::{holds, proven, Scope};
use crate::state::depth;
use crate::{Effect, ObjectState, Refusal, Region, Snapshot, State, Statechart, Transition};

/// What one tick gives: the transitions that fire, in the order they
/// fired, and the object's next state (96).
#[derive(Debug, Clone, PartialEq)]
pub struct Tick {
    /// The transitions that fire, innermost first, at most one per region.
    pub fired: Vec<Fired>,
    /// The object's state with each fired transition applied in order.
    pub next: ObjectState,
}

/// One transition that fires (96).
#[derive(Debug, Clone, PartialEq)]
pub struct Fired {
    /// The region path it fires in.
    pub region: String,
    /// The state it leaves.
    pub from: String,
    /// The state it enters.
    pub to: String,
    /// The counter it bumps, where it names one.
    pub bump: Option<String>,
    /// The response its guard took, where it took one (97).
    pub response: Option<Applied>,
    /// The effects it plans, in order: the left state's `exit`, its own,
    /// the entered state's `entry`, then the `entry` of each state its
    /// `enter` moves a region to. An effect whose proof the snapshot holds
    /// is left out.
    pub effects: Vec<Planned>,
}

/// A response a guard took (97).
#[derive(Debug, Clone, PartialEq)]
pub struct Applied {
    /// Its id, which the next state applies once.
    pub id: String,
    /// The argument its answer bound; empty for a bare word.
    pub argument: String,
}

/// An effect planned, for its caller to perform (96, 98).
#[derive(Debug, Clone, PartialEq)]
pub struct Planned {
    /// Its atom, as written.
    pub atom: String,
    /// Its arguments as written, an argument `$response` given the
    /// argument the transition's response bound.
    pub args: Table,
    /// The region path where it is planned.
    pub region: String,
    /// Where in the firing it is planned.
    pub phase: Phase,
}

/// Where in a firing an effect is planned (96).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    /// The left state's `exit`.
    Exit,
    /// The transition's own `effects`.
    Transition,
    /// The entered state's `entry`.
    Entry,
    /// The `entry` of a state an `enter` moves a region to.
    Enter,
}

/// A transition that fires, with the regions its `enter` moves, each path
/// with the state it moves to.
struct Plan {
    fired: Fired,
    commands: Vec<(String, String)>,
}

/// A statechart with the templates it runs, by the name a state's
/// `machine` gives: what a region path is resolved through.
pub(crate) struct Charts<'a> {
    /// The statechart ticked.
    pub chart: &'a Statechart,
    /// The templates its states run, by name.
    pub templates: &'a BTreeMap<String, Statechart>,
}

/// A region resolved from its path: its definition, and the names of the
/// statecharts it lies inside, outermost first.
pub(crate) struct Found<'a> {
    /// The region as declared.
    pub region: &'a Region,
    /// The statecharts it lies inside, the one ticked first.
    pub inside: Vec<&'a str>,
}

impl<'a> Charts<'a> {
    /// The regions the state at `state_path` holds, its own and those of
    /// the template it runs, by name, each with the statecharts it lies
    /// inside. Refused where the template is not given, where it is one the
    /// state already lies inside, or where a region of its name is both the
    /// state's own and the template's.
    pub fn regions_of(
        &self,
        state_path: &str,
        state: &'a State,
        inside: &[&'a str],
    ) -> Result<BTreeMap<&'a str, Found<'a>>, Refusal> {
        let mut regions: BTreeMap<&'a str, Found<'a>> = state
            .regions
            .iter()
            .map(|(name, region)| {
                let found = Found {
                    region,
                    inside: inside.to_vec(),
                };
                (name.as_str(), found)
            })
            .collect();
        let Some(name) = &state.machine else {
            return Ok(regions);
        };
        let Some((name, template)) = self.templates.get_key_value(name) else {
            return Err(Refusal::new(
                state_path,
                format!("runs `{name}`, which is no template given"),
            ));
        };
        if inside.contains(&name.as_str()) {
            return Err(Refusal::new(
                state_path,
                format!("runs `{name}`, a statechart it already lies inside"),
            ));
        }
        let mut within = inside.to_vec();
        within.push(name.as_str());
        for (region_name, region) in &template.regions {
            if regions.contains_key(region_name.as_str()) {
                return Err(Refusal::new(
                    state_path,
                    format!(
                        "holds a region `{region_name}` as its own and in the template it runs"
                    ),
                ));
            }
            let found = Found {
                region,
                inside: within.clone(),
            };
            regions.insert(region_name.as_str(), found);
        }
        Ok(regions)
    }

    /// The region `path` names, walked as it is written, through each
    /// state it names, whether the region above holds that state now or
    /// held it before (97).
    pub fn region(&self, path: &str) -> Result<Found<'a>, Refusal> {
        let no_region = || Refusal::new(path, "names no region of the statechart");
        let segments: Vec<&str> = path.split('.').collect();
        if segments.len() % 2 == 0 {
            return Err(no_region());
        }
        let mut found = Found {
            region: self.chart.regions.get(segments[0]).ok_or_else(no_region)?,
            inside: vec![self.chart.name.as_str()],
        };
        let mut at = segments[0].to_string();
        for pair in segments[1..].chunks(2) {
            let (state_name, region_name) = (pair[0], pair[1]);
            let state = found.region.states.get(state_name).ok_or_else(no_region)?;
            let state_path = format!("{at}.{state_name}");
            let mut regions = self.regions_of(&state_path, state, &found.inside)?;
            found = regions.remove(region_name).ok_or_else(no_region)?;
            at = format!("{state_path}.{region_name}");
        }
        Ok(found)
    }

    /// The state the region at `path` holds in `held`, as declared.
    pub fn held(&self, path: &str, held: &str) -> Result<(Found<'a>, &'a State), Refusal> {
        let found = self.region(path)?;
        let state = found
            .region
            .states
            .get(held)
            .ok_or_else(|| Refusal::new(path, "holds a state its region does not hold"))?;
        Ok((found, state))
    }

    /// Every region path `state` holds resolved, and each holding a state
    /// of its region, or the first refused, in path order.
    pub fn check(&self, state: &ObjectState) -> Result<(), Refusal> {
        for (path, held) in &state.regions {
            self.held(path, held)?;
        }
        Ok(())
    }

    /// Whether `key`, an `enter`'s, names the region at `path`: by the
    /// region's name, by the state that holds it, or by the template that
    /// state runs, of which it is a region (96).
    fn names(&self, path: &str, key: &str, state: &ObjectState) -> bool {
        let segments: Vec<&str> = path.split('.').collect();
        let last = segments[segments.len() - 1];
        if last == key {
            return true;
        }
        if segments.len() < 3 {
            return false;
        }
        let owner = segments[segments.len() - 2];
        if owner == key {
            return true;
        }
        let above = segments[..segments.len() - 2].join(".");
        let runs = state
            .regions
            .get(&above)
            .and_then(|_| self.region(&above).ok())
            .and_then(|found| found.region.states.get(owner))
            .and_then(|owner| owner.machine.as_deref());
        runs == Some(key)
            && self
                .templates
                .get(key)
                .is_some_and(|template| template.regions.contains_key(last))
    }

    /// The regions an `enter` moves, each path with the state it moves to:
    /// for each key, the transition's first and then the entered state's,
    /// each region path `state` holds, live or not, that the key names and
    /// whose region holds the state named, the outermost alone, a path
    /// the transition's moves never moved again by the state's (96).
    fn commands(
        &self,
        state: &ObjectState,
        transition: &Transition,
        target: &State,
    ) -> Vec<(String, String)> {
        let mut commands: Vec<(String, String)> = Vec::new();
        for enter in [&transition.enter, &target.enter] {
            for (key, to) in enter {
                for path in state.regions.keys() {
                    let taken = commands
                        .iter()
                        .any(|(at, _)| at == path || path.starts_with(&format!("{at}.")));
                    if taken || !self.names(path, key, state) {
                        continue;
                    }
                    let holds_it = self
                        .region(path)
                        .is_ok_and(|found| found.region.states.contains_key(to));
                    if holds_it {
                        commands.push((path.clone(), to.clone()));
                    }
                }
            }
        }
        commands
    }

    /// The region at `path` started: it holds its initial state, entered
    /// at `at`, and what that state runs is started in turn (97).
    pub fn start_region(
        &self,
        next: &mut ObjectState,
        path: &str,
        found: &Found<'a>,
        at: OffsetDateTime,
    ) -> Result<(), Refusal> {
        let initial = &found.region.initial;
        let state = found
            .region
            .states
            .get(initial)
            .ok_or_else(|| Refusal::new(path, "holds a state its region does not hold"))?;
        next.regions.insert(path.to_string(), initial.clone());
        next.entered_at.insert(path.to_string(), at);
        self.start_within(next, &format!("{path}.{initial}"), state, &found.inside, at)
    }

    /// What the state at `state_path` runs and holds started: each of its
    /// regions at its initial state, entered at `at` (97).
    pub fn start_within(
        &self,
        next: &mut ObjectState,
        state_path: &str,
        state: &'a State,
        inside: &[&'a str],
        at: OffsetDateTime,
    ) -> Result<(), Refusal> {
        for (name, found) in self.regions_of(state_path, state, inside)? {
            self.start_region(next, &format!("{state_path}.{name}"), &found, at)?;
        }
        Ok(())
    }
}

/// An object's first state: each top region of `chart` at its initial
/// state, entered at `at`, with what each initial state runs started, and
/// no counter, response or instance set aside. It plans no effect: an
/// initial state's `entry` runs when a transition enters it (96, 97).
///
/// Refused, naming the path, where a state entered runs a template not
/// given, runs a statechart it already lies inside, or holds a region of a
/// name both its own and its template's.
pub fn start(
    chart: &Statechart,
    templates: &BTreeMap<String, Statechart>,
    at: OffsetDateTime,
) -> Result<ObjectState, Refusal> {
    let charts = Charts { chart, templates };
    let mut state = ObjectState::default();
    for (name, region) in &chart.regions {
        let found = Found {
            region,
            inside: vec![chart.name.as_str()],
        };
        charts.start_region(&mut state, name, &found, at)?;
    }
    Ok(state)
}

/// One tick: for each region path `state` holds, deepest first and in
/// path order at one depth, the first transition out of its state whose
/// guard holds over `snapshot` and `state` as given fires, at most one per
/// region; the effects each plans, those whose proof `snapshot` holds left
/// out; and the object's next state, each fired transition applied in the
/// order they fired (96, 97, 99).
///
/// `templates` holds what a state may run, by the name its `machine`
/// gives, and `proofs` the evidence that proves each effect, by its atom.
/// It reads nothing else and changes none of its inputs, so the same
/// inputs always give the same tick (98).
///
/// Refused, naming the path, where a region path `state` holds names no
/// region of `chart`, or holds a state its region does not; where a state
/// runs a template not given, or a statechart it already lies inside; or
/// where a state holds a region of a name both its own and its
/// template's.
pub fn tick(
    chart: &Statechart,
    templates: &BTreeMap<String, Statechart>,
    proofs: &BTreeMap<String, String>,
    state: &ObjectState,
    snapshot: &Snapshot,
) -> Result<Tick, Refusal> {
    let charts = Charts { chart, templates };
    charts.check(state)?;
    let mut paths: Vec<&String> = state.regions.keys().collect();
    paths.sort_by(|a, b| depth(b).cmp(&depth(a)).then(a.cmp(b)));
    let mut plans: Vec<Plan> = Vec::new();
    for path in paths {
        let under_a_fired = plans
            .iter()
            .any(|plan| path.starts_with(&format!("{}.", plan.fired.region)));
        if under_a_fired {
            continue;
        }
        let held = &state.regions[path];
        let (found, def) = charts.held(path, held)?;
        let scope = Scope {
            charts: &charts,
            snapshot,
            state,
            path,
            held,
            def,
        };
        for transition in &def.transitions {
            let guard = holds(&transition.when, &scope);
            if !guard.holds {
                continue;
            }
            let target = found.region.states.get(&transition.to).ok_or_else(|| {
                Refusal::new(
                    format!("{path}.{held}"),
                    format!(
                        "moves to `{}`, which its region does not hold",
                        transition.to
                    ),
                )
            })?;
            let response = guard
                .response
                .map(|(id, argument)| Applied { id, argument });
            let argument = response
                .as_ref()
                .map(|applied| applied.argument.as_str())
                .unwrap_or_default();
            let mut effects = Vec::new();
            let mut plan = |list: &[Effect], region: &str, phase: Phase| {
                for effect in list {
                    let proof = proofs.get(&effect.atom);
                    if proof.is_some_and(|proof| proven(snapshot, region, proof)) {
                        continue;
                    }
                    effects.push(Planned {
                        atom: effect.atom.clone(),
                        args: given(&effect.args, argument),
                        region: region.to_string(),
                        phase,
                    });
                }
            };
            let moves = transition.to != *held;
            if moves {
                plan(&def.exit, path, Phase::Exit);
            }
            plan(&transition.effects, path, Phase::Transition);
            if moves {
                plan(&target.entry, path, Phase::Entry);
            }
            let commands = charts.commands(state, transition, target);
            for (at, to) in &commands {
                if state.regions.get(at) == Some(to) {
                    continue;
                }
                let (_, entered) = charts.held(at, to)?;
                plan(&entered.entry, at, Phase::Enter);
            }
            let nothing_to_do = !moves
                && effects.is_empty()
                && response.is_none()
                && transition.bump.is_none()
                && transition.enter.is_empty();
            if !nothing_to_do {
                plans.push(Plan {
                    fired: Fired {
                        region: path.clone(),
                        from: held.clone(),
                        to: transition.to.clone(),
                        bump: transition.bump.clone(),
                        response,
                        effects,
                    },
                    commands,
                });
            }
            break;
        }
    }
    let mut next = state.clone();
    for plan in &plans {
        charts.apply(&mut next, plan, snapshot.at)?;
    }
    Ok(Tick {
        fired: plans.into_iter().map(|plan| plan.fired).collect(),
        next,
    })
}

/// An effect's arguments as written, an argument `$response` given the
/// argument bound (97).
fn given(args: &Table, argument: &str) -> Table {
    args.iter()
        .map(|(key, value)| {
            let value = match value {
                Value::String(text) if text == "$response" => Value::String(argument.to_string()),
                _ => value.clone(),
            };
            (key.clone(), value)
        })
        .collect()
}

/// Every region path under `path` cleared from `next`.
fn clear_under(next: &mut ObjectState, path: &str) {
    let prefix = format!("{path}.");
    next.regions.retain(|at, _| !at.starts_with(&prefix));
    next.entered_at.retain(|at, _| !at.starts_with(&prefix));
}

/// The instance the state `to` of the region at `region` holds, set aside
/// under its next attempt, `<region>.<to>#<n>` counting from 1, and cleared
/// for a fresh one (97). Nothing is set aside where it holds none.
fn set_aside(next: &mut ObjectState, region: &str, to: &str) {
    let path = format!("{region}.{to}");
    let prefix = format!("{path}.");
    let instance: BTreeMap<String, String> = next
        .regions
        .iter()
        .filter_map(|(at, held)| {
            at.strip_prefix(&prefix)
                .map(|relative| (relative.to_string(), held.clone()))
        })
        .collect();
    if instance.is_empty() {
        return;
    }
    let attempt = 1 + next
        .set_aside
        .keys()
        .filter(|key| key.starts_with(&format!("{path}#")))
        .count();
    next.set_aside.insert(format!("{path}#{attempt}"), instance);
    clear_under(next, &path);
}

impl Charts<'_> {
    /// One fired transition applied to `next`: each region its `enter`
    /// moves and `next` still holds, where it is not in its target already,
    /// cleared under and moved, what it enters started; the instance the
    /// target holds set aside; the region moved, entered at `at` where it
    /// moves to another state; its counter bumped and its response applied
    /// once; and what the target runs and holds started (96, 97).
    fn apply(
        &self,
        next: &mut ObjectState,
        plan: &Plan,
        at: OffsetDateTime,
    ) -> Result<(), Refusal> {
        for (path, to) in &plan.commands {
            match next.regions.get(path) {
                Some(held) if held != to => {}
                _ => continue,
            }
            clear_under(next, path);
            next.regions.insert(path.clone(), to.clone());
            next.entered_at.insert(path.clone(), at);
            let (found, entered) = self.held(path, to)?;
            self.start_within(next, &format!("{path}.{to}"), entered, &found.inside, at)?;
        }
        let fired = &plan.fired;
        set_aside(next, &fired.region, &fired.to);
        next.regions.insert(fired.region.clone(), fired.to.clone());
        if fired.to != fired.from {
            next.entered_at.insert(fired.region.clone(), at);
        }
        if let Some(counter) = &fired.bump {
            *next.counters.entry(counter.clone()).or_insert(0) += 1;
        }
        if let Some(applied) = &fired.response {
            if !next.applied.contains(&applied.id) {
                next.applied.push(applied.id.clone());
            }
        }
        let (found, entered) = self.held(&fired.region, &fired.to)?;
        let path = format!("{}.{}", fired.region, fired.to);
        self.start_within(next, &path, entered, &found.inside, at)
    }
}
