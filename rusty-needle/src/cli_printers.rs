// -----------------------------------------------------------------------------
// Copyright (c) 2025 Hubert Liberacki <hliberacki@gmail.com>
//
// SPDX-License-Identifier: MIT
// -----------------------------------------------------------------------------

use colored::*;
use std::fmt;

use core_needle::representation::{Graph, Issue, Severity};

// This is just a helper util for printing the outputs to the terminal.
// Without formatting the output with colors it's hard to follow the output

// Disclaimer - a lot of that was generated using LLM Claude Sonnet 4
// It's just too much of boilerplate

pub enum ColorMode {
    Auto,
    Always,
    Never,
}

impl ColorMode {
    fn paint<'a>(&self, sev: Severity, s: &'a str) -> String {
        match (self, sev) {
            (ColorMode::Never, _) => s.to_string(),
            (_, Severity::Error) => s.red().bold().to_string(),
            (_, Severity::Warning) => s.yellow().bold().to_string(),
            (_, Severity::Suggestion) => s.bright_magenta().to_string(),
        }
    }
}

pub struct GraphNodesCli<'a> {
    graph: &'a Graph,
}
pub struct GraphAdjCli<'a> {
    graph: &'a Graph,
}
pub struct GraphKindsCli<'a> {
    graph: &'a Graph,
}
pub struct IssuesCli<'a> {
    issues: &'a [Issue],
    colors: ColorMode,
}

impl<'a> GraphNodesCli<'a> {
    pub fn new(graph: &'a Graph) -> Self {
        Self { graph }
    }
}
impl<'a> GraphAdjCli<'a> {
    pub fn new(graph: &'a Graph) -> Self {
        Self { graph }
    }
}
impl<'a> GraphKindsCli<'a> {
    pub fn new(graph: &'a Graph) -> Self {
        Self { graph }
    }
}
impl<'a> IssuesCli<'a> {
    pub fn new(issues: &'a [Issue], colors: ColorMode) -> Self {
        Self { issues, colors }
    }
}

impl fmt::Display for GraphNodesCli<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Nodes ({})", self.graph.nodes_len())?;
        for (id, node) in &self.graph.nodes {
            writeln!(f, "  {id:?}: {:?}", node)?;
        }
        Ok(())
    }
}

impl fmt::Display for GraphAdjCli<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Adjacency")?;
        for (node, neighbors) in &self.graph.adjacency {
            writeln!(f, "  {node:?} -> {neighbors:?}")?;
        }
        Ok(())
    }
}

impl fmt::Display for GraphKindsCli<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Kinds ({})", self.graph.kinds.len())?;
        for (kind, ids) in &self.graph.kinds {
            writeln!(f, "  {kind:?}: {:?}", ids)?;
        }
        Ok(())
    }
}

impl fmt::Display for IssuesCli<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Issues ({})", self.issues.len())?;
        for issue in self.issues {
            let sev_tag = format!("[{}]", issue.severity);
            let sev_colored = self.colors.paint(issue.severity, &sev_tag);
            let code = format!("[{}]", issue.code.to_str());
            writeln!(
                f,
                "  {} {} - {:?}, detail: {}",
                sev_colored, code, issue.subject, issue.detail
            )?;
        }
        Ok(())
    }
}

pub fn nodes<'a>(g: &'a Graph) -> GraphNodesCli<'a> {
    GraphNodesCli::new(g)
}
pub fn adj<'a>(g: &'a Graph) -> GraphAdjCli<'a> {
    GraphAdjCli::new(g)
}
pub fn kinds<'a>(g: &'a Graph) -> GraphKindsCli<'a> {
    GraphKindsCli::new(g)
}
pub fn issues<'a>(g: &'a Graph, colors: ColorMode) -> IssuesCli<'a> {
    IssuesCli::new(&g.issues, colors)
}
pub fn issues_from<'a>(items: &'a [Issue], colors: ColorMode) -> IssuesCli<'a> {
    IssuesCli::new(items, colors)
}
