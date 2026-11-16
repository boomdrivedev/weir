//! The engine's crate (section 9): a statechart's types, its regions,
//! states, transitions, guards, effects and decisions, each field named
//! as C.4 names it (94). `weir-kinds` reads a statechart kind into them,
//! the kind's name and version the statechart's own (81, C.4).
//!
//! The crate reads no text, keeps no state and ticks nothing yet: its
//! types are plain data, and the tick comes with the change that brings
//! it (96 to 99; core.1).
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
mod refusal;
mod snapshot;
mod state;
mod statechart;
mod tick;

pub use decisions::{decisions, Standing};
pub use effect::{Decision, Effect};
pub use guard::{ChildrenTest, Guard, Test};
pub use refusal::Refusal;
pub use snapshot::{Related, Response, Snapshot};
pub use state::ObjectState;
pub use statechart::{Region, State, Statechart, Transition};
pub use tick::{start, tick, Applied, Fired, Phase, Planned, Tick};
