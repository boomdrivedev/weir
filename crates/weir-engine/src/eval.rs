//! Guards over the snapshot and the object's state as given, evidence read
//! at a region, values compared, and an answer matched against a
//! response's pattern (94, 96, 97).

use std::cmp::Ordering;

use time::format_description::well_known::Rfc3339;
use time::{Date, Month, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset};
use toml::value::{Datetime, Offset};
use toml::Value;

use crate::state::parent_path;
use crate::tick::Charts;
use crate::{ChildrenTest, Guard, ObjectState, Related, Snapshot, State, Test};

/// Where a guard is read: the snapshot and the object's state as the tick
/// was given them, the region path of the transition tried and the state
/// it leaves.
pub(crate) struct Scope<'s, 'a> {
    /// The statechart and its templates, for a `final` guard.
    pub charts: &'s Charts<'a>,
    /// The tick's one snapshot.
    pub snapshot: &'s Snapshot,
    /// The object's state as given.
    pub state: &'s ObjectState,
    /// The region path of the transition tried.
    pub path: &'s str,
    /// The state that region holds, which the transition leaves.
    pub held: &'s str,
    /// That state as declared.
    pub def: &'s State,
}

/// Whether a guard holds, and the response it took: its id and the
/// argument its answer bound.
#[derive(Debug, Default)]
pub(crate) struct Held {
    /// Whether it holds.
    pub holds: bool,
    /// The response taken, its id and the argument bound.
    pub response: Option<(String, String)>,
}

impl Held {
    fn when(holds: bool) -> Held {
        Held {
            holds,
            response: None,
        }
    }
}

/// Whether `guard` holds where `scope` reads it (94, 96).
pub(crate) fn holds(guard: &Guard, scope: &Scope) -> Held {
    match guard {
        Guard::All(guards) => {
            let mut response = None;
            for guard in guards {
                let held = holds(guard, scope);
                if !held.holds {
                    return Held::when(false);
                }
                if held.response.is_some() {
                    response = held.response;
                }
            }
            Held {
                holds: true,
                response,
            }
        }
        Guard::Any(guards) => guards
            .iter()
            .map(|guard| holds(guard, scope))
            .find(|held| held.holds)
            .unwrap_or_default(),
        Guard::Not(guard) => Held::when(!holds(guard, scope).holds),
        Guard::Always(always) => Held::when(*always),
        Guard::Evidence { ev, test } => Held::when(tests(ev, test, scope)),
        Guard::Response(pattern) => response(pattern, scope),
        Guard::Final(name) => Held::when(is_final(name, scope)),
        Guard::Region { name, within } => Held::when(region(name, within, scope)),
        Guard::Parent { within } => Held::when(
            scope
                .snapshot
                .owner
                .as_ref()
                .is_some_and(|owner| in_one_of(owner, within)),
        ),
        Guard::Children { kind, test } => Held::when(children(kind, test, scope)),
    }
}

/// The evidence `name` read for the region at `path`: the value the
/// snapshot gives at that path, else at the nearest path above it, else
/// for the object (98).
pub(crate) fn evidence<'s>(snapshot: &'s Snapshot, path: &str, name: &str) -> Option<&'s Value> {
    let mut at = Some(path);
    while let Some(path) = at {
        if let Some(value) = snapshot
            .regions
            .get(path)
            .and_then(|values| values.get(name))
        {
            return Some(value);
        }
        at = parent_path(path);
    }
    snapshot.evidence.get(name)
}

/// Whether the snapshot holds the proof `name` for the region at `path`:
/// read there with any value but `false` (96).
pub(crate) fn proven(snapshot: &Snapshot, path: &str, name: &str) -> bool {
    evidence(snapshot, path, name).is_some_and(|value| *value != Value::Boolean(false))
}

