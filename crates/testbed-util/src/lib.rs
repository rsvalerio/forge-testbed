//! Middle crate of the testbed workspace.
//!
//! Its only real job is to depend on [`testbed_core`], so that `cargo publish --workspace`
//! has an ordering to get right: core must reach the registry before this crate can.

/// Formats the core greeting for display.
#[must_use]
pub fn banner() -> String {
    format!("== {} ==", testbed_core::greeting())
}

/// Sums a slice via [`testbed_core::add`], so the dependency edge is exercised at runtime
/// and not merely declared in the manifest.
#[must_use]
pub fn total(values: &[i64]) -> i64 {
    values.iter().copied().fold(0, testbed_core::add)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn banner_wraps_the_core_greeting() {
        assert_eq!(banner(), "== forge testbed ==");
    }

    #[test]
    fn total_sums_through_core() {
        assert_eq!(total(&[1, 2, 3]), 6);
        assert_eq!(total(&[]), 0);
    }
}
