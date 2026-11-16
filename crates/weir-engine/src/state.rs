//! One object's state: the state each region path holds, when it entered
//! it, its counters, the responses it applied and the instances its
//! states set aside (96, 97).
//!
//! A region path is `<region>` for a top region and `<path>.<state>.<region>`
//! for a region under a state, one of the state's own regions or a region
//! of the template it runs.

use std::collections::BTreeMap;

use time::OffsetDateTime;

/// One object's state, as its caller holds it between ticks (96, 97).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ObjectState {
    /// Each region path, with the state it holds.
    pub regions: BTreeMap<String, String>,
    /// Each region path, with when it entered the state it holds.
    pub entered_at: BTreeMap<String, OffsetDateTime>,
    /// Each counter a transition bumped, with how many times.
    pub counters: BTreeMap<String, u64>,
    /// The ids of the responses a guard took, in the order taken, each
    /// once (97).
    pub applied: Vec<String>,
    /// Each instance a state's entry set aside, under
    /// `<region path>.<state>#<attempt>`, the attempt counting from 1, with
    /// its region paths taken relative to that state and their states (97).
    pub set_aside: BTreeMap<String, BTreeMap<String, String>>,
}

impl ObjectState {
    /// Whether the region at `path` is live: every state its path names is
    /// the state its region holds now. A region under a state its region
    /// has left is not live, and stays at its path (97).
    pub fn is_live(&self, path: &str) -> bool {
        states_named(path)
            .iter()
            .all(|(region, state)| self.regions.get(region) == Some(state))
    }
}

/// The region path above `path`, or `None` for a top region:
/// `a.s.b` gives `a`.
pub(crate) fn parent_path(path: &str) -> Option<&str> {
    let (above, _region) = path.rsplit_once('.')?;
    let (above, _state) = above.rsplit_once('.')?;
    Some(above)
}

/// The states `path` names, each with the region path that holds it, outermost
/// first: `a.s.b.t.c` gives `(a, s)` and `(a.s.b, t)`.
pub(crate) fn states_named(path: &str) -> Vec<(String, String)> {
    let segments: Vec<&str> = path.split('.').collect();
    (1..segments.len())
        .step_by(2)
        .map(|at| (segments[..at].join("."), segments[at].to_string()))
        .collect()
}

/// How deep `path` lies: 0 for a top region, one more for each state above.
pub(crate) fn depth(path: &str) -> usize {
    path.matches('.').count() / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_paths_parent_and_the_states_it_names() {
        assert_eq!(parent_path("a"), None);
        assert_eq!(parent_path("a.s.b"), Some("a"));
        assert_eq!(parent_path("a.s.b.t.c"), Some("a.s.b"));
        assert_eq!(states_named("a"), Vec::<(String, String)>::new());
        assert_eq!(
            states_named("a.s.b.t.c"),
            vec![
                ("a".to_string(), "s".to_string()),
                ("a.s.b".to_string(), "t".to_string())
            ]
        );
        assert_eq!(depth("a"), 0);
        assert_eq!(depth("a.s.b.t.c"), 2);
    }

    #[test]
    fn a_region_is_live_while_its_states_are_held() {
        let mut state = ObjectState::default();
        state.regions.insert("a".into(), "s".into());
        state.regions.insert("a.s.b".into(), "t".into());
        state.regions.insert("a.u.b".into(), "t".into());
        assert!(state.is_live("a"));
        assert!(state.is_live("a.s.b"));
        assert!(!state.is_live("a.u.b"));
    }
}
