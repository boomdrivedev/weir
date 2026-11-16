//! A fact's names, values and sorts (117, 118; C.9).
//!
//! A fact is a name with one value or several, each a text, a number, true
//! or false, or a date (117). In flight it is a key of an object, each value
//! JSON's string, number or boolean, a date being a string that reads as an
//! RFC 3339 full-date, or a list of them (C.1, C.9). Its name gives its
//! sort: stated by a source, joined by a host, or read by a reader (117 to
//! 120; record 0010).

use std::collections::BTreeMap;
use std::fmt;

use jiff::civil::Date;
use serde::{Serialize, Serializer};
use serde_json::Number;

/// One value of a fact (117): a text, a number, true or false, or a date.
#[derive(Clone, Debug)]
pub enum Value {
    /// A text, as written.
    Text(String),
    /// A number, one value however it is spelled: `900` and `900.0` are
    /// equal (record 0010).
    Number(Number),
    /// True or false.
    Bool(bool),
    /// A date, spelled as an RFC 3339 full-date in a string (C.9).
    Date(Date),
}

impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Text(a), Value::Text(b)) => a == b,
            (Value::Number(a), Value::Number(b)) => a.as_f64() == b.as_f64(),
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Date(a), Value::Date(b)) => a == b,
            _ => false,
        }
    }
}

// JSON spells no number that is not a number, so equality is total.
impl Eq for Value {}

impl Value {
    /// The value a JSON scalar spells, or none for `null`, an object or a
    /// list (117; C.9). A string that reads as an RFC 3339 full-date is a
    /// date.
    pub fn from_json(json: &serde_json::Value) -> Option<Value> {
        match json {
            serde_json::Value::String(text) => Some(Value::from_text(text)),
            serde_json::Value::Number(number) => Some(Value::Number(number.clone())),
            serde_json::Value::Bool(flag) => Some(Value::Bool(*flag)),
            _ => None,
        }
    }

    /// The value a string spells: a date where it is an RFC 3339
    /// full-date, `YYYY-MM-DD`, and a text otherwise (C.9).
    pub fn from_text(text: &str) -> Value {
        match full_date(text) {
            Some(date) => Value::Date(date),
            None => Value::Text(text.to_string()),
        }
    }

    /// The value as JSON spells it, a date as its full-date in a string.
    pub fn to_json(&self) -> serde_json::Value {
        match self {
            Value::Text(text) => serde_json::Value::String(text.clone()),
            Value::Number(number) => serde_json::Value::Number(number.clone()),
            Value::Bool(flag) => serde_json::Value::Bool(*flag),
            Value::Date(date) => serde_json::Value::String(date.strftime("%Y-%m-%d").to_string()),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_json())
    }
}

/// An RFC 3339 full-date, four digits of year, two of month and two of
/// day, joined by `-`, and a day the calendar holds.
fn full_date(text: &str) -> Option<Date> {
    let bytes = text.as_bytes();
    let shaped = bytes.len() == 10
        && bytes.iter().enumerate().all(|(at, byte)| match at {
            4 | 7 => *byte == b'-',
            _ => byte.is_ascii_digit(),
        });
    if !shaped {
        return None;
    }
    let number = |range: std::ops::Range<usize>| text[range].parse::<i16>().ok();
    let (year, month, day) = (number(0..4)?, number(5..7)?, number(8..10)?);
    Date::new(year, i8::try_from(month).ok()?, i8::try_from(day).ok()?).ok()
}

/// Which of three a fact is, by its name (117 to 120; C.9).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Sort {
    /// Stated by a source in its capture's offer: lowercase letters,
    /// digits, `_` and `-`, and the offer's own `source`, `title` and
    /// `event_at` (117, 118).
    Stated,
    /// Joined by a host from its people list: `attendee.` or `speaker.`,
    /// then `name`, `role`, `teams`, `org` or `client` (119).
    Joined,
    /// Read by a reader: `says`, `speaker`, `about`, `owner` or `due`
    /// (120).
    Read,
}

/// The read facts' names (120; C.9).
const READ: [&str; 5] = ["says", "speaker", "about", "owner", "due"];

/// Whom a joined fact is of (119; C.9).
const WHOM: [&str; 2] = ["attendee", "speaker"];

/// What a joined fact gives of whom it is of (119; C.9).
const JOINED: [&str; 5] = ["name", "role", "teams", "org", "client"];

/// The stated facts an offer gives as fields of its own, never in its
/// `facts` (117; record 0010).
const OWN_FIELDS: [&str; 3] = ["source", "title", "event_at"];

/// The sort C.9 gives the name `name`, or none where it spells none.
pub fn sort(name: &str) -> Option<Sort> {
    if READ.contains(&name) {
        return Some(Sort::Read);
    }
    if let Some((whom, fact)) = name.split_once('.') {
        return (WHOM.contains(&whom) && JOINED.contains(&fact)).then_some(Sort::Joined);
    }
    let spelled = !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '-');
    spelled.then_some(Sort::Stated)
}

