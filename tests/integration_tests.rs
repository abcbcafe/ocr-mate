// Main integration test file
// This file includes all integration test modules

mod common;
mod integration {
    mod config_persistence;
    mod document_loading;
    mod export_pipeline;
    mod ocr_workflow;
}

// Add a basic sanity test to ensure the test infrastructure works
#[test]
fn test_integration_tests_setup() {
    assert!(true, "Integration tests are properly configured");
}
