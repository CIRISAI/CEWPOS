//! view_form.rs — the **pure, total view function** `view_form(&Form) -> Scene`.
//!
//! This is the heart of the spike: a deterministic, side-effect-free mapping
//! from a REAL Attestation-Calculus [`Form`] (read by the closed-operator gate
//! `attestation_calculus_spike::forms::read_form`) to our renderer-neutral
//! [`Scene`] IR. No IO, no clock, no RNG, no GPU — same attestation in, same
//! `Scene` out, on any platform (native or wasm).
//!
//! ## The `scores` mapping (CC 2.1)
//!
//! A `scores` attestation carries (among other fields) a `dimension`, a numeric
//! `score` in `-1.0..=1.0`, and a `confidence` in `0.0..=1.0`. We render it as a
//! single **subject sphere at the origin**:
//!
//! * **color** lerps red → green as `score` goes `-1 → +1`
//!   (red = "scored low/against", green = "scored high/for"),
//! * **scale** grows with `confidence` (a confident attestation is a bigger,
//!   more present sphere),
//! * **label** is the `dimension` string.
//!
//! To make the single sphere *legible* we also emit a ground plane and two small
//! reference markers at the red (`score = -1`) and green (`score = +1`) ends of
//! the score axis, plus a camera and a Bone background. The whole thing is
//! deterministic data.
//!
//! Non-`scores` forms (the four composers) are still handled totally: they get a
//! neutral grey subject sphere labeled with the operator name, so the function
//! never panics and never returns an "empty" scene.

use crate::scene::{Camera, MeshKind, Scene, SceneNode};
use attestation_calculus_spike::forms::{Form, Op};

/// Bone `#FAF9F5` background (linear-ish sRGB), matching the CIRIS palette.
const BONE: [f32; 3] = [0xFA as f32 / 255.0, 0xF9 as f32 / 255.0, 0xF5 as f32 / 255.0];

/// Red endpoint of the score color axis (`score = -1`).
const SCORE_RED: [f32; 3] = [0.85, 0.12, 0.12];
/// Green endpoint of the score color axis (`score = +1`).
const SCORE_GREEN: [f32; 3] = [0.20, 0.72, 0.30];

/// Minimum subject-sphere scale (at `confidence = 0`).
const SCALE_MIN: f32 = 0.45;
/// Maximum subject-sphere scale (at `confidence = 1`).
const SCALE_MAX: f32 = 1.65;

/// Map a `score` in `[-1, 1]` to an opaque red→green color. Values outside the
/// range are clamped (the function stays total on any `f64`, incl. NaN -> 0).
pub fn score_to_color(score: f64) -> [f32; 4] {
    // NaN-safe clamp into [-1, 1], then normalize to t in [0, 1].
    let s = if score.is_nan() { 0.0 } else { score.clamp(-1.0, 1.0) };
    let t = ((s + 1.0) / 2.0) as f32;
    [
        lerp(SCORE_RED[0], SCORE_GREEN[0], t),
        lerp(SCORE_RED[1], SCORE_GREEN[1], t),
        lerp(SCORE_RED[2], SCORE_GREEN[2], t),
        1.0,
    ]
}

/// Map a `confidence` in `[0, 1]` to a subject-sphere scale in
/// `[SCALE_MIN, SCALE_MAX]`. Monotonic non-decreasing; clamped and NaN-safe.
pub fn confidence_to_scale(confidence: f64) -> f32 {
    let c = if confidence.is_nan() { 0.0 } else { confidence.clamp(0.0, 1.0) };
    SCALE_MIN + (c as f32) * (SCALE_MAX - SCALE_MIN)
}

/// Linear interpolation between `a` and `b` by `t` in `[0, 1]`.
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// Read an `f64` member from the envelope, defaulting if absent / non-numeric.
fn num(form: &Form, key: &str, default: f64) -> f64 {
    form.envelope
        .get(key)
        .and_then(|v| v.as_f64())
        .unwrap_or(default)
}

