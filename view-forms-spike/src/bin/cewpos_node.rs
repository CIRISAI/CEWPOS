//! cewpos-node — Full-stack CEWPOS node consuming `ciris-server` (`tag = "v0.5.243"`) directly as a crate.
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
    epistemic_scene::{project_epistemic_scene, project_first_run_wizard_scene},
    view_form::view_deferral_vote,
};

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
            adapter_id: "cewpos-holon-3d-v0.5.243".to_string(),
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
                            "server_tag": "v0.5.243",
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
                    Json(serde_json::json!({
                        "wizard_scene": wiz_scene,
                        "epistemic_scene": app_scene,
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
    let cfg = ServerConfig::from_home(home, key_id)?;
    serve_with_adapter(cfg, std::sync::Arc::new(CewposFullStackAdapter)).await
}
