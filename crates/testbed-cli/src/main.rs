//! Binary crate of the testbed workspace — the artifact dist builds, archives, and wraps
//! in a Homebrew formula and a `.deb`.
//!
//! It prints its own version because that is the cheapest end-to-end check that the whole
//! chain agreed on one number: cocogitto computed it, `cargo set-version` wrote it, dist
//! built from it, and the tag points at it. A released binary reporting the wrong version
//! means one of those four disagreed.

fn main() {
    println!("{}", testbed_util::banner());
    println!("version: {}", env!("CARGO_PKG_VERSION"));
    println!("total: {}", testbed_util::total(&[1, 2, 3]));
}
