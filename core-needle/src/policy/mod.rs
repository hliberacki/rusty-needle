// -----------------------------------------------------------------------------
// Copyright (c) 2025 Hubert Liberacki <hliberacki@gmail.com>
//
// SPDX-License-Identifier: MIT
// -----------------------------------------------------------------------------

pub mod evaluator;
pub mod field_present;
pub mod has_outgoing;
pub mod model;
pub mod reach_kind;
pub mod registry;

pub use evaluator::evaluate;
pub use model::{Defaults, Policies, Rule, Selector};
pub use registry::{Registry, RuleFn};
