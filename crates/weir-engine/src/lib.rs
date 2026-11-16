//! The engine's crate (section 9): a statechart's types, its regions,
//! states, transitions, guards, effects and decisions, each field named
//! as C.4 names it (94), which `weir-kinds` reads a statechart kind into,
//! the kind's name and version the statechart's own (81, C.4); and the
//! engine over them (96 to 99):
//!
//! - [`tick`], one pass over one object's state and one snapshot, giving
//!   the transitions that fire, the effects each plans and the next state
//!   (96, 97, 99);
//! - [`start`], an object's first state (96, 97);
//! - [`decisions`], the decisions standing in a state (97);
//! - [`names`], the evidence, effects and templates a statechart reads
//!   and runs (98).
//!
//! Each answers from its inputs alone. The crate holds no store, reads no
//! clock, serves no evidence of its own and performs nothing: the snapshot
//! is its caller's reading, and the effects planned, the decisions'
//! numbers and the responses' addresses are its caller's (69, 98; core.1,
//! core.55). The engine was moved from the original flywheel-engine
//! crate, agentplot/flywheel-next at f2ce8e3, onto these types.
//!
//! The values a statechart holds as written are [`toml`]'s, re-exported
//! so that a caller names the same version.

#![deny(missing_docs)]

/// The TOML crate whose values a statechart holds as written, so that a
/// caller reads them with the same version.
pub use toml;

mod decisions;
mod effect;
mod eval;
mod guard;
mod names;
mod refusal;
mod snapshot;
mod state;
mod statechart;
mod tick;

pub use decisions::{decisions, Standing};
pub use effect::{Decision, Effect};
pub use guard::{ChildrenTest, Guard, Test};
pub use names::{names, Names};
pub use refusal::Refusal;
pub use snapshot::{Related, Response, Snapshot};
pub use state::ObjectState;
pub use statechart::{Region, State, Statechart, Transition};
pub use tick::{start, tick, Applied, Fired, Phase, Planned, Tick};
