//! An effect, an atom with its arguments, and a decision, with the answers
//! it takes and what it shows (94, 97; C.4).

use toml::Table;

/// An effect, `{ do = "<atom>", args = { ... } }`, in a state's `entry` or
/// `exit` or a transition's `effects`: an atom its caller performs, with
/// its arguments (94, 98; C.4).
#[derive(Debug, Clone, PartialEq)]
pub struct Effect {
    /// Its `do`, the atom, by the name its reader binds, as written; which
    /// atoms there are is the reader's (95, 98).
    pub atom: String,
    /// Its `args`, as written; empty when left out (94, C.4).
    pub args: Table,
    /// Every key of the effect the format does not name, unread and as
    /// written, `doc` and `note` among them (56, C.4).
    pub sections: Table,
}

/// A state's `decision`, `{ kind, answers, shows }`: asked of a person
/// while its state is active (94, 97; C.4).
#[derive(Debug, Clone, PartialEq)]
pub struct Decision {
    /// Its `kind`, by name as written (C.4).
    pub kind: String,
    /// Its `answers`, the words a response gives, in the order written;
    /// none when left out (94, C.4).
    pub answers: Vec<String>,
    /// Its `shows`, what it shows a person, in the order written; none when
    /// left out (94, C.4).
    pub shows: Vec<String>,
    /// Every key of the decision the format does not name, unread and as
    /// written, `doc` and `note` among them (56, C.4).
    pub sections: Table,
}
