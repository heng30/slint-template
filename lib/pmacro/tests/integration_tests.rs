//! Integration tests for the pmacro library.

use pmacro::SlintFromConvert;

/// Mock Slint types for testing
mod slint {
    use std::sync::Arc;

    pub type VecModel<T> = Vec<T>;
    pub type ModelRc<T> = Arc<VecModel<T>>;
}

use slint::ModelRc;

/// Test struct with basic field conversions
#[derive(Debug, Clone, PartialEq, Default)]
struct TestUIBasic {
    name: String,
    age: u32,
}

/// Test struct with vector field conversions
#[derive(Debug, Clone, PartialEq, Default)]
struct TestUIWithVectors {
    name: String,
    items: ModelRc<String>,
    numbers: ModelRc<i32>,
    empty_vec: ModelRc<u8>,
}

/// Test basic field conversion without vectors
#[derive(Debug, Clone, PartialEq, Default, SlintFromConvert)]
#[from("TestUIBasic")]
struct TestBasic {
    name: String,
    age: u32,
}

/// Test struct with vector field mappings
#[derive(Debug, Clone, PartialEq, Default, SlintFromConvert)]
#[from("TestUIWithVectors")]
#[vec_ui("empty_vec")]
struct TestWithVectors {
    name: String,
    #[vec(from = "items")]
    user_items: Vec<String>,
    #[vec(from = "numbers")]
    user_numbers: Vec<i32>,
}

#[test]
fn test_basic_conversion() {
    let original = TestBasic {
        name: "Alice".to_string(),
        age: 30,
    };

    let ui: TestUIBasic = original.clone().into();
    let converted_back: TestBasic = ui.into();

    assert_eq!(original, converted_back);
}

#[test]
fn test_vector_conversion() {
    let original = TestWithVectors {
        name: "Bob".to_string(),
        user_items: vec!["item1".to_string(), "item2".to_string()],
        user_numbers: vec![1, 2, 3],
    };

    let ui: TestUIWithVectors = original.clone().into();
    let converted_back: TestWithVectors = ui.into();

    assert_eq!(original, converted_back);
}

#[test]
fn test_empty_vector_conversion() {
    let original = TestWithVectors {
        name: "Charlie".to_string(),
        user_items: vec![],
        user_numbers: vec![],
    };

    let ui: TestUIWithVectors = original.clone().into();
    let converted_back: TestWithVectors = ui.into();

    assert_eq!(original, converted_back);
}

#[test]
fn test_ui_to_rust_conversion() {
    let ui = TestUIWithVectors {
        name: "Dave".to_string(),
        items: ModelRc::new(vec!["test".to_string()]),
        numbers: ModelRc::new(vec![42]),
        empty_vec: ModelRc::new(vec![]),
    };

    let rust: TestWithVectors = ui.into();
    assert_eq!(rust.name, "Dave");
    assert_eq!(rust.user_items, vec!["test".to_string()]);
    assert_eq!(rust.user_numbers, vec![42]);
}

#[test]
fn test_rust_to_ui_conversion() {
    let rust = TestWithVectors {
        name: "Eve".to_string(),
        user_items: vec!["a".to_string(), "b".to_string()],
        user_numbers: vec![10, 20],
    };

    let ui: TestUIWithVectors = rust.into();
    assert_eq!(ui.name, "Eve");
    assert_eq!(*ui.items, vec!["a".to_string(), "b".to_string()]);
    assert_eq!(*ui.numbers, vec![10, 20]);
    assert_eq!(*ui.empty_vec, vec![]);
}