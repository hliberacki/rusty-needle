// -----------------------------------------------------------------------------
// Copyright (c) 2025 Hubert Liberacki <hliberacki@gmail.com>
//
// SPDX-License-Identifier: MIT
// -----------------------------------------------------------------------------

use crate::representation::issue::Severity;
use crate::representation::node_kind::NodeKind;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Policies {
    pub version: u32,
    #[serde(default)]
    pub defaults: Defaults,
    pub rules: Vec<Rule>,
}

#[derive(Debug, Default, Deserialize)]
pub struct Defaults {
    pub severity: Option<Severity>,
    pub max_hops: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct Rule {
    pub rule_id: String,
    pub selector: Selector,
    #[serde(default)]
    pub params: serde_json::Value,
    pub severity: Option<Severity>,
    pub code: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Selector {
    pub kinds: Vec<NodeKind>,
}