/// An evidence put to its one test.
fn tests(name: &str, test: &Test, scope: &Scope) -> bool {
    let read = |name: &str| evidence(scope.snapshot, scope.path, name);
    let value = read(name);
    if let Test::Exists(exists) = test {
        return value.is_some() == *exists;
    }
    let Some(value) = value else {
        return false;
    };
    let ordered = |other: &Value, wanted: &[Ordering]| {
        order(value, other).is_some_and(|ordering| wanted.contains(&ordering))
    };
    match test {
        Test::Exists(_) => unreachable!("read above"),
        Test::Is(other) => equal(value, other),
        Test::In(others) => others.iter().any(|other| equal(value, other)),
        Test::Gt(other) => ordered(other, &[Ordering::Greater]),
        Test::Gte(other) => ordered(other, &[Ordering::Greater, Ordering::Equal]),
        Test::Lt(other) => ordered(other, &[Ordering::Less]),
        Test::Lte(other) => ordered(other, &[Ordering::Less, Ordering::Equal]),
        Test::Older(duration) => {
            let Some(then) = time_of(value) else {
                return false;
            };
            time::Duration::try_from(*duration)
                .is_ok_and(|duration| scope.snapshot.at - then > duration)
        }
        Test::EqEv(other) => read(other).is_some_and(|other| equal(value, other)),
        Test::NeEv(other) => read(other).is_none_or(|other| !equal(value, other)),
        Test::GtEv(other) => read(other).is_some_and(|other| ordered(other, &[Ordering::Greater])),
        Test::GteEv(other) => {
            read(other).is_some_and(|other| ordered(other, &[Ordering::Greater, Ordering::Equal]))
        }
        Test::LtEv(other) => read(other).is_some_and(|other| ordered(other, &[Ordering::Less])),
    }
}

/// Whether two values are equal: by `toml`'s equality, but that an integer
/// and a float are equal where their numbers are, and two times where
/// their instants are.
pub(crate) fn equal(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Integer(x), Value::Float(y)) | (Value::Float(y), Value::Integer(x)) => {
            *x as f64 == *y
        }
        _ => match (time_of(a), time_of(b)) {
            (Some(x), Some(y)) => x == y,
            _ => a == b,
        },
    }
}

/// How two values order: numbers by value, times by instant and strings by
/// their bytes; any other pair has none.
pub(crate) fn order(a: &Value, b: &Value) -> Option<Ordering> {
    match (a, b) {
        (Value::Integer(x), Value::Integer(y)) => Some(x.cmp(y)),
        (Value::Integer(x), Value::Float(y)) => (*x as f64).partial_cmp(y),
        (Value::Float(x), Value::Integer(y)) => x.partial_cmp(&(*y as f64)),
        (Value::Float(x), Value::Float(y)) => x.partial_cmp(y),
        _ => match (time_of(a), time_of(b)) {
            (Some(x), Some(y)) => Some(x.cmp(&y)),
            _ => match (a, b) {
                (Value::String(x), Value::String(y)) => Some(x.as_bytes().cmp(y.as_bytes())),
                _ => None,
            },
        },
    }
}

/// A value read as a time: a datetime with its offset, or a string in RFC
/// 3339. A datetime with no offset is no time.
pub(crate) fn time_of(value: &Value) -> Option<OffsetDateTime> {
    match value {
        Value::Datetime(datetime) => from_datetime(datetime),
        Value::String(text) => OffsetDateTime::parse(text, &Rfc3339).ok(),
        _ => None,
    }
}

/// A TOML datetime with its date, time and offset as an instant.
fn from_datetime(datetime: &Datetime) -> Option<OffsetDateTime> {
    let (date, time, offset) = (datetime.date?, datetime.time?, datetime.offset?);
    let date = Date::from_calendar_date(
        i32::from(date.year),
        Month::try_from(date.month).ok()?,
        date.day,
    )
    .ok()?;
    let time = Time::from_hms_nano(
        time.hour,
        time.minute,
        time.second.unwrap_or(0),
        time.nanosecond.unwrap_or(0),
    )
    .ok()?;
    let offset = match offset {
        Offset::Z => UtcOffset::UTC,
        Offset::Custom { minutes } => {
            UtcOffset::from_whole_seconds(i32::from(minutes) * 60).ok()?
        }
    };
    Some(PrimitiveDateTime::new(date, time).assume_offset(offset))
}

/// The first response of the snapshot, in its order, that the object has
/// not applied and whose answer `pattern` matches (97).
fn response(pattern: &str, scope: &Scope) -> Held {
    scope
        .snapshot
        .responses
        .iter()
        .filter(|response| !scope.state.applied.contains(&response.id))
        .find_map(|response| {
            match_answer(pattern, &response.answer).map(|argument| Held {
                holds: true,
                response: Some((response.id.clone(), argument)),
            })
        })
        .unwrap_or_default()
}

