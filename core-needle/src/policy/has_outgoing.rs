// -----------------------------------------------------------------------------
// Copyright (c) 2025 Hubert Liberacki <hliberacki@gmail.com>
//
// SPDX-License-Identifier: MIT
// -----------------------------------------------------------------------------

use crate::policy::model::{Defaults, Rule};
use crate::representation::{Graph, Issue, IssueCode, Severity};
use serde::Deserialize;

#[derive(Deserialize)]
struct Params {
    min: usize,
}

pub fn run(g: &Graph, rule: &Rule, defaults: &Defaults) -> Vec<Issue> {
    let params: Params = serde_json::from_value(rule.params.clone()).unwrap_or(Params { min: 1 });

    let sev = rule
        .severity
        .or(defaults.severity)
        .unwrap_or(Severity::Error);
    let code = rule.code.as_deref().unwrap_or("HAS_OUTGOING");
    let msg = rule
        .message
        .as_deref()
        .unwrap_or("missing required forward links");

    let mut out = Vec::new();
    for kind in &rule.selector.kinds {
        for id in g.of_kind(*kind) {
            if g.out(id).len() < params.min {
                out.push(Issue {
                    severity: sev,
                    code: IssueCode::from_rule_code(code),
                    subject: id.clone(),
                    detail: msg.to_string(),
                });
            }
        }
    }
    out
}
