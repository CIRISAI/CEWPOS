//! cewpos-node — Full-stack CEWPOS node consuming `ciris-server` (`tag = "v0.5.245"`) directly as a crate.
//!
//! Implements [`ciris_server::Adapter`] and boots [`ciris_server::serve_with_adapter`] over the
//! single shared `ciris_server::ciris_persist::prelude::Engine` (`v59.1.2`) and `ciris-edge` (`v43.4.1`),
//! while mounting CEWPOS's 3D Bevy / `CIRISHolon` (`holon-render-3d`) deferral-vote projection routes
//! (`/cewpos/v1/wizard-deferrals` and `/cewpos/v1/scene`) onto the same listener.

use std::path::PathBuf;

use axum::{routing::get, Json, Router};
use ciris_server::{
    Adapter, AdapterConfig, AdapterContext, AdapterStatus, ServerConfig, serve_with_adapter,
};
use view_forms_spike::{
    algebra::{FirstRunWizardState, ViewerContext},
    csd5::{
        build_all_24_primitives_showcase_screen, compile_csd5_screen,
        compile_epistemic_scene_with_holon_draw, CSD5_DRAFT, CSD5_PINNED_COMMIT, CSD5_PINNED_TAG,
        CSD5_STANDARD,
    },
    epistemic_scene::{project_epistemic_scene, project_first_run_wizard_scene},
    view_form::view_deferral_vote,
};
use holon_draw::{ui::FakeText, Camera};

pub struct CewposFullStackAdapter;

#[async_trait::async_trait]
impl Adapter for CewposFullStackAdapter {
    fn adapter_config(&self) -> AdapterConfig {
        AdapterConfig {
            adapter_type: "cewpos-holon-3d".to_string(),
            enabled: true,
        }
    }

    fn status(&self) -> AdapterStatus {
        AdapterStatus {
            adapter_id: "cewpos-holon-3d-v0.5.245".to_string(),
            running: true,
        }
    }

    fn routers(&self, ctx: &AdapterContext) -> Vec<Router> {
        let key_id = ctx.key_id.clone();
        let router = Router::new()
            .route(
                "/cewpos/v1/wizard-deferrals",
                get(move || {
                    let node_key = key_id.clone();
                    async move {
                        let wiz = FirstRunWizardState::default();
                        let votes = wiz.as_deferral_votes();
                        let active = wiz.active_deferral_vote();
                        let (scene_3d, holon_frame) = view_deferral_vote(&active);
                        Json(serde_json::json!({
                            "server_tag": "v0.5.245",
                            "node_key_id": node_key,
                            "active_deferral_vote": active,
                            "holon_frame": holon_frame,
                            "scene_3d": scene_3d,
                            "deferral_series": votes,
                        }))
                    }
                }),
            )
            .route(
                "/cewpos/v1/scene",
                get(|| async move {
                    let viewer = ViewerContext::default();
                    let wiz = FirstRunWizardState::default();
                    let wiz_scene = project_first_run_wizard_scene(&wiz, &viewer);
                    let app_scene = project_epistemic_scene(&viewer, &[], &[]);
                    let (wiz_csd5, wiz_holon) =
                        compile_epistemic_scene_with_holon_draw(&wiz_scene, "wizard", 1, 1.0);
                    let cam = Camera::orbit([0.0, 0.0, 0.0], 0.5, 0.35, 18.0, 0.8, 1280.0, 900.0);
                    let wiz_frame = wiz_holon
                        .evaluate_frame(&cam, 1.0, u64::MAX)
                        .expect("finite holon-draw frame");
                    Json(serde_json::json!({
                        "wizard_scene": wiz_scene,
                        "epistemic_scene": app_scene,
                        "csd5_wizard_nodes": wiz_csd5,
                        "holon_draw_summary": wiz_holon.export_summary(&wiz_frame),
                    }))
                }),
            )
            .route(
                "/cewpos/v1/csd5-holon-draw",
                get(|| async move {
                    let showcase = build_all_24_primitives_showcase_screen("showcase");
                    let mut text = FakeText {
                        dpr: 1.0,
                        placed: 0,
                    };
                    let compiled = compile_csd5_screen(&showcase, &mut text, 1);
                    let cam =
                        Camera::orbit([0.0, 0.0, 0.0], 0.6, 0.4, 15.0, 0.8, 1280.0, 2200.0);
                    let frame = compiled
                        .evaluate_frame(&cam, 1.0, u64::MAX)
                        .expect("finite showcase frame");
                    Json(serde_json::json!({
                        "standard": CSD5_STANDARD,
                        "draft": CSD5_DRAFT,
                        "pinned_tag": CSD5_PINNED_TAG,
                        "pinned_commit": CSD5_PINNED_COMMIT,
                        "showcase_summary": compiled.export_summary(&frame),
                    }))
                }),
            );
        vec![router]
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let home = std::env::var("CIRIS_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/tmp/ciris_cewpos_243"));
    let key_id = std::env::var("CIRIS_KEY_ID").unwrap_or_else(|_| "cewpos-243-node".to_string());
    ciris_server::init_tracing_with(Some(&home.join("logs")));
    let mut cfg = ServerConfig::from_home(home, key_id)?;
    if let Ok(addr_str) = std::env::var("CIRIS_LISTEN_ADDR") {
        if let Ok(addr) = addr_str.parse::<std::net::SocketAddr>() {
            cfg.listen_addr = addr;
        }
    }
    serve_with_adapter(cfg, std::sync::Arc::new(CewposFullStackAdapter)).await
}
