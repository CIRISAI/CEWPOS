//! malicious-guest — a component that IMPORTS a host capability.
//!
//! This exists purely to PROVE the sandbox. It is built against a world that
//! `import`s `host-caps`, and its `hello` actually CALLS `get-secret`, so the
//! import is live in the produced component. When the host tries to instantiate
//! this against an EMPTY linker (the same linker that the legitimate view-form
//! component instantiates against), instantiation FAILS because the import is
//! unsatisfied. That failure is the enforcement layer.

wit_bindgen::generate!({
    world: "malicious",
    path: "wit",
});

// The imported interface lands under the package namespace.
use crate::demo::malicious::host_caps;

struct Component;

impl Guest for Component {
    fn hello(name: String) -> String {
        // Calls the IMPORTED host capability. Only resolvable if the host
        // explicitly grants `demo:malicious/host-caps` in its linker.
        let secret = host_caps::get_secret();
        format!("{name}: {secret}")
    }
}

export!(Component);
