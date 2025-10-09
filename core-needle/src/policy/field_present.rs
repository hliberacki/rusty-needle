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
    field: String,
}

pub fn run(g: &Graph, rule: &Rule, defaults: &Defaults) -> Vec<Issue> {
    let params: Params =
        serde_json::from_value(rule.params.clone()).expect("field_present requires { field }");

    let sev = rule
        .severity
        .or(defaults.severity)
        .unwrap_or(Severity::Error);

    let code_str = rule.code.as_deref().unwrap_or(&rule.rule_id);
    let code = IssueCode::from_rule_code(code_str);

    let msg = rule.message.as_deref().unwrap_or("required field missing");

    let mut out = Vec::new();
    for kind in &rule.selector.kinds {
        for id in g.of_kind(*kind) {
            let n = &g.nodes[id];
            if !n.field_present(&params.field) {
                out.push(Issue {
                    severity: sev,
                    code,
                    subject: id.clone(),
                    detail: msg.to_string(),
                });
            }
        }
    }
    out
}
