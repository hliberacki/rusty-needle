// -----------------------------------------------------------------------------
// Copyright (c) 2025 Hubert Liberacki <hliberacki@gmail.com>
//
// SPDX-License-Identifier: MIT
// -----------------------------------------------------------------------------

use serde::Deserialize;
use std::collections::HashMap;
use serde_json::json;

use super::node_id::NodeId;

#[derive(Debug, Deserialize, Default)]
pub struct Node {
    pub id: Option<NodeId>,

    #[serde(rename = "type")]
    pub kind: Option<String>,

    #[serde(default)]
    pub links: Vec<String>,

    #[serde(default)]
    pub links_back: Vec<String>,

    pub title: Option<String>,
    pub status: Option<String>,
    pub tags: Option<Vec<String>>,

    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
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
        assert_eq!(node.tags, Some(vec!["tag1".to_string(), "tag2".to_string()]));
        assert_eq!(node.extra["custom_field"], json!("custom_value"));
    }
}
