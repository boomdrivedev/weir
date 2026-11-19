//! The pack's kinds cases, `pack/kinds/<case>/case.toml`, each of one
//! form: a declaration and the kinds it reads into, each with its landing
//! fix and its bound where it names them, and each stage with its end, its
//! repeat and its hibernate, and the statechart kinds it reads into, each
//! with its title, its regions and states and its states' parameters (94;
//! C.4); a package held at an identity, laid out in two
//! folders, and the kind, identity and content hash it reads into, checked
//! against a lock where the case names one; or a lock and the identities
//! and hashes it records; or each field refused. Read against the crate,
//! held to "The pack holds the kind format's cases" and "The pack holds
//! package and lock cases" (kinds; 51 to 58, 65, 67, 87 to 93; C.4, C.7 as
//! amended at blueprints 361a599; S17; core.35, core.36). Every folder a
//! package is laid out in is a temporary one of the test's own.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};

use serde::de::DeserializeOwned;
use serde::Deserialize;
use weir_kinds::weir_engine::Region;
use weir_kinds::{
    read_elaboration, read_kinds, read_lock, read_package_dir, ContentHash, Declaration, End,
    Identity, Kind, Package, ParamAgents, Params, Problem, StatechartKind,
};

/// A case as its `case.toml` writes it: a field the format does not name
/// fails the read, naming it, and so does a case of no form or of several.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Written {
    satisfies: Vec<toml::Value>,
    declaration: Option<String>,
    package: Option<PackageCase>,
    lock: Option<String>,
    expect: toml::Table,
}

/// A case read: the clauses it cites, and its one form with what it
/// expects.
#[derive(Debug)]
struct Case {
    satisfies: Vec<toml::Value>,
    form: Form,
}

#[derive(Debug)]
enum Form {
    /// A declaration's text.
    Declaration { text: String, expect: Expect },
    /// A package and the identity it is held at, what it expects boxed so
    /// the variant stays near the others' size.
    Package {
        package: PackageCase,
        held: Identity,
        expect: Box<ExpectPackage>,
    },
    /// A lock's text.
    Lock { text: String, expect: ExpectLock },
}

/// A package as its case writes it: the identity it is held at, its files
/// by their paths within it with their text, its symbolic links by their
/// paths with what each leads to, and a lock's text to check it against.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct PackageCase {
    held: String,
    files: BTreeMap<String, String>,
    #[serde(default)]
    links: BTreeMap<String, String>,
    lock: Option<String>,
}

/// What the declaration reads into: its simple kinds and its statechart
/// kinds, either left out where it reads none of them, the names of the
/// kinds it gives settings to and declares not, and the sections beside
/// them returned unread, or each field refused.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Expect {
    kinds: Option<BTreeMap<String, ExpectKind>>,
    statecharts: Option<BTreeMap<String, ExpectStatechart>>,
    #[serde(default)]
    settings: Vec<String>,
    #[serde(default)]
    unread: Vec<String>,
    refused: Option<Vec<ExpectRefusal>>,
}

/// What the package reads into: the identity it is held at, its content
/// hash, its kind as a declaration case's `kinds` entry is written, and
/// the sections beside its kind returned unread; or each field refused, a
/// lock's refusal of it named at `lock`.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ExpectPackage {
    identity: Option<String>,
    hash: Option<String>,
    kind: Option<ExpectKind>,
    #[serde(default)]
    unread: Vec<String>,
    refused: Option<Vec<ExpectRefusal>>,
}

/// What the lock records, each identity with its hash in identity order,
/// or each field refused.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ExpectLock {
    entries: Option<Vec<ExpectEntry>>,
    refused: Option<Vec<ExpectRefusal>>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ExpectEntry {
    kind: String,
    hash: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ExpectKind {
    version: u64,
    stages: Vec<ExpectStage>,
    /// Its landing fix, the identity as spelled; left out where the kind
    /// names none (52, C.4; record 0005).
    landing_fix: Option<String>,
    /// How many landing fixes it runs; left out where the kind names none
    /// (52, C.4; record 0005).
    max_landing_fixes: Option<u64>,
    /// The kind's sections the format does not name, returned unread.
    #[serde(default)]
    unread: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ExpectStage {
    name: String,
    /// Its deliverable as the format spells it, or `conversation`.
    end: String,
    /// How it repeats as the format spells it; left out when it does not.
    repeat: Option<String>,
    /// Whether it hibernates, as written; left out where the case's stage
    /// leaves it out (53, C.4 as amended at blueprints 30dce35).
    hibernate: Option<bool>,
}

/// A statechart kind as its case writes it (94; C.4).
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ExpectStatechart {
    version: u64,
    title: String,
    /// Its label; left out where the kind names none.
    label: Option<String>,
    /// Its landing fix, the identity as spelled; left out where the kind
    /// names none (94; record 0005).
    landing_fix: Option<String>,
    /// How many landing fixes it runs; left out where the kind names none
    /// (94; record 0005).
    max_landing_fixes: Option<u64>,
    /// The kind's sections the format does not name, returned unread.
    #[serde(default)]
    unread: Vec<String>,
    /// Its regions, every one by name.
    regions: BTreeMap<String, ExpectRegion>,
    /// The stage template's parameters of each state that gives `params`,
    /// by the state's path; none when left out.
    #[serde(default)]
    params: BTreeMap<String, ExpectParams>,
}

/// A region as its case writes it: its initial state and every state.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ExpectRegion {
    initial: String,
    states: BTreeMap<String, ExpectState>,
}

/// A state as its case writes it: whether it is final, false when left
/// out; what it runs, where it runs something; each transition's target in
/// order, none when left out; its decision's answers, none when left out;
/// and its own regions, none when left out.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ExpectState {
    #[serde(default, rename = "final")]
    is_final: bool,
    machine: Option<String>,
    #[serde(default)]
    to: Vec<String>,
    #[serde(default)]
    answers: Vec<String>,
    #[serde(default)]
    regions: BTreeMap<String, ExpectRegion>,
}

/// A state's stage-template parameters as its case writes them, each left
/// out where the state gives none: its agents spelled `entries <n>` or
/// `rule <rule>`, its join, send-back and bound, and its deliverables as
/// the format spells them.
#[derive(Debug, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
struct ExpectParams {
    agents: Option<String>,
    join: Option<String>,
    on_fail: Option<String>,
    max_send_backs: Option<u64>,
    deliverables: Option<Vec<String>>,
}

