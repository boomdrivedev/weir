//! The people list and the join of attendees and speakers from it (119,
//! 121; C.9).
//!
//! The list is read from `[about]`, `[[person]]` and `[[org]]` of a host's
//! settings document, whose other tables are its host's and are left alone.
//! A field the profile does not name is refused naming it, as a policy's is
//! (record 0010).

use std::fmt;

use toml::{Table, Value as Toml};

use crate::fact::{Facts, Value};

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

impl People {
    /// The joined facts of a capture's stated facts and a signal's read
    /// facts (119, 121), written in no record: each stated `attendee` that
    /// is an address joins the person whose addresses hold it and the org
    /// whose domains hold its domain, and each read `speaker` the person
    /// whose names hold it and the orgs of that person's addresses. An
    /// address or a name two persons hold joins neither, a domain two orgs
    /// hold neither, and each is compared with its case evened (record
    /// 0010). Values are listed in the order of the attendees or speakers
    /// they come from.
    pub fn join(&self, stated: &Facts, read: &Facts) -> Facts {
        let mut joined = Facts::new();
        for given in texts(stated, "attendee") {
            let Some(domain) = domain(given) else {
                continue;
            };
            let person = only(self.persons.iter().filter(|person| {
                person
                    .addresses
                    .iter()
                    .any(|held| even(held) == even(given))
            }));
            if let Some(person) = person {
                give_person(&mut joined, "attendee", person);
            }
            if let Some(org) = self.org_of(domain) {
                give_org(&mut joined, "attendee", org);
            }
        }
        for given in texts(read, "speaker") {
            let person = only(
                self.persons
                    .iter()
                    .filter(|person| person.names.iter().any(|held| even(held) == even(given))),
            );
            let Some(person) = person else {
                continue;
            };
            give_person(&mut joined, "speaker", person);
            let mut orgs: Vec<&Org> = Vec::new();
            for org in person
                .addresses
                .iter()
                .filter_map(|address| domain(address))
                .filter_map(|domain| self.org_of(domain))
            {
                if !orgs.iter().any(|held| std::ptr::eq(*held, org)) {
                    orgs.push(org);
                }
            }
            for org in orgs {
                give_org(&mut joined, "speaker", org);
            }
        }
        joined
    }

    /// The one org whose domains hold `domain`, none where none or several
    /// do.
    fn org_of(&self, domain: &str) -> Option<&Org> {
        only(
            self.orgs
                .iter()
                .filter(|org| org.domains.iter().any(|held| even(held) == even(domain))),
        )
    }
}

/// The text values of the fact `name`.
fn texts<'a>(facts: &'a Facts, name: &str) -> impl Iterator<Item = &'a str> {
    facts
        .get(name)
        .unwrap_or_default()
        .iter()
        .filter_map(|value| match value {
            Value::Text(text) => Some(text.as_str()),
            _ => None,
        })
}

/// The domain of an address: one `@` with text on each side and no blank
/// (record 0010).
fn domain(address: &str) -> Option<&str> {
    let (local, domain) = address.split_once('@')?;
    let shaped = !local.is_empty()
        && !domain.is_empty()
        && !domain.contains('@')
        && !address.chars().any(char::is_whitespace);
    shaped.then_some(domain)
}

/// A text with its case evened, so `Dana@Acme.com` is `dana@acme.com`.
fn even(text: &str) -> String {
    text.to_lowercase()
}

/// The one item `found` holds, none where it holds none or several (119).
fn only<T>(mut found: impl Iterator<Item = T>) -> Option<T> {
    let first = found.next()?;
    found.next().is_none().then_some(first)
}

/// A joined person's facts: `<whom>.name`, `<whom>.role` where it has one,
/// and `<whom>.teams`, each team a value (119; C.9).
fn give_person(joined: &mut Facts, whom: &str, person: &Person) {
    joined.push(&format!("{whom}.name"), Value::Text(person.name.clone()));
    if let Some(role) = &person.role {
        joined.push(&format!("{whom}.role"), Value::Text(role.clone()));
    }
    for team in &person.teams {
        joined.push(&format!("{whom}.teams"), Value::Text(team.clone()));
    }
}

/// A joined org's facts: `<whom>.org`, its name, and `<whom>.client`
/// (119; C.9).
fn give_org(joined: &mut Facts, whom: &str, org: &Org) {
    joined.push(&format!("{whom}.org"), Value::Text(org.name.clone()));
    joined.push(&format!("{whom}.client"), Value::Bool(org.client));
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
