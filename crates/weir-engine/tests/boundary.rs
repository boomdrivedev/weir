//! The engine names no reader's object or atom: no file under its `src`
//! names an object the family's readers run, or the toy its tests run,
//! as a whole word in any case (98).

mod common;

use std::fs;
use std::path::Path;

use common::*;
use weir_engine::names;

/// The objects the family's readers run, which the engine never names.
const READERS_OBJECTS: [&str; 10] = [
    "place",
    "stage",
    "session",
    "agent",
    "intent",
    "elaboration",
    "bolt",
    "unit",
    "claim",
    "verdict",
];

/// Whether `text` holds `word` as a whole word: no letter, digit or `_`
/// on either side.
fn holds_word(text: &str, word: &str) -> bool {
    let part = |c: Option<char>| c.is_some_and(|c| c.is_alphanumeric() || c == '_');
    text.match_indices(word).any(|(at, _)| {
        let before = text[..at].chars().next_back();
        let after = text[at + word.len()..].chars().next();
        !part(before) && !part(after)
    })
}

/// Every file under `dir`, with its path.
fn files(dir: &Path, found: &mut Vec<(String, String)>) {
    for entry in fs::read_dir(dir).expect("a readable directory") {
        let path = entry.expect("a directory entry").path();
        if path.is_dir() {
            files(&path, found);
        } else {
            let text = fs::read_to_string(&path).expect("a text file");
            found.push((path.display().to_string(), text));
        }
    }
}

/// Each file under `src` naming one of `words`, with the word.
fn named<S: AsRef<str>>(words: &[S]) -> Vec<String> {
    let mut found = Vec::new();
    files(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("src"),
        &mut found,
    );
    assert!(!found.is_empty(), "no files under src");
    found
        .iter()
        .flat_map(|(path, text)| {
            let text = text.to_lowercase();
            words
                .iter()
                .map(|word| word.as_ref().to_lowercase())
                .filter(move |word| holds_word(&text, word))
                .map(move |word| format!("{path}: {word}"))
        })
        .collect()
}

#[test]
fn the_engine_names_no_readers_object() {
    assert_eq!(named(&READERS_OBJECTS), Vec::<String>::new());
}

#[test]
fn the_engine_names_none_of_its_toys_atoms() {
    let mut atoms = vec!["lamp".to_string(), "bulb".to_string()];
    for chart in [lamp(), bulb()] {
        let names = names(&chart);
        atoms.extend(names.evidence);
        atoms.extend(names.effects);
        atoms.extend(names.machines);
    }
    assert!(atoms.contains(&"lamp.switch".to_string()));
    assert!(atoms.contains(&"cool".to_string()));
    assert_eq!(named(&atoms), Vec::<String>::new());
}

#[test]
fn a_name_put_back_is_found_as_a_whole_word() {
    let found = |text: &str| -> Vec<&str> {
        READERS_OBJECTS
            .into_iter()
            .filter(|word| holds_word(text, word))
            .collect()
    };
    assert_eq!(found("let x = obj.bolt;"), vec!["bolt"]);
    assert_eq!(found("// a claim"), vec!["claim"]);
    assert_eq!(found("let bolted = 1;"), Vec::<&str>::new());
    assert_eq!(found("reunited"), Vec::<&str>::new());
    assert!(holds_word("a lamp.switch read", "lamp.switch"));
}
