// -----------------------------------------------------------------------------
// Copyright (c) 2025 Hubert Liberacki <hliberacki@gmail.com>
//
// SPDX-License-Identifier: MIT
// -----------------------------------------------------------------------------

use super::node_id::NodeId;
use strum_macros::IntoStaticStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, IntoStaticStr)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum IssueCode {
    BrokenLink,
    SelfLoop,
    DuplicateLink,
    MissingAuthor,
    MissingTeam,
    ImplMissingUrl,
    PendingImpl,
    ImplLacksStatus,
    DandlingNode,
    MergeByAuthor,
}

impl IssueCode {
    pub fn from_rule_code(code: &str) -> IssueCode {
        match code {
            "IMPL_MUST_LINK_SOMETHING" => IssueCode::ImplMissingUrl,
            "REQ_MUST_HAVE_DIRECT_TEST" => IssueCode::BrokenLink,
            "REQ_MUST_BE_TESTABLE_TRANSITIVELY" => IssueCode::BrokenLink,
            "IMPL_URL_REQUIRED" => IssueCode::ImplMissingUrl,
            "IMPL_STATUS_REQUIRED" => IssueCode::ImplLacksStatus,
            "PR_NOT_MERGED_BY_AUTHOR" => IssueCode::MergeByAuthor,
            _ => IssueCode::DuplicateLink,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize)]
pub enum Severity {
    Suggestion,
    Warning,
    Error,
}

impl Severity {
    pub fn to_str(self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Suggestion => "suggestion",
            Self::Warning => "warning",
        }
    }
}

#[derive(Debug)]
pub struct Issue {
    pub severity: Severity,
    pub code: IssueCode,
    pub subject: NodeId,
    pub detail: String,
}

impl Issue {
    pub fn warn(code: IssueCode, subject: NodeId, detail: impl Into<String>) -> Self {
        Self {
            severity: Severity::Warning,
            code,
            subject,
            detail: detail.into(),
        }
    }

    pub fn suggest(code: IssueCode, subject: NodeId, detail: impl Into<String>) -> Self {
        Self {
            severity: Severity::Suggestion,
            code,
            subject,
            detail: detail.into(),
        }
    }

    pub fn error(code: IssueCode, subject: NodeId, detail: impl Into<String>) -> Self {
        Self {
            severity: Severity::Error,
            code,
            subject,
            detail: detail.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn severity_to_str_conversion() {
        assert_eq!(Severity::Error.to_str(), "error");
        assert_eq!(Severity::Warning.to_str(), "warning");
        assert_eq!(Severity::Suggestion.to_str(), "suggestion");
    }

    #[test]
    fn issue_creation() {
        let issue = Issue {
            severity: Severity::Error,
            code: IssueCode::BrokenLink,
            subject: NodeId::new("1"),
            detail: "Test error".to_string(),
        };

        assert_eq!(issue.severity, Severity::Error);
        assert_eq!(issue.code, IssueCode::BrokenLink,);
        assert_eq!(issue.subject, NodeId::new("1"));
        assert_eq!(issue.detail, "Test error");
    }
}
