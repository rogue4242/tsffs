//! Tests that the derive macro can correctly parse an input struct

use simics_api::{ClassKind, Create, Module, OwnedMutConfObjectPtr};
use simics_api_macro::module;

#[macro_use]
extern crate simics_api_macro;

// Test that we can have an impl without deriving the module impl

#[module(class_name = "test_module")]
pub struct TestModule {}

impl Module for TestModule {
    fn init(obj: simics_api::OwnedMutConfObjectPtr) -> OwnedMutConfObjectPtr {
        obj
    }
}

// Test that we can derive the Module impl by itself

#[derive(Module)]
pub struct TestModule2 {}

// Test that we can derive and have an impl

#[module(derive, class_name = "test_module_3")]
pub struct TestModule3 {}

// Test that we can customize the description fields

#[module(
    derive,
    class_name = "test_module_4",
    description = "Test module 4",
    short_description = "TM4",
    class_kind = ClassKind::Session
)]
pub struct TestModule4 {}

#[test]
fn test() {}
