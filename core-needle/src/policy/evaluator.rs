// -----------------------------------------------------------------------------
// Copyright (c) 2025 Hubert Liberacki <hliberacki@gmail.com>
//
// SPDX-License-Identifier: MIT
// -----------------------------------------------------------------------------

use crate::policy::model::Policies;
use crate::policy::registry::Registry;
use crate::representation::Issue;
use crate::representation::graph::Graph;

pub fn evaluate(g: &Graph, p: &Policies) -> Vec<Issue> {
    let reg = Registry::builtins();
    let mut out = Vec::new();
    for rule in &p.rules {
        if let Some(run) = reg.get(&rule.rule_id) {
            out.extend(run(g, rule, &p.defaults));
        } else {
            // TODO handle unknown rule - for now just ignore it.
        }
    }
    out
}
