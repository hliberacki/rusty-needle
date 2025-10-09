// -----------------------------------------------------------------------------
// Copyright (c) 2025 Hubert Liberacki <hliberacki@gmail.com>
//
// SPDX-License-Identifier: MIT
// -----------------------------------------------------------------------------

use super::node_id::NodeId;
use std::fmt;
use strum_macros::IntoStaticStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, IntoStaticStr, serde::Serialize)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum IssueCode {
    ImplNoLinks,
    ReqMissingDirectTest,
    ImplMissingUrl,
    ImplLacksStatus,
    PendingImpl,
    MergeByAuthor,
    BrokenLink,
    DanglingNode,
    DuplicateLink,
    SelfLoop,
    MissingAuthor,
    MissingTeam,
    DandlingNode,
    Unknown,
}

impl IssueCode {
    pub fn from_rule_code<S: AsRef<str>>(code: S) -> IssueCode {
        let c = code.as_ref().to_ascii_uppercase();

        match c.as_str() {
            "IMPL_NO_LINKS" => IssueCode::ImplNoLinks,
            "REQ_MISSING_DIRECT_TEST" => IssueCode::ReqMissingDirectTest,
            "IMPL_MISSING_URL" => IssueCode::ImplMissingUrl,
            "IMPL_LACKS_STATUS" => IssueCode::ImplLacksStatus,
            "MERGE_BY_AUTHOR" => IssueCode::MergeByAuthor,

            "BROKEN_LINK" => IssueCode::BrokenLink,
            "DUPLICATE_LINK" => IssueCode::DuplicateLink,
            "SELF_LOOP" => IssueCode::SelfLoop,
            "MISSING_AUTHOR" => IssueCode::MissingAuthor,
            "MISSING_TEAM" => IssueCode::MissingTeam,
            "PENDING_IMPL" => IssueCode::PendingImpl,
            "DANGLING_NODE" => IssueCode::DanglingNode,
            "DANDLING_NODE" => IssueCode::DandlingNode,

            "IMPL_MUST_LINK_SOMETHING" => IssueCode::ImplNoLinks,
            "REQ_MUST_HAVE_DIRECT_TEST" => IssueCode::ReqMissingDirectTest,
            "REQ_MUST_BE_TESTABLE_TRANSITIVELY" => IssueCode::ReqMissingDirectTest,
            "IMPL_URL_REQUIRED" => IssueCode::ImplMissingUrl,
            "IMPL_STATUS_REQUIRED" => IssueCode::ImplLacksStatus,
            "PR_NOT_MERGED_BY_AUTHOR" => IssueCode::MergeByAuthor,

            _ => IssueCode::Unknown,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            IssueCode::ImplNoLinks => "impl_no_links",
            IssueCode::ReqMissingDirectTest => "req_missing_direct_test",
            IssueCode::ImplMissingUrl => "impl_missing_url",
            IssueCode::ImplLacksStatus => "impl_lacks_status",
            IssueCode::PendingImpl => "pending_impl",
            IssueCode::MergeByAuthor => "merge_by_author",
            IssueCode::BrokenLink => "broken_link",
            IssueCode::DanglingNode => "dangling_node",
            IssueCode::DuplicateLink => "duplicate_link",
            IssueCode::SelfLoop => "self_loop",
            IssueCode::MissingAuthor => "missing_author",
            IssueCode::MissingTeam => "missing_team",
            IssueCode::DandlingNode => "dandling_node",
            IssueCode::Unknown => "unknown",
        }
    }
}

impl fmt::Display for IssueCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.to_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
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

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

#[derive(Debug, serde::Serialize)]
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
