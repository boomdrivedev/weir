//! The tick and an object's first state: a region path resolved through a
//! statechart, its states' own regions and the templates they run, and
//! what a state runs started when it is entered (96, 97, 99).

use std::collections::BTreeMap;

use time::OffsetDateTime;

use crate::{ObjectState, Refusal, Region, State, Statechart};

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
