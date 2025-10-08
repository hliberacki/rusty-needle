// -----------------------------------------------------------------------------
// Copyright (c) 2025 Hubert Liberacki <hliberacki@gmail.com>
//
// SPDX-License-Identifier: MIT
// -----------------------------------------------------------------------------

pub mod loader;

pub use loader::{
    load_graph_from_file, load_policy_from_file, load_policy_from_str, populate_from_file,
    populate_from_str,
};