/// Why facts were refused, naming the field: `facts.<name>`, or
/// `facts.<name>[<n>]` for one value of a list, or `fact` at rest (C.9).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FactError {
    /// The field refused.
    pub field: String,
    /// Why.
    pub reason: String,
}

impl FactError {
    fn new(field: impl Into<String>, reason: impl Into<String>) -> FactError {
        FactError {
            field: field.into(),
            reason: reason.into(),
        }
    }
}

impl fmt::Display for FactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "`{}` {}", self.field, self.reason)
    }
}

impl std::error::Error for FactError {}

/// Facts: each name with its values, in the names' order, each fact's
/// values in the order given, and never a name with none (117).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Facts {
    facts: BTreeMap<String, Vec<Value>>,
}

impl Facts {
    /// No facts.
    pub fn new() -> Facts {
        Facts::default()
    }

    /// Whether there are none.
    pub fn is_empty(&self) -> bool {
        self.facts.is_empty()
    }

    /// The values of the fact `name`, in the order given, or none where no
    /// such fact is held.
    pub fn get(&self, name: &str) -> Option<&[Value]> {
        self.facts.get(name).map(Vec::as_slice)
    }

    /// Each fact's name with its values, in the names' order.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &[Value])> {
        self.facts
            .iter()
            .map(|(name, values)| (name.as_str(), values.as_slice()))
    }

    /// Adds `value` after the values of the fact `name`.
    pub(crate) fn push(&mut self, name: &str, value: Value) {
        self.facts.entry(name.to_string()).or_default().push(value);
    }

    /// Adds every fact of `other`, its values after those held.
    pub(crate) fn extend(&mut self, other: &Facts) {
        for (name, values) in other.iter() {
            for value in values {
                self.push(name, value.clone());
            }
        }
    }

    /// The stated facts of an offer's `facts` object (2, 117; C.1, C.9):
    /// names of the stated sort alone, but the offer's own `source`,
    /// `title` and `event_at`, each with values of 117's forms.
    pub fn stated(json: &serde_json::Value) -> Result<Facts, FactError> {
        Facts::of_sort(json, |name| match sort(name) {
            _ if OWN_FIELDS.contains(&name) => Err(format!(
                "is the offer's own `{name}`, which states that fact (117)"
            )),
            Some(Sort::Stated) => Ok(()),
            Some(Sort::Read) => Err("is a read fact's name, which a reader gives (120)".into()),
            Some(Sort::Joined) => Err("is a joined fact's name, which a host joins (119)".into()),
            None => Err(NOT_SPELLED.into()),
        })
    }

    /// The read facts of a reader's object (120; C.9): names of the read
    /// sort alone, each with values of 117's forms. Whether a value is on
    /// its list is the reading's to check.
    pub fn read(json: &serde_json::Value) -> Result<Facts, FactError> {
        Facts::of_sort(json, |name| match sort(name) {
            Some(Sort::Read) => Ok(()),
            Some(_) => Err(format!(
                "is not a read fact's name, one of {}",
                READ.join(", ")
            )),
            None => Err(NOT_SPELLED.into()),
        })
    }

    fn of_sort(
        json: &serde_json::Value,
        named: impl Fn(&str) -> Result<(), String>,
    ) -> Result<Facts, FactError> {
        let serde_json::Value::Object(object) = json else {
            return Err(FactError::new("facts", "is not an object"));
        };
        let mut facts = Facts::new();
        for (name, given) in object {
            let field = format!("facts.{name}");
            named(name).map_err(|reason| FactError::new(&field, reason))?;
            let values = match given {
                serde_json::Value::Array(list) if list.is_empty() => {
                    return Err(FactError::new(field, "is a list of no values"));
                }
                serde_json::Value::Array(list) => list
                    .iter()
                    .enumerate()
                    .map(|(n, item)| {
                        Value::from_json(item).ok_or_else(|| {
                            FactError::new(format!("{field}[{n}]"), not_a_value(item))
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?,
                one => vec![Value::from_json(one)
                    .ok_or_else(|| FactError::new(&field, not_a_value(one)))?],
            };
            facts.facts.insert(name.clone(), values);
        }
        Ok(facts)
    }

    /// The facts as an object, a fact of one value as the value and one of
    /// several as a list (C.1, C.9).
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::Value::Object(
            self.facts
                .iter()
                .map(|(name, values)| {
                    let json = match values.as_slice() {
                        [one] => one.to_json(),
                        several => {
                            serde_json::Value::Array(several.iter().map(Value::to_json).collect())
                        }
                    };
                    (name.clone(), json)
                })
                .collect(),
        )
    }
}

impl Serialize for Facts {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.to_json().serialize(serializer)
    }
}

const NOT_SPELLED: &str = "is not a fact's name C.9 spells: lowercase letters, digits, `_` and `-`";

/// Why a JSON value is no value of 117's forms.
fn not_a_value(json: &serde_json::Value) -> &'static str {
    match json {
        serde_json::Value::Null => "is null, not a text, a number, true or false, or a date",
        serde_json::Value::Array(_) => "is a list in a list",
        _ => "is an object, not a text, a number, true or false, or a date",
    }
}
