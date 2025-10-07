// -----------------------------------------------------------------------------
// Copyright (c) 2025 Hubert Liberacki <hliberacki@gmail.com>
//
// SPDX-License-Identifier: MIT
// -----------------------------------------------------------------------------

use std::collections::HashMap;
use serde::Deserialize;
use super::node::Node;
use super::node_id::NodeId;

#[derive(Debug, Deserialize)]
pub struct Dataset {
    pub latest_version: Option<String>,
    pub versions: HashMap<String, VersionNode>
}

#[derive(Debug, Deserialize)]
pub struct VersionNode {
    pub nodes: HashMap<String, Node>
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dataset_creation() {
        let mut versions = HashMap::new();
        let mut nodes = HashMap::new();
        let mut node = Node::default();
        node.id = Some(NodeId::new(String::from("test")));
        nodes.insert("node1".to_string(), node);
        versions.insert("1.0.0".to_string(), VersionNode { nodes });

        let dataset = Dataset {
            latest_version: Some("1.0.0".to_string()),
            versions,
        };

        assert_eq!(dataset.latest_version.unwrap(), "1.0.0");
        assert!(dataset.versions.contains_key("1.0.0"));
        assert!(dataset.versions["1.0.0"].nodes.contains_key("node1"));
    }

    #[test]
    fn test_empty_dataset() {
        let dataset = Dataset {
            latest_version: None,
            versions: HashMap::new(),
        };

        assert!(dataset.latest_version.is_none());
        assert!(dataset.versions.is_empty());
    }

    #[test]
    fn test_multiple_versions() {
        let mut versions = HashMap::new();
        versions.insert("1.0.0".to_string(), VersionNode { nodes: HashMap::new() });
        versions.insert("2.0.0".to_string(), VersionNode { nodes: HashMap::new() });

        let dataset = Dataset {
            latest_version: Some("2.0.0".to_string()),
            versions,
        };

        assert_eq!(dataset.versions.len(), 2);
    }
}
