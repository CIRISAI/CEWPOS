//! cc-guest — the SAME sandboxed view-form component, built via the blessed
//! `cargo-component` toolchain instead of wit-bindgen + `wasm-tools component new`.
//!
//! cargo-component generates `src/bindings.rs` from `wit/world.wit` at build
//! time; we implement the generated `Guest` trait. Built for
//! `wasm32-unknown-unknown` it produces a component that imports NOTHING — the
//! exact same zero-import sandbox, just via Path B of the toolchain.

#[allow(warnings)]
mod bindings;

use bindings::{Camera, Guest, MeshKind, Rgba, Scene, SceneNode, Vec3};
use serde_json::Value;

const BONE: Rgba = Rgba { r: 250.0 / 255.0, g: 249.0 / 255.0, b: 245.0 / 255.0, a: 1.0 };
const SCORE_RED: [f32; 3] = [0.85, 0.12, 0.12];
const SCORE_GREEN: [f32; 3] = [0.20, 0.72, 0.30];
const SCALE_MIN: f32 = 0.45;
const SCALE_MAX: f32 = 1.65;

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

fn score_to_color(score: f64) -> Rgba {
    let s = if score.is_nan() { 0.0 } else { score.clamp(-1.0, 1.0) };
    let t = ((s + 1.0) / 2.0) as f32;
    Rgba {
        r: lerp(SCORE_RED[0], SCORE_GREEN[0], t),
        g: lerp(SCORE_RED[1], SCORE_GREEN[1], t),
        b: lerp(SCORE_RED[2], SCORE_GREEN[2], t),
        a: 1.0,
    }
}

fn confidence_to_scale(confidence: f64) -> f32 {
    let c = if confidence.is_nan() { 0.0 } else { confidence.clamp(0.0, 1.0) };
    SCALE_MIN + (c as f32) * (SCALE_MAX - SCALE_MIN)
}

fn node(mesh: MeshKind, pos: [f32; 3], scale: f32, color: Rgba, label: Option<&str>, source: &str) -> SceneNode {
    SceneNode {
        pos: Vec3 { x: pos[0], y: pos[1], z: pos[2] },
        scale,
        mesh,
        color,
        label: label.map(|s| s.to_string()),
        source: source.to_string(),
    }
}

fn render_impl(attestation_json: &str) -> Scene {
    let v: Value = serde_json::from_str(attestation_json).unwrap_or(Value::Null);
    let atype = v.get("attestation_type").and_then(|x| x.as_str()).unwrap_or("");

    let ground = node(
        MeshKind::Plane,
        [0.0, -1.2, 0.0],
        9.0,
        Rgba { r: 0.88, g: 0.87, b: 0.84, a: 1.0 },
        None,
        "view-form:ground",
    );
    let mut nodes = vec![ground];

    if atype == "scores" {
        let score = v.get("score").and_then(|x| x.as_f64()).unwrap_or(0.0);
        let confidence = v.get("confidence").and_then(|x| x.as_f64()).unwrap_or(0.0);
        let dimension = v.get("dimension").and_then(|x| x.as_str()).unwrap_or("scores");

        nodes.push(node(
            MeshKind::Cube,
            [-3.0, -0.6, 0.0],
            0.35,
            Rgba { r: SCORE_RED[0], g: SCORE_RED[1], b: SCORE_RED[2], a: 1.0 },
            Some("score=-1"),
            "view-form:ref",
        ));
        nodes.push(node(
            MeshKind::Cube,
            [3.0, -0.6, 0.0],
            0.35,
            Rgba { r: SCORE_GREEN[0], g: SCORE_GREEN[1], b: SCORE_GREEN[2], a: 1.0 },
            Some("score=+1"),
            "view-form:ref",
        ));
        nodes.push(node(
            MeshKind::Sphere,
            [0.0, 0.0, 0.0],
            confidence_to_scale(confidence),
            score_to_color(score),
            Some(dimension),
            "ceg:scores",
        ));
    } else {
        let label = if atype.is_empty() { "unknown" } else { atype };
        nodes.push(node(
            MeshKind::Sphere,
            [0.0, 0.0, 0.0],
            1.0,
            Rgba { r: 0.55, g: 0.55, b: 0.58, a: 1.0 },
            Some(label),
            "ceg:other",
        ));
    }

    Scene {
        nodes,
        camera: Camera {
            eye: Vec3 { x: 3.6, y: 3.0, z: 5.6 },
            look_at: Vec3 { x: 0.0, y: 0.2, z: 0.0 },
        },
        background: BONE,
    }
}

struct Component;

impl Guest for Component {
    fn hello(name: String) -> String {
        format!("hello, {name} -- from a cargo-component-built sandboxed WASM component")
    }

    fn render(attestation_json: String) -> Scene {
        render_impl(&attestation_json)
    }
}

bindings::export!(Component with_types_in bindings);
