///! Contains definition for a tests which should be conditionally run
use crate::testable::{TestResult, Testable};

// type aliases for test function signature
type TestFn = dyn Fn() -> TestResult;
// type alias for function signature for function which checks if a test can be run or not
type CheckFn = dyn Fn() -> bool;

/// Basic Template structure for tests which need to be run conditionally
pub struct ConditionalTest {
    /// name of the test
    name: String,
    /// actual test function
    test_fn: Box<TestFn>,
    /// function to check if a test can be run or not
    check_fn: Box<CheckFn>,
}

impl ConditionalTest {
    /// Create a new condition test
    pub fn new(name: &str, check_fn: Box<CheckFn>, test_fn: Box<TestFn>) -> Self {
        ConditionalTest {
            name: name.to_string(),
            check_fn,
            test_fn,
        }
    }
}

impl Testable for ConditionalTest {
    fn get_name(&self) -> String {
        self.name.clone()
    }
    fn can_run(&self) -> bool {
        (self.check_fn)()
    }
    fn run(&self) -> TestResult {
        (self.test_fn)()
    }
}
