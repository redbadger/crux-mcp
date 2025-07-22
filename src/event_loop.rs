use tokio::sync::mpsc::Sender;

use crate::{
    Error,
    core::{
        app::{App, Effect, Event},
        capabilities::{
            resolve::ResolveResponse, schema::SchemaResponse, update::UpdateResponse,
            view::ViewResponse,
        },
    },
    error::Result,
    shell::host::Host,
};

pub struct Core {
    core: crux_core::Core<App>,
    host: Host,
}

impl Core {
    pub fn new(host: Host) -> Self {
        Self {
            core: crux_core::Core::new(),
            host,
        }
    }

    pub fn update(&self, event: Event, tx: &Sender<Result<Vec<u8>>>) {
        for effect in self.core.process_event(event) {
            self.process_effect(effect, tx);
        }
    }

    fn process_effect(&self, effect: Effect, tx: &Sender<Result<Vec<u8>>>) {
        let effects = match effect {
            Effect::Schema(mut request) => {
                let result = self.host.schema().map_err(Error::Other);
                self.core
                    .resolve(&mut request, SchemaResponse(result))
                    .expect("should resolve")
            }
            Effect::Update(request) => {
                let (request, mut handler) = request.split();
                let result = self.host.update(request.data).map_err(Error::Other);
                self.core
                    .resolve(&mut handler, UpdateResponse(result))
                    .expect("should resolve")
            }
            Effect::Resolve(request) => {
                let (request, mut handler) = request.split();
                let result = self
                    .host
                    .resolve(request.effect_id, request.data)
                    .map_err(Error::Other);
                self.core
                    .resolve(&mut handler, ResolveResponse(result))
                    .expect("should resolve")
            }
            Effect::View(mut request) => {
                let result = self.host.view().map_err(Error::Other);
                self.core
                    .resolve(&mut request, ViewResponse(result))
                    .expect("should resolve")
            }
            Effect::Render(request) => {
                let (request, _) = request.split();
                let tx = tx.clone();
                tokio::spawn(async move {
                    tx.send(request.0).await.expect("receiver dropped");
                });
                vec![]
            }
        };

        for effect in effects {
            self.process_effect(effect, tx);
        }
    }
}
