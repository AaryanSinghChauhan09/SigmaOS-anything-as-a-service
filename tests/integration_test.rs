// Integration test for SigmaOS-anything-as-a-service
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use sigmaos_anything_as_a_service as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
