//! scene.rs — the **typed Scene IR**: our renderer-neutral "typed 3D dialect".
//!
//! This is pure data with **no Bevy dependency**. It is the contract between the
//! pure view layer ([`crate::view_form::view_form`]) and *any* renderer. The
//! Bevy headless renderer (`src/bin/render.rs`) is just one consumer; the same
//! `Scene` could drive a web `<canvas>`, a glTF exporter, or an ASCII dumper.
//!
//! Keeping the IR Bevy-free is what makes the view logic browser-portable: this
//! module (and `view_form`) compile to `wasm32-unknown-unknown` without dragging
//! in a renderer. BSN (Bevy Scene Notation, the `bsn!` macro) is the Bevy-native
//! dialect option; we deliberately use our own typed IR + plain ECS spawn for
//! the spike because it is renderer-neutral and does not couple the pure view
//! function to a specific engine.
//!
//! Coordinates are right-handed Y-up (the Bevy convention) so consumers need no
//! axis remapping. Colors are linear-ish sRGB `[r, g, b, a]` in `0.0..=1.0`.

/// The primitive shapes the IR can express. Closed and small on purpose: a
/// renderer only needs a finite mesh vocabulary, and a closed enum means every
/// `MeshKind` is exhaustively handled by every backend (no `Unknown` escape).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MeshKind {
    /// A unit-diameter sphere centered at the node position.
    Sphere,
    /// A unit cube centered at the node position.
    Cube,
    /// A flat square in the XZ plane (a ground/reference surface). Renderers
    /// keep it thin in Y; `scale` controls its side length.
    Plane,
}

/// One renderable thing: a positioned, scaled, colored primitive with an
/// optional human/agent-legible label.
#[derive(Clone, Debug, PartialEq)]
pub struct SceneNode {
    /// World-space position of the node's center, `[x, y, z]`.
    pub pos: [f32; 3],
    /// Uniform scale factor applied to the unit primitive.
    pub scale: f32,
    /// Which primitive to draw.
    pub mesh: MeshKind,
    /// Linear sRGB color `[r, g, b, a]`, each in `0.0..=1.0`.
    pub color: [f32; 4],
    /// Optional label (e.g. the attestation `dimension`). Renderers may show it
    /// as a billboard, a tooltip, or ignore it; it is metadata, not geometry.
    pub label: Option<String>,
}

impl SceneNode {
    /// Convenience constructor for an unlabeled node.
    pub fn new(mesh: MeshKind, pos: [f32; 3], scale: f32, color: [f32; 4]) -> Self {
        Self { pos, scale, mesh, color, label: None }
    }

    /// Builder-style label setter.
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
}

/// A renderer-neutral camera: an eye position looking at a target. Up is `+Y`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Camera {
    /// Eye / camera position in world space.
    pub eye: [f32; 3],
    /// The point the camera looks at.
    pub look_at: [f32; 3],
}

/// The whole scene: a list of nodes, a camera, and a background clear color.
///
/// This is the *entire* renderer contract. A backend reads `background` as the
/// clear color, places a camera per `camera`, and spawns one drawable per node.
#[derive(Clone, Debug, PartialEq)]
pub struct Scene {
    pub nodes: Vec<SceneNode>,
    pub camera: Camera,
    /// Linear sRGB clear color `[r, g, b]`.
    pub background: [f32; 3],
}

impl Scene {
    /// Number of renderable nodes — handy for logging / assertions.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// The first node carrying the given label, if any. Used by tests and the
    /// renderer to find the "subject" node (e.g. the score sphere).
    pub fn node_with_label(&self, label: &str) -> Option<&SceneNode> {
        self.nodes
            .iter()
            .find(|n| n.label.as_deref() == Some(label))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scene_node_builder_sets_fields() {
        let n = SceneNode::new(MeshKind::Sphere, [1.0, 2.0, 3.0], 0.5, [0.1, 0.2, 0.3, 1.0])
            .with_label("evaluation:quality");
        assert_eq!(n.mesh, MeshKind::Sphere);
        assert_eq!(n.pos, [1.0, 2.0, 3.0]);
        assert_eq!(n.scale, 0.5);
        assert_eq!(n.color, [0.1, 0.2, 0.3, 1.0]);
        assert_eq!(n.label.as_deref(), Some("evaluation:quality"));
    }

    #[test]
    fn scene_counts_and_finds_nodes() {
        let scene = Scene {
            nodes: vec![
                SceneNode::new(MeshKind::Plane, [0.0, -1.0, 0.0], 8.0, [0.9, 0.9, 0.9, 1.0]),
                SceneNode::new(MeshKind::Sphere, [0.0, 0.0, 0.0], 1.0, [0.2, 0.7, 0.3, 1.0])
                    .with_label("subject"),
            ],
            camera: Camera { eye: [3.0, 3.0, 5.0], look_at: [0.0, 0.0, 0.0] },
            background: [0.98, 0.976, 0.961],
        };
        assert_eq!(scene.node_count(), 2);
        assert_eq!(scene.node_with_label("subject").unwrap().mesh, MeshKind::Sphere);
        assert!(scene.node_with_label("nope").is_none());
    }

    #[test]
    fn mesh_kinds_are_distinct() {
        assert_ne!(MeshKind::Sphere, MeshKind::Cube);
        assert_ne!(MeshKind::Cube, MeshKind::Plane);
    }
}
