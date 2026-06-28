//! render.rs — the Bevy 0.19 HEADLESS renderer for the typed Scene IR.
//!
//! End-to-end proof of the CIRISGame DESIGN_BRIEF §7.4 path on this machine:
//!
//!   real `scores` Form  ->  view_form() -> Scene IR  ->  Bevy entities
//!     ->  Camera { RenderTarget::Image(handle) }  (off-screen, no window)
//!     ->  Readback::texture(handle) -> ReadbackComplete(Vec<u8>)
//!     ->  out/attestation-render.png
//!
//! No window is ever created (WinitPlugin disabled, ScheduleRunnerPlugin drives
//! the loop). macOS Metal supports headless wgpu, so no display is required.
//!
//! The Bevy-0.19 API used here is the VERIFIED known-good API from the
//! feasibility spike (RenderTarget as a standalone component, new_target_texture
//! + manual COPY_SRC, On<ReadbackComplete> observer, MessageWriter<AppExit>,
//! PointLight::shadow_maps_enabled, 256-byte row-padding strip).

use bevy::{
    app::{AppExit, ScheduleRunnerPlugin},
    camera::RenderTarget,
    core_pipeline::tonemapping::Tonemapping,
    prelude::*,
    render::{
        gpu_readback::{Readback, ReadbackComplete},
        render_resource::{TextureFormat, TextureUsages},
    },
    window::ExitCondition,
    winit::WinitPlugin,
};
use std::time::Duration;

use attestation_calculus_spike::forms::{read_form, Form};
use view_forms_spike::scene::{Camera as IrCamera, MeshKind, Scene};
use view_forms_spike::view_form;

// 512 is a multiple of 64 => width*4 = 2048 is 256-byte aligned (no GPU row
// padding). Big enough to be legible, still a fast headless render.
const WIDTH: u32 = 512;
const HEIGHT: u32 = 512;
const OUT_PATH: &str = "/Users/macmini/CEWPOS/view-forms-spike/out/attestation-render.png";
const PRE_ROLL_FRAMES: u32 = 8; // warm the PBR pipeline before reading back
const MAX_FRAMES: u32 = 600; // hard safety cap so the process always exits

/// Off-screen render target + output config, shared with the readback observer.
#[derive(Resource)]
struct Capture {
    target: Handle<Image>,
    width: u32,
    height: u32,
    path: String,
}

/// Pre-roll / one-shot bookkeeping.
#[derive(Resource)]
struct Driver {
    frame: u32,
    readback_spawned: bool,
}

/// Build the REAL `scores` Form for this render: a federation-tier quality
/// attestation, read through the closed-operator gate (no shortcuts).
fn build_scores_form() -> Form {
    let envelope = serde_json::json!({
        "attestation_type": "scores",
        "dimension": "evaluation:quality",
        "score": 0.85,
        "confidence": 0.92,
        "evidence_refs": [],
        "cohort_scope": "federation"
    });
    read_form(&envelope).expect("the running-example scores envelope must read")
}

fn main() {
    // 1) CALCULUS -> PURE VIEW -> TYPED SCENE IR (all before any Bevy/GPU work).
    let form = build_scores_form();
    let scene = view_form(&form);

    // Print the Scene IR summary (node count + the score->color it chose).
    println!("[view-forms] read Form: op=scores dimension=evaluation:quality score=0.85 confidence=0.92");
    println!("[view-forms] Scene IR: {} nodes, background={:?}", scene.node_count(), scene.background);
    if let Some(subject) = scene.node_with_label("evaluation:quality") {
        println!(
            "[view-forms]   subject sphere: pos={:?} scale={:.3} color(rgba)={:?} label={:?}",
            subject.pos, subject.scale, subject.color, subject.label
        );
        println!(
            "[view-forms]   score 0.85 -> color #{:02X}{:02X}{:02X} (green-leaning => 'scored high')",
            (subject.color[0] * 255.0) as u8,
            (subject.color[1] * 255.0) as u8,
            (subject.color[2] * 255.0) as u8,
        );
    }
    for n in &scene.nodes {
        println!(
            "[view-forms]   node: {:?} pos={:?} scale={:.2} label={:?}",
            n.mesh, n.pos, n.scale, n.label
        );
    }

    // 2) SCENE IR -> BEVY ENTITIES -> HEADLESS PNG.
    App::new()
        .insert_resource(ClearColor(Color::srgb(
            scene.background[0],
            scene.background[1],
            scene.background[2],
        )))
        .insert_resource(SceneRes(scene))
        .insert_resource(Driver { frame: 0, readback_spawned: false })
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: None,
                    exit_condition: ExitCondition::DontExit,
                    ..default()
                })
                .disable::<WinitPlugin>(),
        )
        .add_plugins(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(1.0 / 60.0)))
        .add_systems(Startup, setup)
        .add_systems(Update, drive)
        .run();
}