/// Read a string member from the envelope, defaulting if absent / non-string.
fn text<'a>(form: &'a Form, key: &str, default: &'a str) -> &'a str {
    form.envelope
        .get(key)
        .and_then(|v| v.as_str())
        .unwrap_or(default)
}

/// **The pure, total view.** Same `Form` in, same `Scene` out — no effects.
pub fn view_form(form: &Form) -> Scene {
    // --- Ground plane (a legibility surface, not part of the data) ----------
    let ground = SceneNode::new(
        MeshKind::Plane,
        [0.0, -1.2, 0.0],
        9.0,
        [0.88, 0.87, 0.84, 1.0], // faint warm grey, reads against Bone
    );

    // --- The subject node: the attestation itself --------------------------
    let (subject, mut nodes) = match form.op {
        Op::Scores => {
            let score = num(form, "score", 0.0);
            let confidence = num(form, "confidence", 0.0);
            let dimension = text(form, "dimension", "scores").to_string();

            let subject = SceneNode::new(
                MeshKind::Sphere,
                [0.0, 0.0, 0.0],
                confidence_to_scale(confidence),
                score_to_color(score),
            )
            .with_label(dimension);

            // Reference markers anchor the red→green score axis so a viewer can
            // read whether the subject leans red (low) or green (high).
            let red_ref = SceneNode::new(
                MeshKind::Cube,
                [-3.0, -0.6, 0.0],
                0.35,
                [SCORE_RED[0], SCORE_RED[1], SCORE_RED[2], 1.0],
            )
            .with_label("score=-1");
            let green_ref = SceneNode::new(
                MeshKind::Cube,
                [3.0, -0.6, 0.0],
                0.35,
                [SCORE_GREEN[0], SCORE_GREEN[1], SCORE_GREEN[2], 1.0],
            )
            .with_label("score=+1");

            (subject, vec![red_ref, green_ref])
        }
        // The four composers: a neutral grey sphere labeled with the operator,
        // so view_form is total over the whole closed 1+4 set.
        other => {
            let subject = SceneNode::new(
                MeshKind::Sphere,
                [0.0, 0.0, 0.0],
                1.0,
                [0.55, 0.55, 0.58, 1.0],
            )
            .with_label(other.as_str());
            (subject, Vec::new())
        }
    };

    // Order: ground first (drawn under), references, then the subject.
    let mut all = vec![ground];
    all.append(&mut nodes);
    all.push(subject);

    Scene {
        nodes: all,
        camera: Camera { eye: [3.6, 3.0, 5.6], look_at: [0.0, 0.2, 0.0] },
        background: BONE,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use attestation_calculus_spike::forms::read_form;
    use serde_json::json;

    /// Build a real `scores` Form through the closed-operator gate.
    fn scores_form(score: f64, confidence: f64, dimension: &str) -> Form {
        let env = json!({
            "attestation_type": "scores",
            "dimension": dimension,
            "score": score,
            "confidence": confidence,
            "evidence_refs": [],
            "cohort_scope": "self"
        });
        read_form(&env).expect("well-formed scores envelope must read")
    }

    #[test]
    fn score_to_color_endpoints_and_midpoint() {
        let red = score_to_color(-1.0);
        let green = score_to_color(1.0);
        let mid = score_to_color(0.0);
        // -1 is reddish: red channel dominates green.
        assert!(red[0] > red[1], "score=-1 must be red-dominant: {red:?}");
        // +1 is greenish: green channel dominates red.
        assert!(green[1] > green[0], "score=+1 must be green-dominant: {green:?}");
        // 0 is the exact midpoint of the two endpoints.
        assert!((mid[0] - (SCORE_RED[0] + SCORE_GREEN[0]) / 2.0).abs() < 1e-6);
        assert!((mid[1] - (SCORE_RED[1] + SCORE_GREEN[1]) / 2.0).abs() < 1e-6);
        // Always opaque.
        assert_eq!(red[3], 1.0);
        assert_eq!(green[3], 1.0);
    }

    #[test]
    fn score_to_color_is_monotonic_in_green() {
        // As score rises, green increases and red decreases monotonically.
        let mut last_g = f32::MIN;
        let mut last_r = f32::MAX;
        for i in -10..=10 {
            let c = score_to_color(i as f64 / 10.0);
            assert!(c[1] >= last_g, "green must be non-decreasing in score");
            assert!(c[0] <= last_r, "red must be non-increasing in score");
            last_g = c[1];
            last_r = c[0];
        }
    }

    #[test]
    fn score_to_color_clamps_out_of_range_and_nan() {
        assert_eq!(score_to_color(5.0), score_to_color(1.0));
        assert_eq!(score_to_color(-5.0), score_to_color(-1.0));
        // NaN folds to the score=0 midpoint, never panics.
        assert_eq!(score_to_color(f64::NAN), score_to_color(0.0));
    }

    #[test]
    fn confidence_to_scale_is_monotonic_and_bounded() {
        assert!((confidence_to_scale(0.0) - SCALE_MIN).abs() < 1e-6);
        assert!((confidence_to_scale(1.0) - SCALE_MAX).abs() < 1e-6);
        // Strictly increasing on the interior.
        assert!(confidence_to_scale(0.92) > confidence_to_scale(0.5));
        assert!(confidence_to_scale(0.5) > confidence_to_scale(0.1));
        // Clamps + NaN-safe.
        assert_eq!(confidence_to_scale(2.0), confidence_to_scale(1.0));
        assert_eq!(confidence_to_scale(-1.0), confidence_to_scale(0.0));
        assert_eq!(confidence_to_scale(f64::NAN), confidence_to_scale(0.0));
    }

    #[test]
    fn view_form_maps_scores_subject_correctly() {
        let form = scores_form(0.85, 0.92, "evaluation:quality");
        let scene = view_form(&form);

        // Subject is found by its dimension label.
        let subject = scene
            .node_with_label("evaluation:quality")
            .expect("subject sphere labeled with dimension");
        assert_eq!(subject.mesh, MeshKind::Sphere);
        assert_eq!(subject.pos, [0.0, 0.0, 0.0]);
        // Color matches the pure mapping for score=0.85.
        assert_eq!(subject.color, score_to_color(0.85));
        // 0.85 is well into the green half: green channel beats red.
        assert!(subject.color[1] > subject.color[0]);
        // Scale matches the pure mapping for confidence=0.92.
        assert!((subject.scale - confidence_to_scale(0.92)).abs() < 1e-6);

        // Ground + 2 reference markers + subject = 4 nodes.
        assert_eq!(scene.node_count(), 4);
        assert!(scene.node_with_label("score=-1").is_some());
        assert!(scene.node_with_label("score=+1").is_some());

        // Background is Bone.
        assert_eq!(scene.background, BONE);
    }

    #[test]
    fn view_form_is_deterministic() {
        let a = view_form(&scores_form(0.3, 0.7, "evaluation:helpfulness"));
        let b = view_form(&scores_form(0.3, 0.7, "evaluation:helpfulness"));
        assert_eq!(a, b, "same attestation -> identical Scene");
    }

    #[test]
    fn low_score_is_red_high_score_is_green() {
        let low = view_form(&scores_form(-0.9, 0.5, "d"));
        let high = view_form(&scores_form(0.9, 0.5, "d"));
        let low_subject = low.node_with_label("d").unwrap();
        let high_subject = high.node_with_label("d").unwrap();
        assert!(low_subject.color[0] > low_subject.color[1], "low score is red");
        assert!(high_subject.color[1] > high_subject.color[0], "high score is green");
    }

    #[test]
    fn composer_form_is_total_neutral() {
        // A `supersedes` form (a composer, not `scores`) still yields a valid,
        // non-empty scene with a neutral subject labeled by the operator.
        let env = json!({
            "attestation_type": "supersedes",
            "dimension": "evaluation:quality"
        });
        let form = read_form(&env).expect("supersedes reads");
        let scene = view_form(&form);
        assert!(scene.node_with_label("supersedes").is_some());
        // ground + subject (no score-axis references for composers).
        assert_eq!(scene.node_count(), 2);
    }
}
