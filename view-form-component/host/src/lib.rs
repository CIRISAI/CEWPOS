//! view-form-host (library) — the wasmtime host runtime for the sandboxed
//! `view-form` component.
//!
//! This library factors out everything the demo binary and the integration test
//! both need:
//!
//!   * the compile-time `bindgen!` of the `view-form` world (which emits the
//!     host-side Scene IR types + the typed `ViewForm` instantiation surface),
//!   * a helper that instantiates a component against an EMPTY linker (zero host
//!     imports — the sandbox) and calls `render`,
//!   * introspection helpers that read the component's declared imports/exports
//!     straight from the artifact, so "imports = 0" is *evidence*, not a claim,
//!   * a Scene -> JSON serializer for the stretch goal.

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
pub const SCORES_JSON: &str = r#"{
  "attestation_type": "scores",
  "dimension": "evaluation:quality",
  "score": 0.85,
  "confidence": 0.92,
  "evidence_refs": [],
  "cohort_scope": "self"
}"#;

/// Build a component-model-enabled engine.
pub fn make_engine() -> Result<Engine> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    Ok(Engine::new(&config)?)
}

/// The names of the instances/funcs a component *imports* (i.e. the host
/// capabilities it demands). For a properly sandboxed view-form this is empty.
pub fn import_names(engine: &Engine, component: &Component) -> Vec<String> {
    component
        .component_type()
        .imports(engine)
        .map(|(name, _)| name.to_string())
        .collect()
}

/// The names a component *exports* (what the host is allowed to call).
pub fn export_names(engine: &Engine, component: &Component) -> Vec<String> {
    component
        .component_type()
        .exports(engine)
        .map(|(name, _)| name.to_string())
        .collect()
}

/// Instantiate `component_path` against an EMPTY linker (no WASI, no host
/// functions, store data = `()`) and call `render(json)`. Returns the typed
/// Scene. This is the whole sandbox in one function: nothing is ever added to
/// the linker, so the guest can reach nothing.
pub fn render_with_empty_linker(component_path: &str, json: &str) -> Result<Scene> {
    let engine = make_engine()?;
    let component = Component::from_file(&engine, component_path)?;

    // EMPTY linker. No WASI, no host functions, nothing.
    let linker: Linker<()> = Linker::new(&engine);
    let mut store = Store::new(&engine, ());

    let bindings = ViewForm::instantiate(&mut store, &component, &linker)?;
    let scene = bindings.call_render(&mut store, json)?;
    Ok(scene)
}

/// Map the closed `MeshKind` enum to its WIT/IR name.
pub fn mesh_name(mesh: &MeshKind) -> &'static str {
    match mesh {
        MeshKind::Sphere => "sphere",
        MeshKind::Cube => "cube",
        MeshKind::Plane => "plane",
        MeshKind::Cylinder => "cylinder",
    }
}

/// Serialize a typed Scene to a renderer-neutral JSON document (the stretch
/// goal: this could feed a JSON-driven renderer such as the Bevy spike in
/// view-forms-spike). Done entirely host-side; the guest never sees it.
pub fn scene_to_json(scene: &Scene) -> String {
    use serde_json::json;
    let nodes: Vec<_> = scene
        .nodes
        .iter()
        .map(|n| {
            json!({
                "pos": [n.pos.x, n.pos.y, n.pos.z],
                "scale": n.scale,
                "mesh": mesh_name(&n.mesh),
                "color": [n.color.r, n.color.g, n.color.b, n.color.a],
                "label": n.label,
                "source": n.source,
            })
        })
        .collect();
    let doc = json!({
        "nodes": nodes,
        "camera": {
            "eye": [scene.camera.eye.x, scene.camera.eye.y, scene.camera.eye.z],
            "look_at": [scene.camera.look_at.x, scene.camera.look_at.y, scene.camera.look_at.z],
        },
        "background": [
            scene.background.r,
            scene.background.g,
            scene.background.b,
            scene.background.a,
        ],
    });
    serde_json::to_string_pretty(&doc).unwrap_or_else(|_| "{}".to_string())
}