/// The typed Scene IR, handed into the ECS world for `setup` to spawn.
#[derive(Resource)]
struct SceneRes(Scene);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    scene: Res<SceneRes>,
) {
    // Off-screen render target. COPY_SRC is REQUIRED for the readback's
    // copy_texture_to_buffer; new_target_texture only sets
    // TEXTURE_BINDING | COPY_DST | RENDER_ATTACHMENT.
    let mut target = Image::new_target_texture(WIDTH, HEIGHT, TextureFormat::Rgba8UnormSrgb, None);
    target.texture_descriptor.usage |= TextureUsages::COPY_SRC;
    let target = images.add(target);

    // --- SceneNode -> Bevy entity (the IR -> ECS step) ---------------------
    for node in &scene.0.nodes {
        let mesh: Handle<Mesh> = match node.mesh {
            MeshKind::Sphere => meshes.add(Sphere::new(0.5)),
            MeshKind::Cube => meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            // A flat slab in XZ; scaled thin in Y so `scale` reads as side length.
            MeshKind::Plane => meshes.add(Cuboid::new(1.0, 0.05, 1.0)),
        };
        let material = materials.add(StandardMaterial {
            base_color: Color::srgba(node.color[0], node.color[1], node.color[2], node.color[3]),
            perceptual_roughness: 0.6,
            ..default()
        });
        // Planes scale only X/Z (stay flat); everything else scales uniformly.
        let scale_vec = match node.mesh {
            MeshKind::Plane => Vec3::new(node.scale, 1.0, node.scale),
            _ => Vec3::splat(node.scale),
        };
        commands.spawn((
            Mesh3d(mesh),
            MeshMaterial3d(material),
            Transform::from_translation(Vec3::from_array(node.pos)).with_scale(scale_vec),
        ));
    }

    // A key light + a fill light so the red->green sphere reads with shading.
    commands.spawn((
        PointLight { intensity: 2_500_000.0, shadow_maps_enabled: false, ..default() },
        Transform::from_xyz(5.0, 9.0, 5.0),
    ));
    commands.spawn((
        PointLight { intensity: 800_000.0, shadow_maps_enabled: false, ..default() },
        Transform::from_xyz(-4.0, 3.0, 4.0),
    ));

    // Camera from the IR camera -> off-screen image (NOT a window).
    let IrCamera { eye, look_at } = scene.0.camera;
    commands.spawn((
        Camera3d::default(),
        Camera { order: -1, ..default() },
        RenderTarget::Image(target.clone().into()),
        Tonemapping::None, // avoid needing tonemapping-LUT assets headless
        Transform::from_translation(Vec3::from_array(eye))
            .looking_at(Vec3::from_array(look_at), Vec3::Y),
    ));

    commands.insert_resource(Capture {
        target,
        width: WIDTH,
        height: HEIGHT,
        path: OUT_PATH.to_string(),
    });
}

/// Counts frames; after pre-roll spawns the Readback (+ observer); enforces cap.
fn drive(
    mut commands: Commands,
    cap: Res<Capture>,
    mut driver: ResMut<Driver>,
    mut exit: MessageWriter<AppExit>,
) {
    driver.frame += 1;

    if !driver.readback_spawned && driver.frame >= PRE_ROLL_FRAMES {
        driver.readback_spawned = true;
        commands
            .spawn(Readback::texture(cap.target.clone()))
            .observe(save_png_on_readback);
    }

    if driver.frame >= MAX_FRAMES {
        eprintln!("[view-forms] hit MAX_FRAMES without a readback; exiting");
        exit.write(AppExit::Success);
    }
}

/// Observer: receives the rendered RGBA8 bytes, strips any GPU row padding,
/// writes the PNG, prints path+size, exits.
fn save_png_on_readback(
    event: On<ReadbackComplete>,
    cap: Res<Capture>,
    mut exit: MessageWriter<AppExit>,
    mut done: Local<bool>,
) {
    if *done {
        return;
    }
    *done = true;

    let raw: &[u8] = &event.data;

    let pixel = 4usize; // Rgba8UnormSrgb
    let row = cap.width as usize * pixel;
    let aligned = (row + 255) & !255; // round up to wgpu's 256-byte alignment
    let tight: Vec<u8> = if row == aligned {
        raw.to_vec()
    } else {
        raw.chunks(aligned)
            .take(cap.height as usize)
            .flat_map(|r| &r[..row])
            .copied()
            .collect()
    };

    if let Some(parent) = std::path::Path::new(&cap.path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    match image::save_buffer(
        &cap.path,
        &tight,
        cap.width,
        cap.height,
        image::ExtendedColorType::Rgba8,
    ) {
        Ok(()) => {
            let bytes = std::fs::metadata(&cap.path).map(|m| m.len()).unwrap_or(0);
            println!(
                "[view-forms] WROTE PNG {} ({}x{}, {} bytes on disk; {} raw readback bytes)",
                cap.path, cap.width, cap.height, bytes, raw.len()
            );
        }
        Err(e) => eprintln!("[view-forms] PNG encode failed: {e}"),
    }

    exit.write(AppExit::Success);
}
