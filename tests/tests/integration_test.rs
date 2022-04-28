// To make integration tests, we create another folder called
// `tests`.
use tests;

// To run this specific integration test
// `cargo test --test integration_test` 
#[test]
fn greeting_contains() {
    let result = tests::greeting("Carol");
    // In this way we can add messages to the test results.
    assert!(
        result.contains("Carol"),
    "Greeting did not contain the name Carol. The received value was  `{}`",
    result );
}