// -----------------------------------------------------------------------------
// Copyright (c) 2025 Hubert Liberacki <hliberacki@gmail.com>
//
// SPDX-License-Identifier: MIT
// -----------------------------------------------------------------------------

pub mod dataset;
pub mod graph;
pub mod graph_data_traits;
pub mod issue;
pub mod node;
pub mod node_id;
pub mod node_kind;

pub use dataset::{Dataset, VersionAccessor, VersionNode};
pub use graph::Graph;
pub use graph_data_traits::Identifiable;
pub use issue::{Issue, IssueCode, Severity};
pub use node::Node;
pub use node_id::NodeId;
pub use node_kind::NodeKind;
