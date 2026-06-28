//! view-form-host (binary) — the demo. Loads a WebAssembly Component and calls
//! it across a WIT-typed boundary, granting it ZERO imports (empty linker).
//!
//! This proves the enforcement layer behind "no hallucinated UI: safe by type +
//! sandbox, not trust":
//!
//!   1. The legitimate `view-form` component imports NOTHING (verified by
//!      reading its component type), so it instantiates cleanly against an EMPTY
//!      `Linker`, and the only data it can return is a WIT-typed `Scene`.
//!   2. The `malicious` component imports a host capability; instantiating it
//!      against the SAME empty linker FAILS — captured as evidence below.
//!
//! Usage:
//!   view-form-host <view_form_component.wasm> [malicious_component.wasm]

use anyhow::Result;
use wasmtime::component::{Component, Linker};
use wasmtime::Store;

use view_form_host::{
    export_names, import_names, make_engine, mesh_name, scene_to_json, MeshKind, ViewForm,
    SCORES_JSON,
};

/// Where the stretch-goal scene.json lands (deterministic regardless of cwd).
const SCENE_OUT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../out/scene.json");

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let view_form_path = args
        .get(1)
        .cloned()
        .unwrap_or_else(|| "../dist/view_form_component.wasm".to_string());
    let malicious_path = args.get(2).cloned();

    let engine = make_engine()?;

    // ---------------------------------------------------------------------
    // 1. The sandboxed view-form component: instantiate with ZERO imports.
    // ---------------------------------------------------------------------
    let component = Component::from_file(&engine, &view_form_path)?;

    // --- SANDBOX EVIDENCE, read straight from the component artifact ------
    let imports = import_names(&engine, &component);
    let exports = export_names(&engine, &component);
    println!("== sandbox evidence (from the component's own type) ==");
    println!("[sandbox] view-form component imports granted-by-world: {} {:?}", imports.len(), imports);
    println!("[sandbox] view-form component exports:                  {} {:?}", exports.len(), exports);
    println!("[sandbox] WASI: none  (built for wasm32-unknown-unknown; no wasi:* imports exist)");

    // EMPTY linker. No WASI, no host functions, nothing. The store data is `()`
    // because there is no host state for the guest to touch.
    let linker: Linker<()> = Linker::new(&engine);
    let mut store = Store::new(&engine, ());
    println!("[sandbox] host linker grants:                           0 (empty Linker<()>, store data = ())");
    println!();

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
            "    node mesh={:<8} pos=({:+.2},{:+.2},{:+.2}) scale={:.2} color=({:.2},{:.2},{:.2},{:.2}) label={:?} source={}",
            mesh_name(&n.mesh),
            n.pos.x, n.pos.y, n.pos.z, n.scale,
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
        "score=0.85 must be green-dominant, got rgba({},{},{},{})",
        subject.color.r, subject.color.g, subject.color.b, subject.color.a,
    );
    assert_eq!(scene.nodes.len(), 4, "ground + 2 refs + subject");
    println!("[assert] Scene IR round-trip OK: subject is a green-dominant sphere, 4 nodes");

    // --- STRETCH: write the typed scene out as JSON for a renderer --------
    let json = scene_to_json(&scene);
    if let Err(e) = std::fs::write(SCENE_OUT, &json) {
        println!("[scene.json] (skipped: {e})");
    } else {
        println!("[scene.json] wrote {} bytes -> {}", json.len(), SCENE_OUT);
    }

    // ---------------------------------------------------------------------
    // 2. Sandbox proof: a guest that IMPORTS a host capability must be
    //    REJECTED by the same empty linker.
    // ---------------------------------------------------------------------
    if let Some(mp) = malicious_path {
        println!();
        let mal = Component::from_file(&engine, &mp)?;
        let mal_imports = import_names(&engine, &mal);
        println!("[sandbox] malicious component demands imports: {} {:?}", mal_imports.len(), mal_imports);
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
