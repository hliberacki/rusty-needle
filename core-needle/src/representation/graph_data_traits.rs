// -----------------------------------------------------------------------------
// Copyright (c) 2025 Hubert Liberacki <hliberacki@gmail.com>
//
// SPDX-License-Identifier: MIT
// -----------------------------------------------------------------------------

pub trait Identifiable {
    fn id(&self) -> &str;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestIdentifiable {
        id: String,
    }

    impl Identifiable for TestIdentifiable {
        fn id(&self) -> &str {
            &self.id
        }
    }

    #[test]
    fn test_identifiable_returns_correct_id() {
        let test_obj = TestIdentifiable {
            id: "test_id".to_string(),
        };
        assert_eq!(test_obj.id(), "test_id");
    }

    #[test]
    fn test_identifiable_with_empty_id() {
        let test_obj = TestIdentifiable { id: "".to_string() };
        assert_eq!(test_obj.id(), "");
    }

    #[test]
    fn test_identifiable_with_special_chars() {
        let test_obj = TestIdentifiable {
            id: "test@123#".to_string(),
        };
        assert_eq!(test_obj.id(), "test@123#");
    }
}
