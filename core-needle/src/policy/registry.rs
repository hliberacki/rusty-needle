// -----------------------------------------------------------------------------
// Copyright (c) 2025 Hubert Liberacki <hliberacki@gmail.com>
//
// SPDX-License-Identifier: MIT
// -----------------------------------------------------------------------------

use crate::policy::model::{Defaults, Policies, Rule, Selector};
use crate::representation::graph::Graph;
use crate::representation::{Issue, IssueCode, Severity};
use crate::representation::{NodeId, NodeKind};
use std::collections::HashMap;
pub type RuleFn = fn(&Graph, &Rule, &Defaults) -> Vec<Issue>;

pub struct Registry {
    rules: HashMap<&'static str, RuleFn>,
}

impl Registry {
    pub fn builtins() -> Self {
        let mut r = Self {
            rules: HashMap::new(),
        };
        r.rules
            .insert("has_outgoing", crate::policy::has_outgoing::run);
        r.rules.insert("reach_kind", crate::policy::reach_kind::run);
        r.rules
            .insert("field_present", crate::policy::field_present::run);
        r
    }
    pub fn get(&self, id: &str) -> Option<&RuleFn> {
        self.rules.get(id)
    }
}
