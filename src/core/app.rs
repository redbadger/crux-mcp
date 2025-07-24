use crux_core::{Command, macros::effect};

use crate::core::capabilities::{
    render::{self, RenderRequest},
    resolve::{ResolveRequest, ResolveResponse, call_resolve},
    schema::{SchemaRequest, SchemaResponse, get_schema},
    update::{UpdateRequest, UpdateResponse, call_update},
    view::{ViewRequest, ViewResponse, call_view},
};

#[derive(Default)]
pub struct Model;

pub enum Event {
    // from shell
    Schema(SchemaRequest),
    Update(UpdateRequest),
    Resolve(ResolveRequest),
    View(ViewRequest),

    // internal
    GotSchema(SchemaResponse),
    GotUpdate(UpdateResponse),
    GotResolve(ResolveResponse),
    GotView(ViewResponse),
}

#[effect]
pub enum Effect {
    Schema(SchemaRequest),
    Update(UpdateRequest),
    Resolve(ResolveRequest),
    View(ViewRequest),
    Render(RenderRequest),
}

#[derive(Default)]
pub struct App;

impl crux_core::App for App {
    type Model = Model;
    type Event = Event;
    type ViewModel = ();
    type Capabilities = ();
    type Effect = Effect;

    fn update(
        &self,
        event: Self::Event,
        _model: &mut Self::Model,
        _caps: &Self::Capabilities,
    ) -> Command<Effect, Event> {
        match event {
            Event::Schema(_) => get_schema().then_send(Event::GotSchema),
            Event::Update(request) => call_update(request.data).then_send(Event::GotUpdate),
            Event::Resolve(request) => call_resolve(request).then_send(Event::GotResolve),
            Event::View(_) => call_view().then_send(Event::GotView),
            Event::GotSchema(SchemaResponse(result))
            | Event::GotUpdate(UpdateResponse(result))
            | Event::GotResolve(ResolveResponse(result))
            | Event::GotView(ViewResponse(result)) => render::render(result),
        }
    }

    fn view(&self, _model: &Self::Model) -> Self::ViewModel {}
}
