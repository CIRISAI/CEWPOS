//! Integration test: the host loads the SANDBOXED component and gets back a
//! correctly typed Scene with the expected node/color mapping — and a component
//! that demands an import is REJECTED by the empty linker.
//!
//! These tests require the components to be built first (see ../build.sh):
//!   dist/view_form_component.wasm   (the sandboxed view-form)
//!   dist/malicious_component.wasm   (the sandbox-breaker)
//! If the artifacts are missing the tests are skipped (so `cargo test` is green
//! on a clean checkout) — build.sh builds them before running the suite.

use view_form_host::{
    export_names, import_names, make_engine, render_with_empty_linker, MeshKind, SCORES_JSON,
};
use wasmtime::component::{Component, Linker};
use wasmtime::Store;

const VIEW_FORM: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../dist/view_form_component.wasm");
const MALICIOUS: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../dist/malicious_component.wasm");

#[test]
fn scores_attestation_renders_green_dominant_sphere() {
    if !std::path::Path::new(VIEW_FORM).exists() {
        eprintln!("skipping: {VIEW_FORM} not built yet (run build.sh)");
        return;
    }

    let scene = render_with_empty_linker(VIEW_FORM, SCORES_JSON)
        .expect("sandboxed render against empty linker");

    // ground + 2 reference markers + subject sphere.
    assert_eq!(scene.nodes.len(), 4, "ground + 2 refs + subject");

    let subject = scene
        .nodes
        .iter()
        .find(|n| n.label.as_deref() == Some("evaluation:quality"))
        .expect("subject node labeled with the dimension");

    assert!(matches!(subject.mesh, MeshKind::Sphere), "subject is a sphere");
    // score = 0.85 (positive) -> green channel dominates red.
    assert!(
        subject.color.g > subject.color.r,
        "score 0.85 must be green-dominant; got g={} r={}",
        subject.color.g,
        subject.color.r,
    );
    // confidence = 0.92 -> near the high end of [0.45, 1.65].
    assert!(subject.scale > 1.4, "high confidence -> large sphere, got {}", subject.scale);
    // Provenance is preserved through the typed boundary.
    assert_eq!(subject.source, "ceg:scores");
    // Bone background round-tripped.
    assert!((scene.background.r - 250.0 / 255.0).abs() < 1e-4);
}

#[test]
fn malformed_json_is_total_no_panic() {
    if !std::path::Path::new(VIEW_FORM).exists() {
        eprintln!("skipping: {VIEW_FORM} not built yet (run build.sh)");
        return;
    }
    // Adversarial / garbage input must NOT panic or trap — defined fallback.
    let scene = render_with_empty_linker(VIEW_FORM, "}{ not json at all")
        .expect("render must be total on bad input");
    // Fallback scene: ground + single neutral 'unknown' sphere.
    assert_eq!(scene.nodes.len(), 2);
    assert!(scene
        .nodes
        .iter()
        .any(|n| n.label.as_deref() == Some("unknown")));
}

#[test]
fn view_form_component_imports_nothing() {
    if !std::path::Path::new(VIEW_FORM).exists() {
        eprintln!("skipping: {VIEW_FORM} not built yet (run build.sh)");
        return;
    }
    let engine = make_engine().unwrap();
    let component = Component::from_file(&engine, VIEW_FORM).unwrap();
    let imports = import_names(&engine, &component);
    assert!(imports.is_empty(), "sandboxed view-form must import nothing, got {imports:?}");
    let exports = export_names(&engine, &component);
    assert!(exports.iter().any(|e| e.contains("render")), "must export render, got {exports:?}");
}

#[test]
fn malicious_component_is_rejected_by_empty_linker() {
    if !std::path::Path::new(MALICIOUS).exists() {
        eprintln!("skipping: {MALICIOUS} not built yet (run build.sh)");
        return;
    }
    let engine = make_engine().unwrap();
    let component = Component::from_file(&engine, MALICIOUS).unwrap();

    // It really does demand an import.
    let imports = import_names(&engine, &component);
    assert!(!imports.is_empty(), "malicious guest must declare an import");

    // The SAME empty linker the view-form used must REFUSE to instantiate it.
    let linker: Linker<()> = Linker::new(&engine);
    let mut store = Store::new(&engine, ());
    let result = linker.instantiate(&mut store, &component);
    assert!(
        result.is_err(),
        "empty linker MUST reject a component with an unsatisfied import"
    );
    let msg = format!("{}", result.err().unwrap());
    assert!(
        msg.contains("host-caps") || msg.contains("matching implementation was not found"),
        "rejection error should name the unsatisfied import; got: {msg}"
    );
}
