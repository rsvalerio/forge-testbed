//! Leaf crate of the testbed workspace.
//!
//! The code here is deliberately trivial. What is under test is the CI that surrounds it:
//! `rust-ci` must find something to format, lint, build and run, and `publish-crates` must
//! find a crate with no internal dependencies to publish first.

/// Returns the greeting the CLI prints.
///
/// Exists so `cargo test` has a real assertion to run — a workspace whose test job passes
/// because there are no tests proves nothing about the test job.
#[must_use]
pub fn greeting() -> &'static str {
    "forge testbed"
}

/// Adds two numbers, saturating rather than wrapping on overflow.
///
/// Saturating is the point: `a + b` would trip `clippy::arithmetic_side_effects` under a
/// strict lint set, so this doubles as evidence that the shared clippy config is actually
/// being applied rather than silently skipped.
#[must_use]
pub fn add(a: i64, b: i64) -> i64 {
    a.saturating_add(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_is_stable() {
        assert_eq!(greeting(), "forge testbed");
    }

    #[test]
    fn add_saturates_instead_of_overflowing() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(i64::MAX, 1), i64::MAX);
    }
}
