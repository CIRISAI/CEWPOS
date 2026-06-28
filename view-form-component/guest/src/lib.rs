//! view-form-guest — a fully sandboxed WebAssembly Component.
//!
//! It is built against the `view-form` WIT world, which declares ZERO imports.
//! Therefore this guest is structurally incapable of doing anything but
//! returning a typed `Scene`: there is no kernel, no substrate, no network, no
//! clock, no filesystem reachable from inside it. The only export that produces
//! data is `render`, which is a pure, total, deterministic function of its
//! string argument.
//!
//! The view logic mirrors the proven native spike (view-forms-spike): parse a
//! CEG `scores` envelope (dimension / score / confidence) and map
//! score -> color (red..green), confidence -> scale, dimension -> label.

// `generate!` reads the WIT in ./wit and produces:
//   * trait `Guest` with `hello` and `render`,
//   * the Scene IR types `Vec3`, `Rgba`, `MeshKind`, `SceneNode`, `Camera`, `Scene`,
//   * the `export!` macro to wire up our implementation.
wit_bindgen::generate!({
    world: "view-form",
    path: "wit",
});

use serde_json::Value;

/// Bone `#FAF9F5` background (linear-ish sRGB), matching the CIRIS palette.
const BONE: Rgba = Rgba { r: 250.0 / 255.0, g: 249.0 / 255.0, b: 245.0 / 255.0, a: 1.0 };

/// Red endpoint of the score color axis (`score = -1`).
const SCORE_RED: [f32; 3] = [0.85, 0.12, 0.12];
/// Green endpoint of the score color axis (`score = +1`).
const SCORE_GREEN: [f32; 3] = [0.20, 0.72, 0.30];

/// Minimum subject-sphere scale (at `confidence = 0`).
const SCALE_MIN: f32 = 0.45;
/// Maximum subject-sphere scale (at `confidence = 1`).
const SCALE_MAX: f32 = 1.65;

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// Map a `score` in [-1, 1] to an opaque red->green color. NaN-safe, clamped.
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

/// Map a `confidence` in [0, 1] to a scale in [SCALE_MIN, SCALE_MAX]. NaN-safe.
fn confidence_to_scale(confidence: f64) -> f32 {
    let c = if confidence.is_nan() { 0.0 } else { confidence.clamp(0.0, 1.0) };
    SCALE_MIN + (c as f32) * (SCALE_MAX - SCALE_MIN)
}

fn node(
    mesh: MeshKind,
    pos: [f32; 3],
    scale: f32,
    color: Rgba,
    label: Option<&str>,
    source: &str,
) -> SceneNode {
    SceneNode {
        pos: Vec3 { x: pos[0], y: pos[1], z: pos[2] },
        scale,
        mesh,
        color,
        label: label.map(|s| s.to_string()),
        source: source.to_string(),
    }
}

/// The pure, total view: same JSON in, same Scene out — no effects.
///
/// Note the deliberate absence of `unwrap()` on parse: malformed input yields a
/// `Null` value and a defined fallback scene, so the component never panics /
/// traps on adversarial input. Totality is part of the safety contract.
fn render_impl(attestation_json: &str) -> Scene {
    let v: Value = serde_json::from_str(attestation_json).unwrap_or(Value::Null);
    let atype = v.get("attestation_type").and_then(|x| x.as_str()).unwrap_or("");

    // Ground plane (a legibility surface, not part of the data).
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

        // Reference markers anchor the red->green score axis.
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

        // The subject node: the attestation itself.
        nodes.push(node(
            MeshKind::Sphere,
            [0.0, 0.0, 0.0],
            confidence_to_scale(confidence),
            score_to_color(score),
            Some(dimension),
            "ceg:scores",
        ));
    } else {
        // Non-`scores` envelopes still render totally: a neutral grey sphere.
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

/// The component instance implementing the world's exports.
struct Component;

impl Guest for Component {
    fn hello(name: String) -> String {
        format!("hello, {name} -- from a fully sandboxed WASM component (zero imports)")
    }

    fn render(attestation_json: String) -> Scene {
        render_impl(&attestation_json)
    }
}

export!(Component);
