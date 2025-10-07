// -----------------------------------------------------------------------------
// Copyright (c) 2025 Hubert Liberacki <hliberacki@gmail.com>
//
// SPDX-License-Identifier: MIT
// -----------------------------------------------------------------------------

use serde::Deserialize;
use serde::Serialize;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Deserialize, Serialize)]
pub struct NodeId(pub String);

impl NodeId {
    pub fn new<S: Into<String>>(s: S) -> Self {
        Self(s.into())
    }
}

impl AsRef<str> for NodeId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

// Convenient conversions
impl From<&str> for NodeId {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for NodeId {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_id_creation() {
        let id = NodeId::new("test");
        assert_eq!(id.0, "test");
    }

    #[test]
    fn test_node_id_display() {
        let id = NodeId::new("test");
        assert_eq!(format!("{}", id), "test");
    }

    #[test]
    fn test_node_id_from_str() {
        let id: NodeId = "test".into();
        assert_eq!(id.0, "test");
    }

    #[test]
    fn test_node_id_from_string() {
        let id: NodeId = String::from("test").into();
        assert_eq!(id.0, "test");
    }

    #[test]
    fn test_node_id_as_ref() {
        let id = NodeId::new("test");
        assert_eq!(id.as_ref(), "test");
    }
}