/// `{ final = "X" }` or `{ final = "R.X" }`: a region the guard sees is in
/// a state named `X` that is final in its region (96, 97). It sees the
/// regions directly under a state of its own region, and the regions of a
/// statechart run by a state of one of the left state's own regions; where
/// the left state runs a template or holds regions of its own, only those
/// under it, and where it runs nothing, what an earlier state of its
/// region ran, still standing at its path. `R` names the region, or the
/// left state's own region it lies under.
fn is_final(name: &str, scope: &Scope) -> bool {
    let (want_region, want_state) = match name.split_once('.') {
        Some((region, state)) => (Some(region), state),
        None => (None, name),
    };
    let prefix = format!("{}.", scope.path);
    let runs_its_own = scope.def.machine.is_some() || !scope.def.regions.is_empty();
    scope.state.regions.iter().any(|(path, held)| {
        if held != want_state {
            return false;
        }
        let Some(rest) = path.strip_prefix(&prefix) else {
            return false;
        };
        let parts: Vec<&str> = rest.split('.').collect();
        let (owner, under) = (parts[0], &parts[1..]);
        if runs_its_own && owner != scope.held {
            return false;
        }
        let visible = match under.len() {
            1 => true,
            3 => owner == scope.held && scope.def.regions.contains_key(under[0]),
            _ => false,
        };
        let named = want_region
            .is_none_or(|region| under.first() == Some(&region) || under.last() == Some(&region));
        visible
            && named
            && scope
                .charts
                .region(path)
                .ok()
                .and_then(|found| found.region.states.get(held))
                .is_some_and(|state| state.is_final)
    })
}

/// `{ region = { name, in } }`: by a dotted name, some region path equal
/// to it or ending in it holds a state listed; by a bare name, a region of
/// that name under the left state, or beside the guard's own region, does
/// (94).
fn region(name: &str, within: &[String], scope: &Scope) -> bool {
    let listed = |held: &String| within.contains(held);
    if name.contains('.') {
        let suffix = format!(".{name}");
        return scope
            .state
            .regions
            .iter()
            .any(|(path, held)| (path == name || path.ends_with(&suffix)) && listed(held));
    }
    let below = format!("{}.{}.", scope.path, scope.held);
    let beside = without_last(scope.path);
    scope.state.regions.iter().any(|(path, held)| {
        let last = path.rsplit('.').next().unwrap_or(path);
        last == name && (path.starts_with(&below) || without_last(path) == beside) && listed(held)
    })
}

/// A path with its last name taken off, or `None` where it has one name.
fn without_last(path: &str) -> Option<&str> {
    path.rsplit_once('.').map(|(above, _)| above)
}

/// Whether a related object holds one of the states listed.
fn in_one_of(related: &Related, within: &[String]) -> bool {
    related.states.iter().any(|state| within.contains(state))
}

/// `{ children = { kind, <test> } }`, over the children of `kind` the
/// snapshot gives (94).
fn children(kind: &str, test: &ChildrenTest, scope: &Scope) -> bool {
    let kids: Vec<&Related> = scope
        .snapshot
        .children
        .iter()
        .filter(|child| child.kind == kind)
        .collect();
    match test {
        ChildrenTest::All(within) => {
            !kids.is_empty() && kids.iter().all(|kid| in_one_of(kid, within))
        }
        ChildrenTest::Any(within) => kids.iter().any(|kid| in_one_of(kid, within)),
        ChildrenTest::None(within) => !kids.iter().any(|kid| in_one_of(kid, within)),
        ChildrenTest::CountGte(count) => kids.len() as u64 >= *count,
    }
}

/// Match an answer against a pattern: a bare word, `word <arg>`, or `word: <text>`.
/// Returns the bound argument (empty for a bare word).
pub(crate) fn match_answer(pattern: &str, answer: &str) -> Option<String> {
    let p = pattern.trim();
    let a = answer.trim();
    if let Some(head) = p
        .strip_suffix('>')
        .and_then(|s| s.rsplit_once('<'))
        .map(|(h, _)| h.trim_end())
    {
        // `route <name>` → head "route"; `redo: <notes>` → head "redo:"
        let head_word = head.trim_end_matches(':').trim();
        let colon = head.ends_with(':');
        // The head may be several words (`new route <name>`) or none (`<text>`).
        let rest = a.strip_prefix(head_word)?;
        if colon {
            return Some(rest.trim_start().strip_prefix(':')?.trim().to_string());
        }
        if !head_word.is_empty() && !rest.is_empty() && !rest.starts_with(char::is_whitespace) {
            return None;
        }
        return Some(rest.trim().to_string());
    }
    // `<name>: drop` — the argument leads and a fixed tail follows. A machine
    // whose answer names one of several things and then says what to do with it
    // is written this way, and a matcher that only read a trailing placeholder
    // could never match one: the answer was unanswerable however it was given.
    if let Some(tail) = p
        .strip_prefix('<')
        .and_then(|rest| rest.split_once('>'))
        .map(|(_, tail)| tail)
    {
        let tail = tail.trim();
        let bound = match tail.is_empty() {
            true => a,
            false => a.strip_suffix(tail)?.trim().trim_end_matches(':').trim(),
        };
        return match bound.is_empty() {
            true => None,
            false => Some(bound.to_string()),
        };
    }
    if a == p {
        return Some(String::new());
    }
    None
}

