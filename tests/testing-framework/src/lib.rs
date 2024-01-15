//! Testing infrastructure
//!
//! This crate has a few entry points depending on what you want to do:
//! * `RuntimeTest` - bootstraps and runs a single runtime test
//! * `TestEnvironment` - bootstraps a test environment which can be used by more than just runtime tests

mod cloud;
mod io;
mod manifest_template;
mod services;
mod spin;
mod utils;
mod configuration;
mod app_item;
mod cloud_api;
mod test_environment;

pub use manifest_template::ManifestTemplate;
pub use services::ServicesConfig;
pub use spin::{Request, Spin, SpinMode};
pub use cloud::Cloud;
pub use test_environment::{TestEnvironment, TestEnvironmentConfig};

#[derive(Debug, Clone, Copy)]
/// What to do when a test errors
pub enum OnTestError {
    /// Panic
    Panic,
    /// Log the error to stderr
    Log,
}

/// A runtime which can be tested
pub trait Runtime {
    /// Return an error if one has occurred
    fn error(&mut self) -> anyhow::Result<()>;
}

/// A test which can be run against a runtime
pub trait Test {
    /// The runtime the test is run against
    type Runtime: Runtime;
    /// The type of error the test can return when the test is in a failure state
    ///
    /// This type is used when the test is actually run but it fails as opposed to the
    /// error state where the test cannot be run at all.
    type Failure;

    /// Run the test against the runtime
    fn test(self, env: &mut TestEnvironment<Self::Runtime>) -> TestResult<Self::Failure>;
}

// impl<F, E> Test for F
// where
//     F: FnOnce(&mut TestEnvironment<Spin>) -> TestResult<E> + 'static,
// {
//     type Runtime = Spin;
//     type Failure = E;

//     fn test(self, env: &mut TestEnvironment<Self::Runtime>) -> TestResult<Self::Failure> {
//         self(env)
//     }
// }

impl<X, Y> Test for X
where
    X: FnOnce(&mut TestEnvironment<Cloud>) -> TestResult<Y> + 'static,
{
    type Runtime = Cloud;
    type Failure = Y;

    fn test(self, env: &mut TestEnvironment<Self::Runtime>) -> TestResult<Self::Failure> {
        self(env)
    }
}

/// The result of running a test.
///
/// The result has three states:
/// * `Ok(())` - the test ran and passed
/// * `Err(TestError::Failure(_))` - the test ran and failed
/// * `Err(TestError::Fatal(_))` - the test did not run because of an error
pub type TestResult<E> = Result<(), TestError<E>>;

/// An error in a test.
///
/// This type is generic over the `Failure` type (i.e., the type that is returned when the test
/// is actually run and fails).
#[derive(Debug)]
pub enum TestError<E> {
    /// The test was run but failed.
    Failure(E),
    /// The test did not run because of an error.
    Fatal(anyhow::Error),
}

impl<E> From<anyhow::Error> for TestError<E> {
    fn from(e: anyhow::Error) -> Self {
        TestError::Fatal(e)
    }
}

impl<E: std::fmt::Display + std::fmt::Debug> std::error::Error for TestError<E> {}

impl<E: std::fmt::Display> std::fmt::Display for TestError<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TestError::Failure(e) => {
                write!(f, "{e}")?;
                Ok(())
            }
            TestError::Fatal(e) => write!(f, "Test failed to run: {}", e),
        }
    }
}
