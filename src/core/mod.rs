use std::sync::Arc;

use tokio::sync::mpsc::Sender;

use crate::core::capabilities::{
    resolve::ResolveResponse, schema::SchemaResponse, update::UpdateResponse, view::ViewResponse,
};
use crate::{
    core::app::{App, Effect, Event},
    shell::host::Host,
};

pub mod app;
pub mod capabilities;

pub type Core = Arc<crux_core::Core<App>>;

pub fn new() -> Core {
    Arc::new(crux_core::Core::new())
}

pub fn update(core: &Core, host: &Host, event: Event, tx: &Sender<Result<Vec<u8>, String>>) {
    for effect in core.process_event(event) {
        process_effect(core, host, effect, tx);
    }
}

pub fn process_effect(
    core: &Core,
    host: &Host,
    effect: Effect,
    tx: &Sender<Result<Vec<u8>, String>>,
) {
    match effect {
        Effect::Schema(mut request) => {
            let response = host.schema().map_err(|e| e.to_string());
            for effect in core
                .resolve(&mut request, SchemaResponse(response))
                .expect("should resolve")
            {
                process_effect(core, host, effect, tx);
            }
        }
        Effect::Update(request) => {
            let (request, mut handler) = request.split();
            let response = host
                .update(request.data.into_bytes())
                .map_err(|e| e.to_string());
            for effect in core
                .resolve(&mut handler, UpdateResponse(response))
                .expect("should resolve")
            {
                process_effect(core, host, effect, tx);
            }
        }
        Effect::Resolve(request) => {
            let (request, mut handler) = request.split();
            let response = host
                .resolve(request.effect_id, request.data.into_bytes())
                .map_err(|e| e.to_string());
            for effect in core
                .resolve(&mut handler, ResolveResponse(response))
                .expect("should resolve")
            {
                process_effect(core, host, effect, tx);
            }
        }
        Effect::View(mut request) => {
            let response = host.view().map_err(|e| e.to_string());
            for effect in core
                .resolve(&mut request, ViewResponse(response))
                .expect("should resolve")
            {
                process_effect(core, host, effect, tx);
            }
        }
        Effect::Render(request) => {
            let (request, _) = request.split();
            let tx = tx.clone();
            tokio::spawn(async move {
                tx.send(request.0).await.expect("receiver dropped");
            });
        }
    }
}
