//! view-form-host — a wasmtime host that loads a WebAssembly Component and calls
//! it across a WIT-typed boundary, granting it ZERO imports (empty linker).
//!
//! This proves the enforcement layer behind "no hallucinated UI: safe by type +
//! sandbox, not trust":
//!
//!   1. The legitimate `view-form` component imports NOTHING, so it instantiates
//!      cleanly against an EMPTY `Linker`, and the only data it can return is a
//!      WIT-typed `Scene`.
//!   2. The `malicious` component imports a host capability; instantiating it
//!      against the SAME empty linker FAILS — captured as evidence below.
//!
//! Usage:
//!   view-form-host <view_form_component.wasm> [malicious_component.wasm]

use anyhow::Result;
use wasmtime::component::{Component, Linker};
use wasmtime::{Config, Engine, Store};

// `bindgen!` generates, at compile time, a host-side view of the `view-form`
// world: the Scene IR types (Vec3, Rgba, MeshKind, SceneNode, Camera, Scene)
// and a `ViewForm` struct with `instantiate` + typed `call_hello` / `call_render`.
wasmtime::component::bindgen!({
    world: "view-form",
    path: "wit/view-form.wit",
});

/// A real CEG `scores` envelope, like the native spike's tests use.
const SCORES_JSON: &str = r#"{
  "attestation_type": "scores",
  "dimension": "evaluation:quality",
  "score": 0.85,
  "confidence": 0.92,
  "evidence_refs": [],
  "cohort_scope": "self"
}"#;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let view_form_path = args
        .get(1)
        .cloned()
        .unwrap_or_else(|| "view_form_component.wasm".to_string());
    let malicious_path = args.get(2).cloned();

    // Component model is a default wasmtime feature; enable explicitly for clarity.
    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;

    // ---------------------------------------------------------------------
    // 1. The sandboxed view-form component: instantiate with ZERO imports.
    // ---------------------------------------------------------------------
    let component = Component::from_file(&engine, &view_form_path)?;

    // EMPTY linker. No WASI, no host functions, nothing. The store data is `()`
    // because there is no host state for the guest to touch.
    let linker: Linker<()> = Linker::new(&engine);
    let mut store = Store::new(&engine, ());

    let bindings = ViewForm::instantiate(&mut store, &component, &linker)?;

    let greeting = bindings.call_hello(&mut store, "world")?;
    println!("[hello] {greeting}");

    let scene = bindings.call_render(&mut store, SCORES_JSON)?;
    println!(
        "[render] {} nodes, background=({:.3},{:.3},{:.3},{:.3})",
        scene.nodes.len(),
        scene.background.r,
        scene.background.g,
        scene.background.b,
        scene.background.a,
    );
    for n in &scene.nodes {
        println!(
            "    node mesh={:?} pos=({:.2},{:.2},{:.2}) scale={:.2} color=({:.2},{:.2},{:.2},{:.2}) label={:?} source={}",
            n.mesh, n.pos.x, n.pos.y, n.pos.z, n.scale,
            n.color.r, n.color.g, n.color.b, n.color.a, n.label, n.source,
        );
    }

    // Assert the proven-spike mapping survived the WIT round-trip.
    let subject = scene
        .nodes
        .iter()
        .find(|n| n.label.as_deref() == Some("evaluation:quality"))
        .expect("subject sphere labeled with the dimension");
    assert!(matches!(subject.mesh, MeshKind::Sphere));
    assert!(
        subject.color.g > subject.color.r,
        "score=0.85 must be green-dominant, got {:?}",
        subject.color
    );
    assert_eq!(scene.nodes.len(), 4, "ground + 2 refs + subject");
    println!("[assert] Scene IR round-trip OK: subject is green-dominant sphere, 4 nodes");

    // ---------------------------------------------------------------------
    // 2. Sandbox proof: a guest that IMPORTS a host capability must be
    //    REJECTED by the same empty linker.
    // ---------------------------------------------------------------------
    if let Some(mp) = malicious_path {
        let mal = Component::from_file(&engine, &mp)?;
        // Low-level untyped instantiate against the SAME empty linker.
        match linker.instantiate(&mut store, &mal) {
            Ok(_) => {
                println!("[sandbox] UNEXPECTED: malicious component instantiated against empty linker!");
                anyhow::bail!("sandbox breach: import was satisfied with no grant");
            }
            Err(e) => {
                println!("[sandbox] REJECTED as expected — empty linker cannot satisfy the guest's import:");
                println!("[sandbox] error: {e}");
            }
        }
    }

    println!("\nOK: sandboxed component called across a WIT-typed boundary with zero imports.");
    Ok(())
}
