//! The facts a signal carries, the people list they are joined from, and
//! the keeper's policy (A.16, A.17; C.9), read once for every host
//! (core.54).
//!
//! The crate holds a fact's names, values and sorts, in flight as JSON and
//! at rest as `fact` fields (117, 118; C.9); the people list, read from a
//! host's settings, and the join of attendees and speakers from it (119,
//! 121); a policy, read and checked whole (122, 123); a rule's match over
//! facts and a capture's age (123); the policy read for a signal and for a
//! capture as it lands (124, 125); and a rule's item's name (128, 129).
//!
//! Every function answers from its arguments alone: the time and the time
//! zone are its caller's, so the crate reads no clock and keeps no state,
//! and reading a policy changes nothing (core.1, core.56).
#![deny(missing_docs)]

mod fact;
mod people;
mod policy;
mod reading;
