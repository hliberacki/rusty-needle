// -----------------------------------------------------------------------------
// Copyright (c) 2025 Hubert Liberacki <hliberacki@gmail.com>
//
// SPDX-License-Identifier: MIT
// -----------------------------------------------------------------------------

use crate::policy::model::{Defaults, Rule};
use crate::representation::Graph;
use crate::representation::{Issue, IssueCode, Severity};
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
    let code = rule.code.as_deref().unwrap_or("FIELD_PRESENT");
    let msg = rule.message.as_deref().unwrap_or("required field missing");

    let mut out = Vec::new();
    for kind in &rule.selector.kinds {
        for id in g.of_kind(*kind) {
            let n = &g.nodes[id];
            let present = match params.field.as_str() {
                "url" => n.url.is_some(),
                "status" => n.status.is_some(),
                other => n.extra.get(other).is_some(),
            };
            if !present {
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
