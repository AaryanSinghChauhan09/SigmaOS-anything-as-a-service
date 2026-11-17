//! SigmaOS-anything-as-a-service — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_anything_as_a_service_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-anything-as-a-service: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-anything-as-a-service: ok");
}
