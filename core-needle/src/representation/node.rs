// -----------------------------------------------------------------------------
// Copyright (c) 2025 Hubert Liberacki <hliberacki@gmail.com>
//
// SPDX-License-Identifier: MIT
// -----------------------------------------------------------------------------

use super::node_id::NodeId;
use serde::Deserialize;
use serde_json::Value;
use serde_json::json;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Node {
    pub id: Option<NodeId>,

    #[serde(rename = "type")]
    pub kind: Option<String>,

    #[serde(default)]
    pub links: Vec<String>,

    #[serde(default)]
    pub links_back: Vec<String>,

    pub title: Option<String>,

    #[serde(default)]
    pub status: Option<String>,

    #[serde(default)]
    pub url: Option<String>,

    pub tags: Option<Vec<String>>,

    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl Node {
    // This boilerplate is LLM generate but the whole point is to match policy rule
    // with the specified field - so that we can check for example is the field is
    // defined in a given Node
    pub fn field_present(&self, key: &str) -> bool {
        match key {
            "id" => self.id.is_some(),
            "type" | "kind" => self
                .kind
                .as_deref()
                .map(|s| !s.trim().is_empty())
                .unwrap_or(false),
            "title" => self
                .title
                .as_deref()
                .map(|s| !s.trim().is_empty())
                .unwrap_or(false),
            "status" => self
                .status
                .as_deref()
                .map(|s| !s.trim().is_empty())
                .unwrap_or(false),
            "url" => self
                .url
                .as_deref()
                .map(|s| !s.trim().is_empty())
                .unwrap_or(false),
            "tags" => self.tags.as_ref().map(|v| !v.is_empty()).unwrap_or(false),
            "links" => !self.links.is_empty(),
            "links_back" => !self.links_back.is_empty(),
            other => self.extra.get(other).map_or(false, json_value_present),
        }
    }
}

fn json_value_present(v: &Value) -> bool {
    match v {
        Value::Null => false,
        Value::String(s) => !s.trim().is_empty(),
        Value::Array(a) => !a.is_empty(),
        Value::Object(o) => !o.is_empty(),
        Value::Bool(_) | Value::Number(_) => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_default() {
        let node = Node::default();
        assert!(node.id.is_none());
        assert!(node.kind.is_none());
        assert!(node.links.is_empty());
        assert!(node.links_back.is_empty());
        assert!(node.title.is_none());
        assert!(node.status.is_none());
        assert!(node.tags.is_none());
        assert!(node.extra.is_empty());
    }

    #[test]
    fn test_node_deserialization() {
        let json = r#"{
            "id": "test-id",
            "type": "note",
            "links": ["link1", "link2"],
            "links_back": ["back1"],
            "title": "Test Title",
            "status": "active",
            "tags": ["tag1", "tag2"],
            "custom_field": "custom_value"
        }"#;

        let node: Node = serde_json::from_str(json).unwrap();

        assert_eq!(node.id, Some(NodeId::new("test-id".to_string())));
        assert_eq!(node.kind, Some("note".to_string()));
        assert_eq!(node.links, vec!["link1", "link2"]);
        assert_eq!(node.links_back, vec!["back1"]);
        assert_eq!(node.title, Some("Test Title".to_string()));
        assert_eq!(node.status, Some("active".to_string()));
        assert_eq!(
            node.tags,
            Some(vec!["tag1".to_string(), "tag2".to_string()])
        );
        assert_eq!(node.extra["custom_field"], json!("custom_value"));
    }
}
