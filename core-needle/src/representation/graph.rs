// -----------------------------------------------------------------------------
// Copyright (c) 2025 Hubert Liberacki <hliberacki@gmail.com>
//
// SPDX-License-Identifier: MIT
// -----------------------------------------------------------------------------

use crate::representation::{
    Dataset, Issue, IssueCode, Node, NodeId, NodeKind, Severity, VersionAccessor,
};
use std::collections::HashMap;
use std::fmt;

// Keeping those values precalculated only to save time while fetching
// and having those values precalculated. This might be a problem in
// production if JSON will be massive and it would need to be stored.
// But I assume it would be not (in the demo context).

#[derive(Debug)]
pub struct Graph {
    pub adjacency: HashMap<NodeId, Vec<NodeId>>,
    pub reverse: HashMap<NodeId, Vec<NodeId>>,

    pub nodes: HashMap<NodeId, Node>,
    pub kinds: HashMap<NodeKind, Vec<NodeId>>,

    pub issues: Vec<Issue>,
}

// Views to have the printers a bit better
pub struct GraphAdjView<'a>(&'a Graph);
pub struct GraphIssuesView<'a>(&'a Graph);
pub struct GraphKindsView<'a>(&'a Graph);

impl Graph {
    pub fn new(view: VersionAccessor<'_>) -> Self {
        let nodes = view.needs.clone();
        let (adjacency, mut issues) = Self::seed_adjacency(&view);

        let kinds = Self::seed_by_kind(&view);
        let reverse = Self::seed_reverse(&adjacency);

        // Validate graph consistency
        // TODO: This should maybe be outside of the constructor?

        issues.extend(Self::validate_consistency(&adjacency, &reverse));
        issues.extend(Self::validate_by_kind(&nodes, &kinds));
        issues.extend(Self::validate_dangling(&adjacency, &reverse, &nodes));

        Self {
            adjacency,
            reverse,
            nodes,
            issues,
            kinds,
        }
    }

