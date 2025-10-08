// -----------------------------------------------------------------------------
// Copyright (c) 2025 Hubert Liberacki <hliberacki@gmail.com>
//
// SPDX-License-Identifier: MIT
// -----------------------------------------------------------------------------

use crate::policy::model::{Defaults, Rule};
use crate::representation::{Graph, Issue, IssueCode, NodeKind, Severity};
use serde::Deserialize;
use std::collections::{HashSet, VecDeque};

#[derive(Deserialize)]
struct Params {
    target_kinds: Vec<NodeKind>,
    #[serde(default)]
    min: usize,
    #[serde(default)]
    max_hops: Option<usize>,
}

pub fn run(g: &Graph, rule: &Rule, defaults: &Defaults) -> Vec<Issue> {
    let mut params: Params = serde_json::from_value(rule.params.clone()).unwrap_or(Params {
        target_kinds: vec![],
        min: 1,
        max_hops: None,
    });
    if params.max_hops.is_none() {
        params.max_hops = defaults.max_hops;
    }
    let hops = params.max_hops.unwrap_or(2);

    let sev = rule
        .severity
        .or(defaults.severity)
        .unwrap_or(Severity::Error);
    let code = rule.code.as_deref().unwrap_or("REACH_KIND");
    let msg = rule
        .message
        .as_deref()
        .unwrap_or("missing required reachable target");

    let targets: std::collections::HashSet<_> = params.target_kinds.into_iter().collect();

    let mut out = Vec::new();
    for kind in &rule.selector.kinds {
        for start in g.of_kind(*kind) {
            let mut q = VecDeque::from([(start, 0usize)]);
            let mut seen: HashSet<&crate::representation::NodeId> = HashSet::from([start]);
            let mut hits = 0usize;

            while let Some((id, d)) = q.pop_front() {
                if d > 0 {
                    if let Some(n) = g.nodes.get(id) {
                        if targets.contains(&NodeKind::from_str(n.kind.as_deref().unwrap())) {
                            hits += 1;
                        }
                    }
                }
                if d == hops {
                    continue;
                }
                for nxt in g.out(id) {
                    if seen.insert(nxt) {
                        q.push_back((nxt, d + 1));
                    }
                }
            }

            if hits < 1.max(params.min) {
                out.push(Issue {
                    severity: sev,
                    code: IssueCode::from_rule_code(code),
                    subject: start.clone(),
                    detail: msg.to_string(),
                });
            }
        }
    }
    out
}
