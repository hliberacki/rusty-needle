// -----------------------------------------------------------------------------
// Copyright (c) 2025 Hubert Liberacki <hliberacki@gmail.com>
//
// SPDX-License-Identifier: MIT
// -----------------------------------------------------------------------------

use super::super::representation::Dataset;
use serde_json;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

pub fn populate_from_file(path: &Path) -> std::result::Result<Dataset, Box<dyn std::error::Error>> {
    let raw = std::fs::read_to_string(path)?;
    Ok(populate_from_str(&raw)?)
}

pub fn populate_from_str(str: &str) -> std::result::Result<Dataset, serde_json::Error> {
    let json: Dataset = serde_json::from_str(str)?;
    Ok(json)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_JSON: &str = r#"
                                {
                                "current_version": "1.0",
                                "versions": {
                                    "1.0": {
                                    "needs": {
                                        "REQ_001": {
                                        "id": "REQ_001",
                                        "type": "req",
                                        "title": "System shall start within 3 seconds",
                                        "status": "draft",
                                        "tags": ["startup", "performance"],
                                        "links": ["SPEC_001"]
                                        },
                                        "SPEC_001": {
                                        "id": "SPEC_001",
                                        "type": "spec",
                                        "title": "Engine start time specification",
                                        "status": "approved",
                                        "links": ["TEST_001"]
                                        },
                                        "TEST_001": {
                                        "id": "TEST_001",
                                        "type": "test",
                                        "title": "Verify engine start time below 3 seconds",
                                        "status": "implemented",
                                        "links_back": ["SPEC_001"]
                                        }
                                    }
                                    }
                                }
                                }
                                "#;

    #[test]
    fn test_populate_from_str_valid_json() {
        let result = populate_from_str(TEST_JSON);
        assert!(result.is_ok());
    }

    #[test]
    fn test_populate_from_str_invalid_json() {
        let invalid_json = "{ invalid json }";
        let result = populate_from_str(invalid_json);
        assert!(result.is_err());
    }

    #[test]
    fn test_populate_from_file_nonexistent() {
        let path = PathBuf::from("nonexistent.json");
        let result = populate_from_file(&path);
        assert!(result.is_err());
    }

    #[test]
    fn test_populate_from_file_valid() {
        let temp_file = std::env::temp_dir().join("test.json");
        fs::write(&temp_file, TEST_JSON).unwrap();

        let result = populate_from_file(&temp_file);
        assert!(result.is_ok());

        fs::remove_file(temp_file).unwrap();
    }
}
