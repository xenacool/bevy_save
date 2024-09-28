#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::module_inception)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::redundant_closure_for_method_calls)]
#![allow(clippy::return_self_not_must_use)]
#![allow(clippy::too_many_lines)]
#![doc = include_str!("../README.md")]

pub use crate::{
    app::*,
    applier::*,
    backend::*,
    builder::*,
    clone::*,
    error::*,
    format::*,
    middleware::*,
    pipeline::*,
    plugins::*,
    registry::*,
    rollbacks::*,
    serde::*,
    snapshot::*,
    world::*,
};

mod app;
mod applier;
mod backend;
mod builder;
mod clone;
mod error;
mod format;
mod middleware;
mod pipeline;
mod plugins;
mod registry;
mod rollbacks;
mod serde;
mod snapshot;
mod world;

/// Prelude: convenient import for all the user-facing APIs provided by the crate
pub mod prelude {
    pub use crate::{
        app::*,
        applier::*,
        backend::*,
        builder::*,
        clone::*,
        format::*,
        middleware::*,
        pipeline::*,
        plugins::*,
        registry::*,
        rollbacks::*,
        serde::*,
        snapshot::*,
        world::*,
    };
}