#[cfg(test)]
mod tests {
    use super::match_answer;

    #[test]
    fn bare_word_matches_exactly() {
        assert_eq!(match_answer("yes", "yes"), Some(String::new()));
        assert_eq!(
            match_answer("yes", "  yes "),
            Some(String::new()),
            "surrounding whitespace is ignored"
        );
        assert_eq!(match_answer("drop", "drop"), Some(String::new()));
        assert_eq!(match_answer("yes", "no"), None);
        assert_eq!(
            match_answer("yes", "yes please"),
            None,
            "a bare word takes no argument"
        );
        assert_eq!(match_answer("yes", "Yes"), None, "case matters");
        assert_eq!(match_answer("yes", ""), None);
    }

    #[test]
    fn word_with_argument_binds_the_rest() {
        assert_eq!(
            match_answer("route <name>", "route plan-rows"),
            Some("plan-rows".into())
        );
        assert_eq!(
            match_answer("route <name>", "route   route/one/plan-rows  "),
            Some("route/one/plan-rows".into())
        );
        assert_eq!(
            match_answer("route <name>", "route"),
            Some(String::new()),
            "the argument may be empty"
        );
        assert_eq!(match_answer("route <name>", "routes x"), None);
        assert_eq!(match_answer("route <name>", "rename x"), None);
        // A multi-word head matches whole, never by one of its words.
        assert_eq!(
            match_answer("new route <name>", "new route thing"),
            Some("thing".into())
        );
        assert_eq!(match_answer("new route <name>", "route thing"), None);
        assert_eq!(match_answer("new route <name>", "new thing"), None);
        assert_eq!(
            match_answer("route <name>", "new route thing"),
            None,
            "`route` is not the first word"
        );
        assert_eq!(
            match_answer("pick <letters>", "pick a b c"),
            Some("a b c".into())
        );
        // `<text>` alone has an empty head: the whole answer is the argument.
        assert_eq!(
            match_answer("<text>", "anything at all"),
            Some("anything at all".into())
        );
        assert_eq!(match_answer("<text>", ""), Some(String::new()));
    }

    #[test]
    fn word_with_colon_binds_the_text_after_it() {
        assert_eq!(
            match_answer("redo: <notes>", "redo: split the item"),
            Some("split the item".into())
        );
        assert_eq!(
            match_answer("redo: <notes>", "redo:split"),
            Some("split".into())
        );
        assert_eq!(
            match_answer("redo: <notes>", "redo: a: b"),
            Some("a: b".into()),
            "only the first colon splits"
        );
        assert_eq!(match_answer("redo: <notes>", "redo:"), Some(String::new()));
        assert_eq!(
            match_answer("redo: <notes>", "redo split"),
            None,
            "the colon form needs its colon"
        );
        assert_eq!(match_answer("redo: <notes>", "undo: x"), None);
    }

    #[test]
    fn an_argument_before_a_tail() {
        assert_eq!(
            match_answer("<name>: drop", "north: drop"),
            Some("north".into())
        );
        assert_eq!(
            match_answer("<name>: drop", "  north wing : drop "),
            Some("north wing".into()),
            "the argument may be several words, and whitespace is ignored"
        );
        assert_eq!(
            match_answer("<name>: drop", "north drop"),
            None,
            "the tail needs its colon"
        );
        assert_eq!(
            match_answer("<name>: drop", "north: Drop"),
            None,
            "case matters"
        );
        assert_eq!(
            match_answer("<name>: drop", "north: drop now"),
            None,
            "the tail ends the answer"
        );
        assert_eq!(
            match_answer("<name>: drop", ": drop"),
            None,
            "the argument may not be empty"
        );
        assert_eq!(match_answer("<name>: drop", "drop"), None);
    }
}