    pub fn as_adj(&self) -> GraphAdjView<'_> {
        GraphAdjView(self)
    }
    pub fn as_issues(&self) -> GraphIssuesView<'_> {
        GraphIssuesView(self)
    }
    pub fn as_kinds(&self) -> GraphKindsView<'_> {
        GraphKindsView(self)
    }

    pub fn nodes_len(&self) -> usize {
        self.nodes.len()
    }
    pub fn edges_len(&self) -> usize {
        self.adjacency.values().map(|v| v.len()).sum()
    }
    pub fn of_kind(&self, k: crate::representation::NodeKind) -> &[crate::representation::NodeId] {
        self.kinds.get(&k).map(|v| v.as_slice()).unwrap_or(&[])
    }
    pub fn out(&self, id: &crate::representation::NodeId) -> &[crate::representation::NodeId] {
        self.adjacency.get(id).map(|v| v.as_slice()).unwrap_or(&[])
    }
    pub fn inc(&self, id: &crate::representation::NodeId) -> &[crate::representation::NodeId] {
        self.reverse.get(id).map(|v| v.as_slice()).unwrap_or(&[])
    }
    pub fn has_errors(&self) -> bool {
        self.issues.iter().any(|i| i.severity == Severity::Error)
    }

    fn seed_adjacency(view: &VersionAccessor<'_>) -> (HashMap<NodeId, Vec<NodeId>>, Vec<Issue>) {
        let mut adj: HashMap<NodeId, Vec<NodeId>> = HashMap::with_capacity(view.needs.len());
        let mut issues: Vec<Issue> = Vec::new();

        for (current_id, node) in view.needs.iter() {
            let mut visited = std::collections::HashSet::new();
            let mut linked = Vec::with_capacity(node.links.len());

            for linked_id in &node.links {
                if visited.insert(linked_id) {
                    linked.push(NodeId(linked_id.clone()));
                } else {
                    // Duplicated link
                    issues.push(Issue::warn(
                        IssueCode::DuplicateLink,
                        current_id.clone(),
                        format!("duplicate link {} -> {}", current_id, linked_id),
                    ));
                }
            }
            adj.insert(current_id.clone(), linked);
        }

        (adj, issues)
    }

    fn seed_reverse(adj: &HashMap<NodeId, Vec<NodeId>>) -> HashMap<NodeId, Vec<NodeId>> {
        let mut rev: HashMap<NodeId, Vec<NodeId>> = HashMap::new();

        for (current_id, linked) in adj {
            for tgt in linked {
                rev.entry(tgt.clone()).or_default().push(current_id.clone());
            }

            // validation - all nodes exists both ways
            rev.entry(current_id.clone()).or_default();
        }

        rev
    }

    fn seed_by_kind(view: &VersionAccessor<'_>) -> HashMap<NodeKind, Vec<NodeId>> {
        let mut by_kind: HashMap<NodeKind, Vec<NodeId>> = HashMap::new();

        for (id, node) in view.needs {
            by_kind
                .entry(NodeKind::from_str(node.kind.as_deref().unwrap()))
                .or_default()
                .push(id.clone());
        }

        by_kind
    }

    fn validate_consistency(
        adj: &HashMap<NodeId, Vec<NodeId>>,
        rev: &HashMap<NodeId, Vec<NodeId>>,
    ) -> Vec<Issue> {
        let mut issues = Vec::new();

        // Checking if there are the same connections both ways
        for current_id in adj.keys() {
            if !rev.contains_key(current_id) {
                issues.push(Issue::error(
                    IssueCode::BrokenLink,
                    current_id.clone(),
                    "node {current_id} is missing in reverse connection graph",
                ));
            }
        }
        for current_id in rev.keys() {
            if !adj.contains_key(current_id) {
                issues.push(Issue::error(
                    IssueCode::BrokenLink,
                    current_id.clone(),
                    "node {current_id} is missing in forward connection graph",
                ));
            }
        }

        for (source_id, outs) in adj {
            for linked_id in outs {
                match rev.get(linked_id) {
                    None => issues.push(Issue::error(
                        IssueCode::BrokenLink,
                        source_id.clone(),
                        format!(
                            "edge {} → {} not found in reverse connection",
                            source_id, linked_id
                        ),
                    )),
                    Some(instance) => {
                        let count = instance.iter().filter(|x| *x == source_id).count();
                        if count == 0 {
                            issues.push(Issue::error(
                                IssueCode::BrokenLink,
                                source_id.clone(),
                                format!("reverse connection [{}] missing {}", linked_id, source_id),
                            ));
                        } else if count > 1 {
                            issues.push(Issue::warn(
                                IssueCode::DuplicateLink,
                                linked_id.clone(),
                                format!(
                                    "reverse connection [{}] contains {} twice",
                                    linked_id, source_id
                                ),
                            ));
                        }
                    }
                }
            }
        }

        issues
    }

    fn validate_by_kind(
        by_id: &HashMap<NodeId, Node>,
        by_kind: &HashMap<NodeKind, Vec<NodeId>>,
    ) -> Vec<Issue> {
        let mut issues = Vec::new();

        // Check if there are no duplicated nodes between Node types
        for (id, node) in by_id {
            let kind = NodeKind::from_str(node.kind.as_deref().expect("Kind missing"));

            match by_kind.get(&kind) {
                None => issues.push(Issue::error(
                    IssueCode::BrokenLink,
                    id.clone(),
                    format!("node missing from by_kind[{}]", kind.as_str()),
                )),
                Some(bucket) => {
                    let count = bucket.iter().filter(|x| *x == id).count();
                    if count == 0 {
                        issues.push(Issue::error(
                            IssueCode::BrokenLink,
                            id.clone(),
                            "Node not found in the correct kind list",
                        ));
                    } else if count > 1 {
                        issues.push(Issue::warn(
                            IssueCode::DuplicateLink,
                            id.clone(),
                            "Node duplicated in kind list",
                        ));
                    }
                }
            }
        }

        issues
    }

    fn validate_dangling(
        adj: &HashMap<NodeId, Vec<NodeId>>,
        rev: &HashMap<NodeId, Vec<NodeId>>,
        nodes: &HashMap<NodeId, Node>,
    ) -> Vec<Issue> {
        let mut issues = Vec::new();
        for id in adj.keys() {
            // Some nodes most likely would never have links e.g (NodeKind::Person | NodeKind::Team)
            // just exclude them and continue
            if let Some(node) = nodes.get(id) {
                if matches!(
                    NodeKind::from_str(node.kind.as_deref().unwrap()),
                    NodeKind::Person | NodeKind::Team
                ) {
                    continue;
                }
            }

            let outs_empty = adj.get(id).map_or(true, |v| v.is_empty());
            let ins_empty = rev.get(id).map_or(true, |v| v.is_empty());
            if outs_empty && ins_empty {
                issues.push(Issue::suggest(
                    IssueCode::DandlingNode,
                    id.clone(),
                    "Node is dangling (empty forward and reverse links)",
                ));
            }
        }
        issues
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(formatter, "Graph with {} nodes:", self.nodes_len())?;
        for (id, node) in &self.nodes {
            writeln!(formatter, "  {id:?}: {:?}", node)?;
        }
        Ok(())
    }
}

