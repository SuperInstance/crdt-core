//! # crdt-core
//!
//! Conflict-free replicated data types: G-Counter, PN-Counter, G-Set, OR-Set, LWW-Register.
//!
//! ## Modules
//! - `gcounter` — Grow-only counter
//! - `pncounter` — Positive-negative counter
//! - `gset` — Grow-only set
//! - `orset` — Observed-remove set
//! - `lww` — Last-writer-wins register

pub mod gcounter;
pub mod pncounter;
pub mod gset;
pub mod orset;
pub mod lww;

pub use gcounter::GCounter;
pub use pncounter::PNCounter;
pub use gset::GSet;
pub use orset::ORSet;
pub use lww::LWWRegister;
