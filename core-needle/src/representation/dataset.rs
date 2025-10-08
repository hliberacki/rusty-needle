// -----------------------------------------------------------------------------
// Copyright (c) 2025 Hubert Liberacki <hliberacki@gmail.com>
//
// SPDX-License-Identifier: MIT
// -----------------------------------------------------------------------------

use super::node::Node;
use super::node_id::NodeId;
use serde::Deserialize;
use std::collections::HashMap;

pub struct VersionAccessor<'source> {
    pub version: &'source str,
    pub needs: &'source HashMap<NodeId, Node>,
}

#[derive(Debug, Deserialize)]
pub struct Dataset {
    pub current_version: Option<String>,
    pub versions: HashMap<String, VersionNode>,
}

impl Dataset {
    pub fn access_version<'a>(&'a self, version: &'a str) -> VersionAccessor<'a> {
        let v = &self.versions.get(version).expect("Version not found");
        VersionAccessor {
            version,
            needs: &v.needs,
        }
    }

    pub fn access_current_version<'a>(&'a self) -> VersionAccessor<'a> {
        let ver = self.current_version.as_deref().expect("Empty Optional");
        self.access_version(ver)
    }
}

#[derive(Debug, Deserialize)]
pub struct VersionNode {
    pub needs: HashMap<NodeId, Node>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dataset_creation() {
        let mut versions = HashMap::new();
        let mut needs: HashMap<NodeId, Node> = HashMap::new();
        let mut node = Node::default();
        node.id = Some(NodeId::new(String::from("test")));
        needs.insert(NodeId::new("node1"), node);
        versions.insert("1.0.0".to_string(), VersionNode { needs });

        let dataset = Dataset {
            current_version: Some("1.0.0".to_string()),
            versions,
        };

        assert_eq!(dataset.current_version.unwrap(), "1.0.0");
        assert!(dataset.versions.contains_key("1.0.0"));
        assert!(
            dataset.versions["1.0.0"]
                .needs
                .contains_key(&NodeId::new("node1"))
        );
    }

    #[test]
    fn test_empty_dataset() {
        let dataset = Dataset {
            current_version: None,
            versions: HashMap::new(),
        };

        assert!(dataset.current_version.is_none());
        assert!(dataset.versions.is_empty());
    }

    #[test]
    fn test_multiple_versions() {
        let mut versions = HashMap::new();
        versions.insert(
            "1.0.0".to_string(),
            VersionNode {
                needs: HashMap::new(),
            },
        );
        versions.insert(
            "2.0.0".to_string(),
            VersionNode {
                needs: HashMap::new(),
            },
        );

        let dataset = Dataset {
            current_version: Some("2.0.0".to_string()),
            versions,
        };

        assert_eq!(dataset.versions.len(), 2);
    }
}