impl fmt::Display for GraphAdjView<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(formatter, "Adjacency:")?;
        for (node, neighbors) in &self.0.adjacency {
            writeln!(formatter, "  {node:?} -> {neighbors:?}")?;
        }
        Ok(())
    }
}

impl fmt::Display for GraphIssuesView<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(formatter, "Issues ({}):", self.0.issues.len())?;
        for issue in &self.0.issues {
            writeln!(formatter, "  {:?}", issue)?;
        }
        Ok(())
    }
}

impl fmt::Display for GraphKindsView<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(formatter, "Kinds:")?;
        for (kind, ids) in &self.0.kinds {
            writeln!(formatter, "  {kind:?}: {:?}", ids)?;
        }
        Ok(())
    }
}

// Disclaimer tests are generated by AI (Claude Sonnet 4) - manually corrected because
// LLM makes a lot of problems generating rust ... sadly (or not ;))

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io::populate_from_str;
    use crate::representation::{Dataset, NodeId, VersionAccessor};

    // ---- Test fixtures ----------------------------------------------------

    const JSON_OK: &str = r#"
    {
      "current_version": "1.0",
      "versions": {
        "1.0": {
          "needs": {
            "REQ_1":  { "id":"REQ_1","type":"req","links":["SPEC_1"] },
            "SPEC_1": { "id":"SPEC_1","type":"spec","links":["TEST_1"] },
            "TEST_1": { "id":"TEST_1","type":"test","links":[] }
          }
        }
      }
    }"#;

    // duplicate link: REQ_1 -> SPEC_1 twice
    const JSON_DUP_EDGE: &str = r#"
    {
      "current_version": "1.0",
      "versions": {
        "1.0": {
          "needs": {
            "REQ_1":  { "id":"REQ_1","type":"req","links":["SPEC_1","SPEC_1"] },
            "SPEC_1": { "id":"SPEC_1","type":"spec","links":[] }
          }
        }
      }
    }"#;

    // broken link: REQ_1 -> SPEC_MISSING does not exist
    const JSON_BROKEN: &str = r#"
    {
      "current_version": "1.0",
      "versions": {
        "1.0": {
          "needs": {
            "REQ_1": { "id":"REQ_1","type":"req","links":["SPEC_MISSING"] }
          }
        }
      }
    }"#;

    // isolated node (no in/out)
    const JSON_ISOLATED: &str = r#"
    {
      "current_version": "1.0",
      "versions": {
        "1.0": {
          "needs": {
            "LONE": { "id":"LONE","type":"req","links":[] }
          }
        }
      }
    }"#;

    fn build_graph(json: &str) -> Graph {
        let ds: Dataset = populate_from_str(json).expect("parse json");
        let view: VersionAccessor<'_> = ds.access_current_version();
        Graph::new(view)
    }

    // ---- Tests ------------------------------------------------------------

    #[test]
    fn graph_builds_ok_and_indexes() {
        let g = build_graph(JSON_OK);

        // nodes & edges
        assert_eq!(g.nodes_len(), 3);
        assert_eq!(g.edges_len(), 2);

        // adjacency shape
        assert_eq!(g.out(&NodeId::from("REQ_1")), &[NodeId::from("SPEC_1")]);
        assert_eq!(g.out(&NodeId::from("SPEC_1")), &[NodeId::from("TEST_1")]);
        assert!(g.out(&NodeId::from("TEST_1")).is_empty());

        // reverse shape
        assert_eq!(g.inc(&NodeId::from("SPEC_1")), &[NodeId::from("REQ_1")]);
        assert_eq!(g.inc(&NodeId::from("TEST_1")), &[NodeId::from("SPEC_1")]);

        // by_kind buckets
        let reqs = g.of_kind(NodeKind::Req);
        assert_eq!(reqs, &[NodeId::from("REQ_1")]);

        // no issues
        assert!(!g.has_errors());
        assert!(g.issues.is_empty());
    }

    #[test]
    fn duplicate_outgoing_edge_emits_warning_and_is_deduped() {
        let g = build_graph(JSON_DUP_EDGE);

        // Only one edge remains after dedupe
        assert_eq!(g.out(&NodeId::from("REQ_1")), &[NodeId::from("SPEC_1")]);

        // A warning is recorded
        let mut saw_dup = false;
        for i in &g.issues {
            if i.code == IssueCode::DuplicateLink && matches!(i.severity, Severity::Warning) {
                saw_dup = true;
                break;
            }
        }
        assert!(saw_dup, "expected DuplicateLink warning");
    }

    #[test]
    fn broken_target_emits_error() {
        let g = build_graph(JSON_BROKEN);

        // out edge exists
        assert_eq!(
            g.out(&NodeId::from("REQ_1")),
            &[NodeId::from("SPEC_MISSING")]
        );

        // should have at least one BrokenLink error
        assert!(
            g.issues
                .iter()
                .any(|i| i.code == IssueCode::BrokenLink && matches!(i.severity, Severity::Error)),
            "expected BrokenLink error"
        );
        assert!(g.has_errors());
    }

    #[test]
    fn reverse_and_by_kind_are_consistent() {
        let g = build_graph(JSON_OK);

        for (u, outs) in &g.adjacency {
            for v in outs {
                let ins = g.reverse.get(v).expect("rev bucket for target");
                assert!(ins.iter().any(|x| x == u), "rev[{v}] missing {u}");
            }
        }

        // Each node should appear in exactly one matching by_kind bucket
        for (id, n) in &g.nodes {
            let bucket = g
                .kinds
                .get(&NodeKind::from_str(&n.kind.as_deref().unwrap()))
                .expect("bucket for kind");
            assert!(
                bucket.iter().any(|x| x == id),
                "node {id} missing from its kind bucket"
            );
        }
    }

    #[test]
    fn isolated_node_detected_as_suggestion() {
        let g = build_graph(JSON_ISOLATED);

        // no edges
        assert!(g.out(&NodeId::from("LONE")).is_empty());
        assert!(g.inc(&NodeId::from("LONE")).is_empty());

        // Depending on how you coded suggest_isolated(), this should surface.
        // If you haven't added IssueCode::IsolatedNode yet, temporarily skip or adjust this test.
        let saw_isolated = g.issues.iter().any(|i| {
            matches!(i.severity, Severity::Suggestion) && i.subject == NodeId::from("LONE")
        });
        assert!(saw_isolated, "expected suggestion for isolated node");
    }
}
