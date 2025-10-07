// -----------------------------------------------------------------------------
// Copyright (c) 2025 Hubert Liberacki <hliberacki@gmail.com>
//
// SPDX-License-Identifier: MIT
// -----------------------------------------------------------------------------

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NodeKind {
    Person,
    Team,
    Arch,
    Swarch,
    Req,
    Swreq,
    Spec,
    Test,
    Testsuite,
    Testrun,
    Impl,
    Need,
    Release,

    #[serde(other)]
    Unknown,
}

impl NodeKind {
    pub fn from_str(raw: &str) -> Self {
        match raw.trim().to_lowercase().as_str() {
            "person" => Self::Person,
            "team" => Self::Team,
            "arch" => Self::Arch,
            "swarch" => Self::Swarch,
            "req" => Self::Req,
            "swreq" => Self::Swreq,
            "spec" => Self::Spec,
            "test" => Self::Test,
            "testsuite" => Self::Testsuite,
            "testrun" => Self::Testrun,
            "impl" => Self::Impl,
            "need" => Self::Need,
            "release" => Self::Release,
            _ => Self::Unknown,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Person => "person",
            Self::Team => "team",
            Self::Arch => "arch",
            Self::Swarch => "swarch",
            Self::Req => "req",
            Self::Swreq => "swreq",
            Self::Spec => "spec",
            Self::Test => "test",
            Self::Testsuite => "testsuite",
            Self::Testrun => "testrun",
            Self::Impl => "impl",
            Self::Need => "need",
            Self::Release => "release",
            Self::Unknown => "unknown",
        }
    }
}

impl fmt::Display for NodeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str_valid_inputs() {
        assert_eq!(NodeKind::from_str("person"), NodeKind::Person);
        assert_eq!(NodeKind::from_str("team"), NodeKind::Team);
        assert_eq!(NodeKind::from_str("arch"), NodeKind::Arch);
        assert_eq!(NodeKind::from_str("swarch"), NodeKind::Swarch);
        assert_eq!(NodeKind::from_str("req"), NodeKind::Req);
    }

    #[test]
    fn test_from_str_case_insensitive() {
        assert_eq!(NodeKind::from_str("PERSON"), NodeKind::Person);
        assert_eq!(NodeKind::from_str("TeAm"), NodeKind::Team);
        assert_eq!(NodeKind::from_str(" test "), NodeKind::Test);
    }

    #[test]
    fn test_from_str_unknown() {
        assert_eq!(NodeKind::from_str("invalid"), NodeKind::Unknown);
        assert_eq!(NodeKind::from_str(""), NodeKind::Unknown);
    }

    #[test]
    fn test_as_str() {
        assert_eq!(NodeKind::Person.as_str(), "person");
        assert_eq!(NodeKind::Team.as_str(), "team");
        assert_eq!(NodeKind::Unknown.as_str(), "unknown");
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", NodeKind::Person), "person");
        assert_eq!(format!("{}", NodeKind::Team), "team");
        assert_eq!(format!("{}", NodeKind::Unknown), "unknown");
    }
}