impl ExpectParams {
    /// The parameters read, spelled as a case writes them.
    fn spelling(params: &Params) -> Self {
        Self {
            agents: params.agents.as_ref().map(|agents| match agents {
                ParamAgents::Entries(entries) => format!("entries {}", entries.len()),
                ParamAgents::Rule { rule, .. } => format!("rule {rule}"),
            }),
            join: params.join.map(|join| join.to_string()),
            on_fail: params.on_fail.clone(),
            max_send_backs: params.max_send_backs,
            deliverables: params
                .deliverables
                .as_ref()
                .map(|all| all.iter().map(ToString::to_string).collect()),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ExpectRefusal {
    field: String,
    reason_holds: String,
}

/// The field a package's case names a lock's refusal of it at: the case's
/// own `lock`, since a mismatch is the lock's and names no field of the
/// package (91, S18).
const LOCK: &str = "lock";

fn folder() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../pack/kinds")
}

/// Reads the case `name` from `text`, or names the case and why not.
fn read_case(name: &str, text: &str) -> Result<Case, String> {
    let written: Written =
        toml::from_str(text).map_err(|error| format!("{name}: {}", error.message()))?;
    if written.satisfies.is_empty() {
        return Err(format!("{name}: `satisfies` cites no clause"));
    }
    let form = match (written.declaration, written.package, written.lock) {
        (Some(text), None, None) => {
            let expect: Expect = expected(name, written.expect)?;
            let reads = expect.kinds.is_some() || expect.statecharts.is_some();
            if reads == expect.refused.is_some() {
                return Err(format!(
                    "{name}: `expect` names its kinds or what is refused, exactly one of them"
                ));
            }
            Form::Declaration { text, expect }
        }
        (None, Some(package), None) => {
            let held = package
                .held
                .parse()
                .map_err(|refusal| format!("{name}: `package.held`: {refusal}"))?;
            lies_within(&package).map_err(|why| format!("{name}: {why}"))?;
            let expect: ExpectPackage = expected(name, written.expect)?;
            let reads = [
                expect.identity.is_some(),
                expect.hash.is_some(),
                expect.kind.is_some(),
            ];
            let refuses = expect.refused.is_some();
            if !(refuses && reads == [false; 3] && expect.unread.is_empty()
                || !refuses && reads == [true; 3])
            {
                return Err(format!(
                    "{name}: `expect` names the package's identity, hash and kind, or what is \
                     refused, exactly one of them"
                ));
            }
            Form::Package {
                package,
                held,
                expect: Box::new(expect),
            }
        }
        (None, None, Some(text)) => {
            let expect: ExpectLock = expected(name, written.expect)?;
            if expect.entries.is_some() == expect.refused.is_some() {
                return Err(format!(
                    "{name}: `expect` names the lock's entries or what is refused, exactly one of them"
                ));
            }
            Form::Lock { text, expect }
        }
        _ => {
            return Err(format!(
                "{name}: a kinds case holds one of a declaration, a package and a lock, exactly \
                 one of them"
            ))
        }
    };
    Ok(Case {
        satisfies: written.satisfies,
        form,
    })
}

/// The case's `expect` read as its form writes it, or the case named with
/// the field that fails it.
fn expected<T: DeserializeOwned>(name: &str, expect: toml::Table) -> Result<T, String> {
    toml::Value::Table(expect)
        .try_into()
        .map_err(|error: toml::de::Error| format!("{name}: `expect`: {}", error.message()))
}

/// Whether every path the case lays out lies within its package, none
/// beneath a link, so laying it out writes nothing outside the folder it
/// is given; or the path that does not.
fn lies_within(package: &PackageCase) -> Result<(), String> {
    let paths: Vec<&String> = package.files.keys().chain(package.links.keys()).collect();
    for path in &paths {
        let plain = path
            .split('/')
            .all(|segment| !segment.is_empty() && segment != "." && segment != "..");
        if !plain {
            return Err(format!(
                "`{path}` is no path within a package, and a case lays its files out within one"
            ));
        }
    }
    for link in package.links.keys() {
        if let Some(path) = paths.iter().find(|path| {
            path.strip_prefix(link.as_str())
                .is_some_and(|rest| rest.starts_with('/'))
        }) {
            return Err(format!(
                "`{path}` lies beneath the link `{link}`, and a case writes nothing through a link"
            ));
        }
    }
    Ok(())
}

/// Every case of the pack, by its folder's name.
fn cases() -> Vec<(String, Case)> {
    let mut names: Vec<String> = fs::read_dir(folder())
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    names
        .into_iter()
        .map(|name| {
            let text = fs::read_to_string(folder().join(&name).join("case.toml"))
                .unwrap_or_else(|error| panic!("{name}: case.toml: {error}"));
            let case = read_case(&name, &text).unwrap_or_else(|why| panic!("{why}"));
            (name, case)
        })
        .collect()
}

/// A stage's end as the case spells it.
fn spelled(end: &End) -> String {
    match end {
        End::Deliverable(deliverable) => deliverable.to_string(),
        End::Conversation => "conversation".to_string(),
    }
}

/// The declaration read, its elaboration sections with it, as whoever runs
/// elaborations reads it: every problem of either, or the declaration.
fn read(text: &str) -> Result<Declaration, Vec<Problem>> {
    let declaration = read_kinds(text).map_err(|refusal| refusal.problems)?;
    let problems: Vec<Problem> = declaration
        .kinds
        .values()
        .filter_map(|kind| read_elaboration(kind).err())
        .flat_map(|refusal| refusal.problems)
        .collect();
    if problems.is_empty() {
        Ok(declaration)
    } else {
        Err(problems)
    }
}

/// How the crate's reading of the case differs from what it expects, each
/// difference naming the case; none when they agree.
fn differences(name: &str, case: &Case) -> Vec<String> {
    match &case.form {
        Form::Declaration { text, expect } => declaration_differences(name, text, expect),
        Form::Package {
            package,
            held,
            expect,
        } => package_differences(name, package, held, expect),
        Form::Lock { text, expect } => lock_differences(name, text, expect),
    }
}

fn declaration_differences(name: &str, text: &str, expect: &Expect) -> Vec<String> {
    let read = read(text);
    if read != self::read(text) {
        return vec![format!("{name}: the declaration read twice reads two ways")];
    }
    match (read, expect) {
        (
            Ok(declaration),
            Expect {
                kinds,
                statecharts,
                settings,
                unread,
                refused: None,
            },
        ) => {
            let no_kinds = BTreeMap::new();
            let kinds = kinds.as_ref().unwrap_or(&no_kinds);
            let no_statecharts = BTreeMap::new();
            let statecharts = statecharts.as_ref().unwrap_or(&no_statecharts);
            let mut said = Vec::new();
            let given: Vec<&String> = declaration.settings.keys().collect();
            if given != settings.iter().collect::<Vec<_>>() {
                said.push(format!(
                    "{name}: reads settings for {given:?}, and the case expects {settings:?}"
                ));
            }
            let beside: Vec<&String> = declaration.sections.keys().collect();
            if beside != unread.iter().collect::<Vec<_>>() {
                said.push(format!(
                    "{name}: returns {beside:?} unread beside the kinds, and the case expects {unread:?}"
                ));
            }
            let read_names: BTreeSet<&String> = declaration.kinds.keys().collect();
            let expected_names: BTreeSet<&String> = kinds.keys().collect();
            if read_names != expected_names {
                said.push(format!(
                    "{name}: reads the kinds {read_names:?}, and the case expects {expected_names:?}"
                ));
            }
            for (kind_name, expected) in kinds {
                if let Some(kind) = declaration.kinds.get(kind_name) {
                    said.extend(kind_differences(name, kind, expected));
                }
            }
            let read_names: BTreeSet<&String> = declaration.statecharts.keys().collect();
            let expected_names: BTreeSet<&String> = statecharts.keys().collect();
            if read_names != expected_names {
                said.push(format!(
                    "{name}: reads the statechart kinds {read_names:?}, and the case expects \
                     {expected_names:?}"
                ));
            }
            for (kind_name, expected) in statecharts {
                if let Some(kind) = declaration.statecharts.get(kind_name) {
                    said.extend(statechart_differences(name, kind, expected));
                }
            }
            said
        }
        (
            Err(problems),
            Expect {
                refused: Some(refused),
                ..
            },
        ) => refusal_differences(name, &problems, refused),
        (Ok(declaration), _) => vec![format!(
            "{name}: reads the kinds {:?}, and the case expects a refusal",
            declaration
                .kinds
                .keys()
                .chain(declaration.statecharts.keys())
                .collect::<Vec<_>>()
        )],
        (Err(problems), _) => vec![format!(
            "{name}: refuses {:?}, and the case expects its kinds",
            spelled_problems(&problems)
        )],
    }
}

/// How the package laid out at two paths and read at its identity, then
/// checked against its lock, differs from what its case expects.
fn package_differences(
    name: &str,
    package: &PackageCase,
    held: &Identity,
    expect: &ExpectPackage,
) -> Vec<String> {
    let (first, second) = match (
        read_laid_out(package, held, &[]),
        read_laid_out(package, held, &["deeper", "still"]),
    ) {
        (Ok(first), Ok(second)) => (first, second),
        (Err(why), _) | (_, Err(why)) => return vec![format!("{name}: {why}")],
    };
    if first != second {
        return vec![format!(
            "{name}: the package laid out at two paths reads two ways, {first:?} and {second:?}"
        )];
    }
    let read = match (first, &package.lock) {
        (Ok(read), Some(lock)) => match read_lock(lock) {
            Err(refusal) => return vec![format!("{name}: its lock is refused: {refusal}")],
            Ok(lock) => match lock.check(&read.identity, &read.hash) {
                Ok(_) => Ok(read),
                Err(mismatch) => Err(vec![Problem {
                    field: LOCK.to_owned(),
                    reason: mismatch.to_string(),
                }]),
            },
        },
        (read, _) => read.map_err(|refusal| refusal.problems),
    };
    match (read, expect) {
        (
            Ok(read),
            ExpectPackage {
                identity: Some(identity),
                hash: Some(hash),
                kind: Some(kind),
                unread,
                ..
            },
        ) => {
            let mut said = Vec::new();
            if read.identity.to_string() != *identity {
                said.push(format!(
                    "{name}: reads at `{}`, and the case expects `{identity}`",
                    read.identity
                ));
            }
            if read.hash.to_string() != *hash {
                said.push(format!(
                    "{name}: hashes to `{}`, and the case expects `{hash}`",
                    read.hash
                ));
            }
            let beside: Vec<&String> = read.sections.keys().collect();
            if beside != unread.iter().collect::<Vec<_>>() {
                said.push(format!(
                    "{name}: returns {beside:?} unread beside the kind, and the case expects {unread:?}"
                ));
            }
            match read.kind.simple() {
                Some(read_kind) => said.extend(kind_differences(name, read_kind, kind)),
                None => said.push(format!(
                    "{name}: reads a statechart kind, and the case expects a simple kind"
                )),
            }
            said
        }
        (
            Err(problems),
            ExpectPackage {
                refused: Some(refused),
                ..
            },
        ) => refusal_differences(name, &problems, refused),
        (Ok(read), _) => vec![format!(
            "{name}: reads `{}` at `{}`, and the case expects a refusal",
            read.identity, read.hash
        )],
        (Err(problems), _) => vec![format!(
            "{name}: refuses {:?}, and the case expects its kind",
            spelled_problems(&problems)
        )],
    }
}

/// The package laid out in a temporary folder of its own, `depth` folders
/// below it, and read there at `held`; or why it could not be laid out.
fn read_laid_out(
    package: &PackageCase,
    held: &Identity,
    depth: &[&str],
) -> Result<Result<Package, weir_kinds::Refusal>, String> {
    let top = tempfile::tempdir().map_err(|error| format!("a temporary folder: {error}"))?;
    let mut dir = top.path().to_path_buf();
    dir.extend(depth);
    dir.push("pkg");
    lay_out(&dir, package)?;
    Ok(read_package_dir(&dir, held))
}

/// Writes the case's files under `dir`, then makes its links, each with
/// the folders it lies in; or why not, naming the path.
fn lay_out(dir: &Path, package: &PackageCase) -> Result<(), String> {
    for (path, text) in &package.files {
        let at = dir.join(path);
        fs::create_dir_all(at.parent().unwrap())
            .and_then(|()| fs::write(&at, text))
            .map_err(|error| format!("`{path}` could not be written: {error}"))?;
    }
    for (path, target) in &package.links {
        let at = dir.join(path);
        fs::create_dir_all(at.parent().unwrap())
            .and_then(|()| symlink(target, &at))
            .map_err(|error| format!("`{path}` could not be made a link: {error}"))?;
    }
    Ok(())
}

fn lock_differences(name: &str, text: &str, expect: &ExpectLock) -> Vec<String> {
    let read = read_lock(text);
    if read != read_lock(text) {
        return vec![format!("{name}: the lock read twice reads two ways")];
    }
    match (read, expect) {
        (
            Ok(lock),
            ExpectLock {
                entries: Some(entries),
                ..
            },
        ) => {
            let recorded: Vec<(String, String)> = lock
                .entries
                .iter()
                .map(|(identity, entry)| (identity.to_string(), entry.hash.to_string()))
                .collect();
            let expected: Vec<(String, String)> = entries
                .iter()
                .map(|entry| (entry.kind.clone(), entry.hash.clone()))
                .collect();
            if recorded == expected {
                Vec::new()
            } else {
                vec![format!(
                    "{name}: records {recorded:?}, and the case expects {expected:?}"
                )]
            }
        }
        (
            Err(refusal),
            ExpectLock {
                refused: Some(refused),
                ..
            },
        ) => refusal_differences(name, &refusal.problems, refused),
        (Ok(lock), _) => vec![format!(
            "{name}: records {:?}, and the case expects a refusal",
            lock.entries
                .keys()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
        )],
        (Err(refusal), _) => vec![format!(
            "{name}: refuses {:?}, and the case expects its entries",
            spelled_problems(&refusal.problems)
        )],
    }
}

/// How a kind read differs from the kind its case writes.
fn kind_differences(name: &str, kind: &Kind, expected: &ExpectKind) -> Vec<String> {
    let mut said = Vec::new();
    let kind_name = &kind.name;
    let stages: Vec<(String, String, Option<String>, Option<bool>)> = kind
        .stages
        .iter()
        .map(|stage| {
            (
                stage.name.clone(),
                spelled(&stage.end),
                stage.repeat.map(|repeat| repeat.to_string()),
                stage.hibernate,
            )
        })
        .collect();
    let expected_stages: Vec<(String, String, Option<String>, Option<bool>)> = expected
        .stages
        .iter()
        .map(|stage| {
            (
                stage.name.clone(),
                stage.end.clone(),
                stage.repeat.clone(),
                stage.hibernate,
            )
        })
        .collect();
    if kind.version != expected.version || stages != expected_stages {
        said.push(format!(
            "{name}: reads {kind_name}@{} with {stages:?}, and the case expects \
             {kind_name}@{} with {expected_stages:?}",
            kind.version, expected.version
        ));
    }
    let landing_fix = kind.landing_fix.as_ref().map(ToString::to_string);
    if landing_fix != expected.landing_fix {
        said.push(format!(
            "{name}: reads {kind_name} with landing_fix {landing_fix:?}, and the case expects landing_fix {:?}",
            expected.landing_fix
        ));
    }
    if kind.max_landing_fixes != expected.max_landing_fixes {
        said.push(format!(
            "{name}: reads {kind_name} with max_landing_fixes {:?}, and the case expects max_landing_fixes {:?}",
            kind.max_landing_fixes, expected.max_landing_fixes
        ));
    }
    let unread: Vec<&String> = kind.sections.keys().collect();
    if unread != expected.unread.iter().collect::<Vec<_>>() {
        said.push(format!(
            "{name}: returns {unread:?} of {kind_name} unread, and the case expects {:?}",
            expected.unread
        ));
    }
    said
}

/// How a statechart kind read differs from the statechart kind its case
/// writes, each difference naming the case, the kind and the field (94;
/// C.4).
fn statechart_differences(
    name: &str,
    kind: &StatechartKind,
    expected: &ExpectStatechart,
) -> Vec<String> {
    let mut said = Vec::new();
    let kind_name = kind.name();
    let mut differs = |field: &str, read: String, expects: String| {
        if read != expects {
            said.push(format!(
                "{name}: reads {kind_name} with {field} {read}, and the case expects {expects}"
            ));
        }
    };
    differs(
        "version",
        kind.version().to_string(),
        expected.version.to_string(),
    );
    differs(
        "title",
        format!("{:?}", kind.title),
        format!("{:?}", expected.title),
    );
    differs(
        "label",
        format!("{:?}", kind.label),
        format!("{:?}", expected.label),
    );
    differs(
        "landing_fix",
        format!("{:?}", kind.landing_fix.as_ref().map(ToString::to_string)),
        format!("{:?}", expected.landing_fix),
    );
    differs(
        "max_landing_fixes",
        format!("{:?}", kind.max_landing_fixes),
        format!("{:?}", expected.max_landing_fixes),
    );
    differs(
        "unread",
        format!("{:?}", kind.sections.keys().collect::<Vec<_>>()),
        format!("{:?}", expected.unread),
    );
    regions_differences(
        "regions",
        &kind.statechart.regions,
        &expected.regions,
        &mut differs,
    );
    differs(
        "params",
        format!("{:?}", kind.params.keys().collect::<Vec<_>>()),
        format!("{:?}", expected.params.keys().collect::<Vec<_>>()),
    );
    for (path, expects) in &expected.params {
        if let Some(read) = kind.params.get(path) {
            let read = ExpectParams::spelling(read);
            let at = format!("params.{path:?}");
            differs(
                &format!("{at}.agents"),
                format!("{:?}", read.agents),
                format!("{:?}", expects.agents),
            );
            differs(
                &format!("{at}.join"),
                format!("{:?}", read.join),
                format!("{:?}", expects.join),
            );
            differs(
                &format!("{at}.on_fail"),
                format!("{:?}", read.on_fail),
                format!("{:?}", expects.on_fail),
            );
            differs(
                &format!("{at}.max_send_backs"),
                format!("{:?}", read.max_send_backs),
                format!("{:?}", expects.max_send_backs),
            );
            differs(
                &format!("{at}.deliverables"),
                format!("{:?}", read.deliverables),
                format!("{:?}", expects.deliverables),
            );
        }
    }
    said
}

/// How regions read at `at` differ from the regions a case writes, each
/// difference given to `differs` with its field.
fn regions_differences(
    at: &str,
    read: &BTreeMap<String, Region>,
    expected: &BTreeMap<String, ExpectRegion>,
    differs: &mut impl FnMut(&str, String, String),
) {
    differs(
        at,
        format!("{:?}", read.keys().collect::<Vec<_>>()),
        format!("{:?}", expected.keys().collect::<Vec<_>>()),
    );
    for (name, expects) in expected {
        let Some(region) = read.get(name) else {
            continue;
        };
        let at = format!("{at}.{name}");
        differs(
            &format!("{at}.initial"),
            format!("{:?}", region.initial),
            format!("{:?}", expects.initial),
        );
        differs(
            &format!("{at}.states"),
            format!("{:?}", region.states.keys().collect::<Vec<_>>()),
            format!("{:?}", expects.states.keys().collect::<Vec<_>>()),
        );
        for (state_name, expects) in &expects.states {
            let Some(state) = region.states.get(state_name) else {
                continue;
            };
            let at = format!("{at}.states.{state_name}");
            differs(
                &format!("{at}.final"),
                state.is_final.to_string(),
                expects.is_final.to_string(),
            );
            differs(
                &format!("{at}.machine"),
                format!("{:?}", state.machine),
                format!("{:?}", expects.machine),
            );
            let to: Vec<&String> = state.transitions.iter().map(|t| &t.to).collect();
            differs(
                &format!("{at}.to"),
                format!("{to:?}"),
                format!("{:?}", expects.to),
            );
            let answers: Vec<String> = state
                .decision
                .as_ref()
                .map(|decision| decision.answers.clone())
                .unwrap_or_default();
            differs(
                &format!("{at}.answers"),
                format!("{answers:?}"),
                format!("{:?}", expects.answers),
            );
            regions_differences(
                &format!("{at}.regions"),
                &state.regions,
                &expects.regions,
                differs,
            );
        }
    }
}

/// How the problems given differ from the refusal the case expects: each
/// field, in order, and a phrase of each reason.
fn refusal_differences(name: &str, problems: &[Problem], refused: &[ExpectRefusal]) -> Vec<String> {
    let agrees = problems.len() == refused.len()
        && problems.iter().zip(refused).all(|(problem, expected)| {
            problem.field == expected.field && problem.reason.contains(&expected.reason_holds)
        });
    if agrees {
        Vec::new()
    } else {
        vec![format!(
            "{name}: refuses {:?}, and the case expects {refused:?}",
            spelled_problems(problems)
        )]
    }
}

fn spelled_problems(problems: &[Problem]) -> Vec<String> {
    problems.iter().map(ToString::to_string).collect()
}

/// The case `name` of the pack.
fn case(name: &str) -> Case {
    cases()
        .into_iter()
        .find_map(|(found, case)| (found == name).then_some(case))
        .unwrap_or_else(|| panic!("no kinds case {name}"))
}

fn declaration_expect(case: &Case) -> &Expect {
    match &case.form {
        Form::Declaration { expect, .. } => expect,
        other => panic!("a declaration's case, not {other:?}"),
    }
}

fn package_expect(case: &Case) -> &ExpectPackage {
    match &case.form {
        Form::Package { expect, .. } => expect,
        other => panic!("a package's case, not {other:?}"),
    }
}

#[test]
fn every_kinds_case_is_answered_as_it_says() {
    let cases = cases();
    assert!(!cases.is_empty(), "no cases under {}", folder().display());
    let differences: Vec<String> = cases
        .iter()
        .flat_map(|(name, case)| differences(name, case))
        .collect();
    assert_eq!(differences, Vec::<String>::new());
}

#[test]
fn the_cases_cite_each_of_51_to_58_and_88_to_93() {
    let cited: BTreeSet<i64> = cases()
        .iter()
        .flat_map(|(_, case)| case.satisfies.iter().filter_map(toml::Value::as_integer))
        .collect();
    for clause in (51..=58).chain(88..=93) {
        assert!(cited.contains(&clause), "no kinds case cites {clause}");
    }
}

#[test]
fn the_cases_of_s10_s11_and_s12() {
    let flume = case("flumes-own-declaration-read-twice-the-same");
    let kinds = declaration_expect(&flume).kinds.as_ref().unwrap();
    assert_eq!(kinds.keys().collect::<Vec<_>>(), ["change", "fix"]);
    let change_2 = case("flumes-change-2-read-twice-the-same");
    let change = &declaration_expect(&change_2).kinds.as_ref().unwrap()["change"];
    assert_eq!(change.version, 2);
    let repeats: Vec<_> = change
        .stages
        .iter()
        .map(|stage| (stage.name.as_str(), stage.repeat.as_deref()))
        .collect();
    assert_eq!(
        repeats,
        [
            ("construct", None),
            ("code", Some("per-task-group")),
            ("verify", None),
        ]
    );
    let elaboration = case("an-elaboration-kind");
    assert_eq!(
        declaration_expect(&elaboration).kinds.as_ref().unwrap()["writeback"].unread,
        ["elaboration"]
    );
    for name in [
        "a-stage-with-two-deliverables",
        "a-kind-that-does-not-land-with-no-stages",
    ] {
        assert_eq!(
            declaration_expect(&case(name))
                .refused
                .as_ref()
                .unwrap()
                .len(),
            1,
            "{name}"
        );
    }
}

/// A kind that lands with `stages = []` reads with none, and its two
/// neighbours are refused naming their kind's `stages`: one with `stages =
/// []` that does not land, one that lands with `stages` left out (51, 57
/// as amended at blueprints 51b6316; S12).
#[test]
fn a_kind_that_lands_with_no_stages_and_its_two_neighbours() {
    let name = "a-kind-that-lands-with-no-stages";
    let landing = case(name);
    let Form::Declaration { text, expect } = &landing.form else {
        panic!("a declaration's case");
    };
    assert!(text.contains("lands = true"), "{text}");
    assert!(text.contains("stages = []"), "{text}");
    let kinds = expect.kinds.as_ref().unwrap();
    assert_eq!(kinds.keys().collect::<Vec<_>>(), ["train"]);
    assert_eq!(kinds["train"].version, 1);
    assert!(kinds["train"].stages.is_empty());
    assert_eq!(differences(name, &landing), Vec::<String>::new());

    for (name, lands, stages, reason) in [
        (
            "a-kind-that-does-not-land-with-no-stages",
            false,
            true,
            "a kind that does not land has at least one stage",
        ),
        (
            "a-kind-that-lands-with-its-stages-left-out",
            true,
            false,
            "a kind declares its stages",
        ),
    ] {
        let neighbour = case(name);
        let Form::Declaration { text, expect } = &neighbour.form else {
            panic!("{name}: a declaration's case");
        };
        assert_eq!(text.contains("lands = true"), lands, "{name}: {text}");
        assert_eq!(text.contains("stages = []"), stages, "{name}: {text}");
        let refused = expect.refused.as_ref().unwrap();
        assert_eq!(refused.len(), 1, "{name}");
        assert_eq!(refused[0].field, "kinds.bump.stages", "{name}");
        assert_eq!(refused[0].reason_holds, reason, "{name}");
        assert_eq!(differences(name, &neighbour), Vec::<String>::new());
    }
}

#[test]
fn flumes_change_1_and_2_each_a_package_of_its_own() {
    let one = case("flumes-change-1-as-a-package");
    let two = case("flumes-change-2-as-a-package");
    let (one, two) = (package_expect(&one), package_expect(&two));
    assert_eq!(one.identity.as_deref(), Some("flume/change@1"));
    assert_eq!(two.identity.as_deref(), Some("flume/change@2"));
    assert_ne!(one.hash, two.hash);
    let repeats = |expect: &ExpectPackage| -> Vec<Option<String>> {
        let kind = expect.kind.as_ref().unwrap();
        kind.stages
            .iter()
            .map(|stage| stage.repeat.clone())
            .collect()
    };
    assert_eq!(repeats(one), [None, None, None]);
    assert_eq!(
        repeats(two),
        [None, Some("per-task-group".to_owned()), None]
    );
}

#[test]
fn the_cases_of_s17_and_s18() {
    let outside = case("a-prompt-outside-its-package");
    let refused = package_expect(&outside).refused.as_ref().unwrap();
    assert_eq!(refused.len(), 1);
    assert_eq!(refused[0].field, "kinds.review.stages[0].prompt");

    let inside = case("the-prompt-inside-its-package");
    let hash = package_expect(&inside).hash.clone().unwrap();

    let linked = case("a-prompt-that-is-a-link");
    let Form::Package { package, .. } = &linked.form else {
        panic!("a package's case");
    };
    assert_eq!(
        package.links.keys().collect::<Vec<_>>(),
        ["prompts/code.md"]
    );
    let refused = package_expect(&linked).refused.as_ref().unwrap();
    assert_eq!(refused[0].field, "prompts/code.md");

    let changed = case("a-package-changed-by-a-byte");
    let Form::Package { package, .. } = &changed.form else {
        panic!("a package's case");
    };
    assert!(package.lock.as_deref().unwrap().contains(&hash));
    let refused = package_expect(&changed).refused.as_ref().unwrap();
    assert_eq!(refused.len(), 1);
    assert_eq!(refused[0].field, LOCK);
    let reason = &refused[0].reason_holds;
    assert!(reason.contains("`acme/review@1`"), "{reason}");
    assert!(reason.contains(&hash), "{reason}");
    assert_eq!(reason.matches("sha256:").count(), 2, "{reason}");
}

#[test]
fn a_stage_with_two_deliverables() {
    let name = "a-stage-with-two-deliverables";
    let case = case(name);
    let Form::Declaration { text, .. } = &case.form else {
        panic!("a declaration's case");
    };
    assert!(text.contains(r#"deliverables = ["commits", "tasks"]"#));
    let problems = read(text).unwrap_err();
    assert_eq!(problems[0].field, "kinds.bump.stages[0].deliverables");
    assert_eq!(differences(name, &case), Vec::<String>::new());
}

#[test]
fn a_misspelt_field_of_the_case() {
    let refused = read_case(
        "misspelt",
        "satisfies = [57]\ndeclaration = \"\"\n\n[kind.fix]\nversion = 1\n",
    )
    .expect_err("a refusal");
    assert!(refused.starts_with("misspelt: "), "{refused}");
    assert!(refused.contains("`kind`"), "{refused}");

    let package = "satisfies = [92]\n[package]\nheld = \"acme/review@1\"\n";
    for (text, field) in [
        (
            format!("{package}file = {{ \"kind.toml\" = \"\" }}\n[expect]\n"),
            "`file`",
        ),
        (
            format!(
                "{package}files = {{ \"kind.toml\" = \"\" }}\n[expect]\nidentity = \"acme/review@1\"\nhashes = \"\"\n"
            ),
            "`hashes`",
        ),
        (
            "satisfies = [93]\nlock = \"\"\n[expect]\nentry = []\n".to_owned(),
            "`entry`",
        ),
    ] {
        let refused = read_case("misspelt", &text).expect_err("a refusal");
        assert!(refused.starts_with("misspelt: "), "{refused}");
        assert!(refused.contains(field), "{refused}");
    }
}

#[test]
fn a_case_of_more_than_one_form() {
    for text in [
        "satisfies = [91]\ndeclaration = \"\"\nlock = \"\"\n[expect]\n",
        "satisfies = [91]\nlock = \"\"\n[package]\nheld = \"acme/review@1\"\nfiles = {}\n[expect]\nentries = []\n",
        "satisfies = [91]\n[expect]\nentries = []\n",
    ] {
        let refused = read_case("both", text).expect_err("a refusal");
        assert!(refused.starts_with("both: "), "{refused}");
        assert!(
            refused.contains("holds one of a declaration, a package and a lock"),
            "{refused}"
        );
    }
}

#[test]
fn a_case_lays_out_nothing_outside_its_package() {
    let head = "satisfies = [89]\n[package]\nheld = \"acme/review@1\"\n";
    for (paths, says) in [
        (
            "files = { \"../kind.toml\" = \"\" }\n",
            "`../kind.toml` is no path",
        ),
        (
            "files = { \"/kind.toml\" = \"\" }\n",
            "`/kind.toml` is no path",
        ),
        ("files = { \"a//b.md\" = \"\" }\n", "`a//b.md` is no path"),
        (
            "files = {}\nlinks = { \"./a\" = \"b\" }\n",
            "`./a` is no path",
        ),
        (
            "files = { \"out/kind.toml\" = \"\" }\nlinks = { out = \"/tmp\" }\n",
            "`out/kind.toml` lies beneath the link `out`",
        ),
    ] {
        let text = format!("{head}{paths}[expect]\nrefused = []\n");
        let refused = read_case("outside", &text).expect_err("a refusal");
        assert!(refused.starts_with("outside: "), "{refused}");
        assert!(refused.contains(says), "{refused}");
    }
}

#[test]
fn a_package_case_that_differs_is_named() {
    let text = r#"
satisfies = [92]

[package]
held = "acme/review@1"
files = { "kind.toml" = """
[[kinds.review.stages]]
name = "review"
prompt = "review.md"
deliverables = ["commits"]
""", "review.md" = "Review.\n" }

[expect]
identity = "acme/review@1"
hash = "sha256:0000000000000000000000000000000000000000000000000000000000000000"

[expect.kind]
version = 1
stages = [{ name = "review", end = "commits" }]
"#;
    let case = read_case("wrong-hash", text).unwrap();
    let said = differences("wrong-hash", &case);
    assert_eq!(said.len(), 1, "{said:?}");
    assert!(
        said[0].starts_with("wrong-hash: hashes to `sha256:"),
        "{said:?}"
    );
}

/// S19's declaration, a simple kind beside a statechart kind, read the
/// same twice and as the kinds crate's `persona.toml`; its third kind
/// holding both `stages` and `regions`, refused naming its `regions`; and
/// S20's transition, refused naming its `to` (51, 81, 94, 95; C.4).
#[test]
fn the_cases_of_s19_and_s20() {
    let name = "a-statechart-kind-beside-a-simple-kind";
    let beside = case(name);
    let Form::Declaration { text, expect } = &beside.form else {
        panic!("{name}: a declaration's case");
    };
    let fixture = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/persona.toml"),
    )
    .unwrap();
    assert_eq!(read(text), read(&fixture), "{name}");
    let kinds = expect.kinds.as_ref().unwrap();
    assert_eq!(kinds.keys().collect::<Vec<_>>(), ["bump"]);
    let stages: Vec<(&str, &str)> = kinds["bump"]
        .stages
        .iter()
        .map(|stage| (stage.name.as_str(), stage.end.as_str()))
        .collect();
    assert_eq!(stages, [("code", "commits"), ("review", "conversation")]);
    let statecharts = expect.statecharts.as_ref().unwrap();
    assert_eq!(statecharts.keys().collect::<Vec<_>>(), ["persona"]);
    let persona = &statecharts["persona"];
    assert_eq!(
        persona.regions["stages"].states["test"].machine.as_deref(),
        Some("stage")
    );
    let test = &persona.params["stages.test"];
    assert_eq!(
        test.agents.as_deref(),
        Some("rule every-persona-definition")
    );
    assert_eq!(test.join.as_deref(), Some("all"));
    assert_eq!(differences(name, &beside), Vec::<String>::new());

    for (name, field, holds) in [
        (
            "a-kind-holding-both-stages-and-regions",
            "kinds.both.regions",
            "beside `stages`",
        ),
        (
            "a-transition-to-a-state-its-region-does-not-hold",
            "kinds.persona.regions.stages.states.test.transitions[0].to",
            "`tested`",
        ),
    ] {
        let refusing = case(name);
        let refused = declaration_expect(&refusing).refused.as_ref().unwrap();
        assert_eq!(refused.len(), 1, "{name}");
        assert_eq!(refused[0].field, field, "{name}");
        assert!(refused[0].reason_holds.contains(holds), "{name}");
        assert_eq!(differences(name, &refusing), Vec::<String>::new());
    }
}

#[test]
fn the_cases_cite_94_and_95() {
    let cited: BTreeSet<i64> = cases()
        .iter()
        .flat_map(|(_, case)| case.satisfies.iter().filter_map(toml::Value::as_integer))
        .collect();
    for clause in [94, 95] {
        assert!(cited.contains(&clause), "no kinds case cites {clause}");
    }
}

#[test]
fn a_case_whose_statechart_differs_is_named() {
    // A statechart kind is held against the kind read, each difference
    // naming the case, the kind and the field (94; C.4; C.7).
    let text = r#"
satisfies = [94]
declaration = """
[kinds.persona]
title = "Persona test"

[kinds.persona.regions.stages]
initial = "test"

[kinds.persona.regions.stages.states.test]
machine = "stage"
params = { join = "all" }
transitions = [
  { when = { final = "passed" }, to = "passed" },
  { when = { final = "sent-back" }, to = "build" },
  { when = { final = "stopped" }, to = "stopped" },
]

[kinds.persona.regions.stages.states.build]
[kinds.persona.regions.stages.states.passed]
final = true
[kinds.persona.regions.stages.states.stopped]
"""

[expect.statecharts.persona]
version = 1
title = "Persona test"

[expect.statecharts.persona.regions.stages]
initial = "test"
states.build = {}
states.passed = { final = true }
states.stopped = {}
states.test = { machine = "stage", to = ["passed", "stopped"] }

[expect.statecharts.persona.params."stages.test"]
join = "all"
"#;
    let case = read_case("wrong-targets", text).unwrap();
    assert_eq!(
        differences("wrong-targets", &case),
        [
            r#"wrong-targets: reads persona with regions.stages.states.test.to ["passed", "build", "stopped"], and the case expects ["passed", "stopped"]"#
        ]
    );

    let unexpected = text.replace("[expect.statecharts.persona]", "[expect.statecharts.other]");
    let unexpected = unexpected.replace("expect.statecharts.persona.", "expect.statecharts.other.");
    let case = read_case("unexpected", &unexpected).unwrap();
    assert_eq!(
        differences("unexpected", &case),
        [r#"unexpected: reads the statechart kinds {"persona"}, and the case expects {"other"}"#]
    );

    let misspelt = text.replace("states.build = {}", "states.build = { finale = true }");
    let refused = read_case("misspelt", &misspelt).expect_err("a refusal");
    assert!(refused.contains("`finale`"), "{refused}");
}

#[test]
fn a_case_whose_kind_differs_in_its_fix_is_named() {
    // A kind's landing fix and its bound are held against the kind read
    // (52, C.4 as amended at blueprints 8eec6dd; C.7).
    let text = r#"
satisfies = [52]
declaration = """
[kinds.train]
lands = true
stages = []
landing_fix = "acme/other@1"
max_landing_fixes = 2
"""

[expect.kinds.train]
version = 1
stages = []
landing_fix = "acme/fix@1"
"#;
    let case = read_case("wrong-fix", text).unwrap();
    assert_eq!(
        differences("wrong-fix", &case),
        [
            r#"wrong-fix: reads train with landing_fix Some("acme/other@1"), and the case expects landing_fix Some("acme/fix@1")"#,
            "wrong-fix: reads train with max_landing_fixes Some(2), and the case expects max_landing_fixes None",
        ]
    );
}

#[test]
fn a_case_whose_stage_differs_in_its_hibernate_is_named() {
    // A stage's hibernate is held against the stage read, a stage the case
    // expects with it left out being one read with it left out (53, C.4 as
    // amended at blueprints 30dce35; C.7).
    let text = r#"
satisfies = [53]
declaration = """
[[kinds.talk.stages]]
name = "draft"
prompt = "prompts/talk-draft.md"
conversation = true
"""

[expect.kinds.talk]
version = 1
stages = [{ name = "draft", end = "conversation", hibernate = true }]
"#;
    let case = read_case("wrong-hibernate", text).unwrap();
    assert_eq!(
        differences("wrong-hibernate", &case),
        [
            r#"wrong-hibernate: reads talk@1 with [("draft", "conversation", None, None)], and the case expects talk@1 with [("draft", "conversation", None, Some(true))]"#,
        ]
    );
}

/// A kind that lands with no stages names its landing fix and its bound,
/// read with both as a declaration and as a package, and a kind with
/// stages naming both is refused naming each (52, 57 and 89 as amended at
/// blueprints 8eec6dd; 92).
#[test]
fn a_kind_that_names_its_fix_and_one_with_stages_that_may_not() {
    let cites = |case: &Case| -> Vec<i64> {
        case.satisfies
            .iter()
            .filter_map(toml::Value::as_integer)
            .collect()
    };
    let name = "a-kind-that-lands-with-no-stages-names-its-fix";
    let naming = case(name);
    assert_eq!(cites(&naming), [52, 57]);
    let Form::Declaration {
        text: train,
        expect,
    } = &naming.form
    else {
        panic!("{name}: a declaration's case");
    };
    for line in [
        "lands = true",
        "stages = []",
        r#"landing_fix = "acme/fix@1""#,
        "max_landing_fixes = 2",
    ] {
        assert!(train.contains(line), "{name}: {train}");
    }
    let kinds = expect.kinds.as_ref().unwrap();
    assert_eq!(kinds.keys().collect::<Vec<_>>(), ["train"]);
    let expected = &kinds["train"];
    assert_eq!(expected.version, 1);
    assert!(expected.stages.is_empty());
    assert_eq!(expected.landing_fix.as_deref(), Some("acme/fix@1"));
    assert_eq!(expected.max_landing_fixes, Some(2));
    assert_eq!(differences(name, &naming), Vec::<String>::new());

    let name = "a-kind-with-stages-that-names-its-fix";
    let with_stages = case(name);
    assert_eq!(cites(&with_stages), [52, 57]);
    let Form::Declaration { text, expect } = &with_stages.form else {
        panic!("{name}: a declaration's case");
    };
    for line in [
        "lands = true",
        r#"landing_fix = "acme/fix@1""#,
        "max_landing_fixes = 2",
    ] {
        assert!(text.contains(line), "{name}: {text}");
    }
    assert_eq!(
        text.matches("[[kinds.bump.stages]]").count(),
        1,
        "{name}: {text}"
    );
    let refused = expect.refused.as_ref().unwrap();
    let fields: Vec<_> = refused.iter().map(|r| r.field.as_str()).collect();
    assert_eq!(
        fields,
        ["kinds.bump.landing_fix", "kinds.bump.max_landing_fixes"],
        "{name}"
    );
    for refusal in refused {
        assert_eq!(
            refusal.reason_holds, "only a kind that lands with no stages names",
            "{name}"
        );
    }
    assert_eq!(differences(name, &with_stages), Vec::<String>::new());

    let name = "a-package-of-a-kind-that-names-its-fix";
    let packaged = case(name);
    assert_eq!(cites(&packaged), [52, 89, 92]);
    let Form::Package {
        package, expect, ..
    } = &packaged.form
    else {
        panic!("{name}: a package's case");
    };
    assert_eq!(package.held, "acme/train@1");
    assert_eq!(package.files.keys().collect::<Vec<_>>(), ["kind.toml"]);
    assert_eq!(&package.files["kind.toml"], train, "{name}");
    assert!(package.links.is_empty() && package.lock.is_none(), "{name}");
    assert_eq!(expect.identity.as_deref(), Some("acme/train@1"));
    assert_eq!(
        expect.hash.as_deref(),
        Some(ContentHash::of(&package.files).to_string().as_str())
    );
    let kind = expect.kind.as_ref().unwrap();
    assert_eq!(kind.version, 1);
    assert!(kind.stages.is_empty());
    assert_eq!(kind.landing_fix.as_deref(), Some("acme/fix@1"));
    assert_eq!(kind.max_landing_fixes, Some(2));
    assert_eq!(differences(name, &packaged), Vec::<String>::new());
}

/// A landing fix named by a bare name is a case of its own, refused naming
/// the field, since a landing fix is an identity with its version (52, 57
/// as amended at blueprints 8eec6dd; record 0005).
#[test]
fn a_landing_fix_that_is_no_identity_is_a_case_of_its_own() {
    let name = "a-landing-fix-that-is-no-identity";
    let bare = case(name);
    let cites: Vec<i64> = bare
        .satisfies
        .iter()
        .filter_map(toml::Value::as_integer)
        .collect();
    assert_eq!(cites, [52, 57]);
    let Form::Declaration { text, expect } = &bare.form else {
        panic!("{name}: a declaration's case");
    };
    for line in ["lands = true", "stages = []", r#"landing_fix = "fix""#] {
        assert!(text.contains(line), "{name}: {text}");
    }
    let refused = expect.refused.as_ref().unwrap();
    let named: Vec<_> = refused
        .iter()
        .map(|r| (r.field.as_str(), r.reason_holds.as_str()))
        .collect();
    assert_eq!(
        named,
        [("kinds.train.landing_fix", "is no identity")],
        "{name}"
    );
    assert_eq!(differences(name, &bare), Vec::<String>::new());
}

/// A stage that hibernates, one that does not and one that leaves it out,
/// a case of its own (53, 58; C.4 as amended at blueprints 30dce35; C.7).
#[test]
fn a_stage_that_hibernates_is_a_case_of_its_own() {
    let name = "a-stage-that-hibernates";
    let hibernates = case(name);
    let cites: Vec<i64> = hibernates
        .satisfies
        .iter()
        .filter_map(toml::Value::as_integer)
        .collect();
    assert_eq!(cites, [53, 58]);
    let kinds = declaration_expect(&hibernates).kinds.as_ref().unwrap();
    let expected: Vec<_> = kinds["talk"]
        .stages
        .iter()
        .map(|stage| (stage.name.as_str(), stage.hibernate))
        .collect();
    assert_eq!(
        expected,
        [
            ("draft", Some(true)),
            ("code", Some(false)),
            ("review", None)
        ],
        "{name}"
    );
    assert_eq!(differences(name, &hibernates), Vec::<String>::new());
}
