//! A refusal: a state or a template the engine cannot read, named by its
//! path (96, 97).

use std::fmt;

/// Why `start`, `tick` or `decisions` refused: the path it names and the
/// reason, displayed as `` `<path>`: <reason> ``.
#[derive(Debug, Clone, PartialEq)]
pub struct Refusal {
    /// The region path, or a state's path `<region path>.<state>`.
    pub path: String,
    /// What is wrong at that path.
    pub reason: String,
}

impl Refusal {
    pub(crate) fn new(path: impl Into<String>, reason: impl Into<String>) -> Refusal {
        Refusal {
            path: path.into(),
            reason: reason.into(),
        }
    }
}

impl fmt::Display for Refusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "`{}`: {}", self.path, self.reason)
    }
}

impl std::error::Error for Refusal {}
