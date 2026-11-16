//! The people list and the join of attendees and speakers from it (119,
//! 121; C.9).
//!
//! The list is read from `[about]`, `[[person]]` and `[[org]]` of a host's
//! settings document, whose other tables are its host's and are left alone.
//! A field the profile does not name is refused naming it, as a policy's is
//! (record 0010).

use std::fmt;

use toml::{Table, Value as Toml};

/// The people list (119): what a reader's `about` picks from, the persons
/// and the orgs.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct People {
    /// What a reader's `about` picks from (120).
    pub about: About,
    /// The persons, in the order given.
    pub persons: Vec<Person>,
    /// The orgs, in the order given.
    pub orgs: Vec<Org>,
}

/// What a reader's `about` picks from (120): `[about]`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct About {
    /// The teams, empty where none are given.
    pub teams: Vec<String>,
    /// The repositories, empty where none are given.
    pub repositories: Vec<String>,
}

/// One `[[person]]` (119).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Person {
    /// Their name, which a joined `name` gives.
    pub name: String,
    /// Their addresses, which an attendee joins by.
    pub addresses: Vec<String>,
    /// The names a transcript may give them, which a speaker joins by.
    pub names: Vec<String>,
    /// Their role, where one is given.
    pub role: Option<String>,
    /// Their teams.
    pub teams: Vec<String>,
}

/// One `[[org]]` (119).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Org {
    /// Its name, which a joined `org` gives.
    pub name: String,
    /// The domains of its addresses.
    pub domains: Vec<String>,
    /// Whether it is a client, false where it does not say.
    pub client: bool,
}

/// Why a people list or a policy was refused, naming the field by its path
/// counting from 0, as `person[0].name` or `rules[1].when.all[0].fact`
/// (122; C.9; record 0010), or naming none where the document is not TOML.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SettingsError {
    /// The field refused, none where the document is not TOML.
    pub field: Option<String>,
    /// Why.
    pub reason: String,
}

impl SettingsError {
    pub(crate) fn new(field: impl Into<String>, reason: impl Into<String>) -> SettingsError {
        SettingsError {
            field: Some(field.into()),
            reason: reason.into(),
        }
    }
}

impl fmt::Display for SettingsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.field {
            Some(field) => write!(f, "`{field}` {}", self.reason),
            None => f.write_str(&self.reason),
        }
    }
}

impl std::error::Error for SettingsError {}

impl People {
    /// Reads the people list from a settings document (119; C.9): its
    /// `[about]`, `[[person]]` and `[[org]]`, each field checked and every
    /// other taking its default (record 0010).
    pub fn read(settings: &str) -> Result<People, SettingsError> {
        let document = document(settings)?;
        let about = match document.get("about") {
            None => About::default(),
            Some(Toml::Table(about)) => {
                unnamed(about, &["teams", "repositories"], "about")?;
                About {
                    teams: strings(about, "teams", "about")?,
                    repositories: strings(about, "repositories", "about")?,
                }
            }
            Some(_) => return Err(SettingsError::new("about", "is not a table")),
        };
        let persons = tables(&document, "person")?
            .into_iter()
            .map(|(at, person)| {
                unnamed(
                    person,
                    &["name", "addresses", "names", "role", "teams"],
                    &at,
                )?;
                Ok(Person {
                    name: required_text(person, "name", &at)?,
                    addresses: strings(person, "addresses", &at)?,
                    names: strings(person, "names", &at)?,
                    role: text(person, "role", &at)?,
                    teams: strings(person, "teams", &at)?,
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        let orgs = tables(&document, "org")?
            .into_iter()
            .map(|(at, org)| {
                unnamed(org, &["name", "domains", "client"], &at)?;
                Ok(Org {
                    name: required_text(org, "name", &at)?,
                    domains: strings(org, "domains", &at)?,
                    client: flag(org, "client", &at)?,
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(People {
            about,
            persons,
            orgs,
        })
    }
}

/// The settings document `settings` is, or why it is not TOML.
pub(crate) fn document(settings: &str) -> Result<Table, SettingsError> {
    settings.parse::<Table>().map_err(|error| SettingsError {
        field: None,
        reason: format!("the settings are not TOML: {error}"),
    })
}

/// The path of the field `name` of the table at `at`.
pub(crate) fn path(at: &str, name: &str) -> String {
    if at.is_empty() {
        name.to_string()
    } else {
        format!("{at}.{name}")
    }
}

/// Each table of the array of tables `name`, with its path, none where it
/// is left out.
pub(crate) fn tables<'a>(
    document: &'a Table,
    name: &str,
) -> Result<Vec<(String, &'a Table)>, SettingsError> {
    let Some(value) = document.get(name) else {
        return Ok(Vec::new());
    };
    let Toml::Array(items) = value else {
        return Err(SettingsError::new(name, "is not an array of tables"));
    };
    items
        .iter()
        .enumerate()
        .map(|(n, item)| {
            let at = format!("{name}[{n}]");
            match item {
                Toml::Table(table) => Ok((at, table)),
                _ => Err(SettingsError::new(at, "is not a table")),
            }
        })
        .collect()
}

/// Refuses the first key of `table` the profile does not name.
pub(crate) fn unnamed(table: &Table, named: &[&str], at: &str) -> Result<(), SettingsError> {
    match table.keys().find(|key| !named.contains(&key.as_str())) {
        Some(key) => Err(SettingsError::new(
            path(at, key),
            "is a field the profile does not name",
        )),
        None => Ok(()),
    }
}

/// A text, none where it is left out.
pub(crate) fn text(table: &Table, name: &str, at: &str) -> Result<Option<String>, SettingsError> {
    match table.get(name) {
        None => Ok(None),
        Some(Toml::String(text)) => Ok(Some(text.clone())),
        Some(_) => Err(SettingsError::new(path(at, name), "is not a text")),
    }
}

/// A text that must be given.
pub(crate) fn required_text(table: &Table, name: &str, at: &str) -> Result<String, SettingsError> {
    text(table, name, at)?.ok_or_else(|| SettingsError::new(path(at, name), "is missing"))
}

/// A list of texts, empty where it is left out.
fn strings(table: &Table, name: &str, at: &str) -> Result<Vec<String>, SettingsError> {
    let Some(value) = table.get(name) else {
        return Ok(Vec::new());
    };
    let field = path(at, name);
    let Toml::Array(items) = value else {
        return Err(SettingsError::new(field, "is not a list of texts"));
    };
    items
        .iter()
        .enumerate()
        .map(|(n, item)| match item {
            Toml::String(text) => Ok(text.clone()),
            _ => Err(SettingsError::new(format!("{field}[{n}]"), "is not a text")),
        })
        .collect()
}

/// True or false, false where it is left out.
fn flag(table: &Table, name: &str, at: &str) -> Result<bool, SettingsError> {
    match table.get(name) {
        None => Ok(false),
        Some(Toml::Boolean(flag)) => Ok(*flag),
        Some(_) => Err(SettingsError::new(path(at, name), "is not true or false")),
    }
}
