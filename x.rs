#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use rust_mcp_sdk::{
    McpServer, StdioTransport, TransportOptions, error::SdkResult,
    mcp_server::server_runtime,
    schema::{
        Implementation, InitializeResult, LATEST_PROTOCOL_VERSION, ServerCapabilities,
        ServerCapabilitiesTools,
    },
};
use crate::{event_loop::Core, shell::{host::Host, mcp::handler::MyServerHandler}};
mod core {
    pub mod app {
        use crux_core::{Command, macros::effect};
        use crate::core::capabilities::{
            render::{self, RenderRequest},
            resolve::{ResolveRequest, ResolveResponse, call_resolve},
            schema::{SchemaRequest, SchemaResponse, get_schema},
            update::{UpdateRequest, UpdateResponse, call_update},
            view::{ViewRequest, ViewResponse, call_view},
        };
        pub struct Model;
        #[automatically_derived]
        impl ::core::default::Default for Model {
            #[inline]
            fn default() -> Model {
                Model {}
            }
        }
        pub enum Event {
            Schema(SchemaRequest),
            Update(UpdateRequest),
            Resolve(ResolveRequest),
            View(ViewRequest),
            GotSchema(SchemaResponse),
            GotUpdate(UpdateResponse),
            GotResolve(ResolveResponse),
            GotView(ViewResponse),
        }
        pub enum Effect {
            Schema(::crux_core::Request<SchemaRequest>),
            Update(::crux_core::Request<UpdateRequest>),
            Resolve(::crux_core::Request<ResolveRequest>),
            View(::crux_core::Request<ViewRequest>),
            Render(::crux_core::Request<RenderRequest>),
        }
        impl crux_core::Effect for Effect {}
        impl From<::crux_core::Request<SchemaRequest>> for Effect {
            fn from(value: ::crux_core::Request<SchemaRequest>) -> Self {
                Self::Schema(value)
            }
        }
        impl TryFrom<Effect> for ::crux_core::Request<SchemaRequest> {
            type Error = Effect;
            fn try_from(value: Effect) -> Result<Self, Self::Error> {
                if let Effect::Schema(value) = value { Ok(value) } else { Err(value) }
            }
        }
        impl From<::crux_core::Request<UpdateRequest>> for Effect {
            fn from(value: ::crux_core::Request<UpdateRequest>) -> Self {
                Self::Update(value)
            }
        }
        impl TryFrom<Effect> for ::crux_core::Request<UpdateRequest> {
            type Error = Effect;
            fn try_from(value: Effect) -> Result<Self, Self::Error> {
                if let Effect::Update(value) = value { Ok(value) } else { Err(value) }
            }
        }
        impl From<::crux_core::Request<ResolveRequest>> for Effect {
            fn from(value: ::crux_core::Request<ResolveRequest>) -> Self {
                Self::Resolve(value)
            }
        }
        impl TryFrom<Effect> for ::crux_core::Request<ResolveRequest> {
            type Error = Effect;
            fn try_from(value: Effect) -> Result<Self, Self::Error> {
                if let Effect::Resolve(value) = value { Ok(value) } else { Err(value) }
            }
        }
        impl From<::crux_core::Request<ViewRequest>> for Effect {
            fn from(value: ::crux_core::Request<ViewRequest>) -> Self {
                Self::View(value)
            }
        }
        impl TryFrom<Effect> for ::crux_core::Request<ViewRequest> {
            type Error = Effect;
            fn try_from(value: Effect) -> Result<Self, Self::Error> {
                if let Effect::View(value) = value { Ok(value) } else { Err(value) }
            }
        }
        impl From<::crux_core::Request<RenderRequest>> for Effect {
            fn from(value: ::crux_core::Request<RenderRequest>) -> Self {
                Self::Render(value)
            }
        }
        impl TryFrom<Effect> for ::crux_core::Request<RenderRequest> {
            type Error = Effect;
            fn try_from(value: Effect) -> Result<Self, Self::Error> {
                if let Effect::Render(value) = value { Ok(value) } else { Err(value) }
            }
        }
        impl Effect {
            pub fn is_schema(&self) -> bool {
                if let Effect::Schema(_) = self { true } else { false }
            }
            pub fn into_schema(self) -> Option<::crux_core::Request<SchemaRequest>> {
                if let Effect::Schema(request) = self { Some(request) } else { None }
            }
            #[track_caller]
            pub fn expect_schema(self) -> ::crux_core::Request<SchemaRequest> {
                if let Effect::Schema(request) = self {
                    request
                } else {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("not a {0} effect", "Schema"),
                        );
                    }
                }
            }
        }
        impl Effect {
            pub fn is_update(&self) -> bool {
                if let Effect::Update(_) = self { true } else { false }
            }
            pub fn into_update(self) -> Option<::crux_core::Request<UpdateRequest>> {
                if let Effect::Update(request) = self { Some(request) } else { None }
            }
            #[track_caller]
            pub fn expect_update(self) -> ::crux_core::Request<UpdateRequest> {
                if let Effect::Update(request) = self {
                    request
                } else {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("not a {0} effect", "Update"),
                        );
                    }
                }
            }
        }
        impl Effect {
            pub fn is_resolve(&self) -> bool {
                if let Effect::Resolve(_) = self { true } else { false }
            }
            pub fn into_resolve(self) -> Option<::crux_core::Request<ResolveRequest>> {
                if let Effect::Resolve(request) = self { Some(request) } else { None }
            }
            #[track_caller]
            pub fn expect_resolve(self) -> ::crux_core::Request<ResolveRequest> {
                if let Effect::Resolve(request) = self {
                    request
                } else {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("not a {0} effect", "Resolve"),
                        );
                    }
                }
            }
        }
        impl Effect {
            pub fn is_view(&self) -> bool {
                if let Effect::View(_) = self { true } else { false }
            }
            pub fn into_view(self) -> Option<::crux_core::Request<ViewRequest>> {
                if let Effect::View(request) = self { Some(request) } else { None }
            }
            #[track_caller]
            pub fn expect_view(self) -> ::crux_core::Request<ViewRequest> {
                if let Effect::View(request) = self {
                    request
                } else {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("not a {0} effect", "View"),
                        );
                    }
                }
            }
        }
        impl Effect {
            pub fn is_render(&self) -> bool {
                if let Effect::Render(_) = self { true } else { false }
            }
            pub fn into_render(self) -> Option<::crux_core::Request<RenderRequest>> {
                if let Effect::Render(request) = self { Some(request) } else { None }
            }
            #[track_caller]
            pub fn expect_render(self) -> ::crux_core::Request<RenderRequest> {
                if let Effect::Render(request) = self {
                    request
                } else {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("not a {0} effect", "Render"),
                        );
                    }
                }
            }
        }
        pub struct App;
        #[automatically_derived]
        impl ::core::default::Default for App {
            #[inline]
            fn default() -> App {
                App {}
            }
        }
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
                    Event::Update(request) => {
                        call_update(request.data).then_send(Event::GotUpdate)
                    }
                    Event::Resolve(request) => {
                        call_resolve(request).then_send(Event::GotResolve)
                    }
                    Event::View(_) => call_view().then_send(Event::GotView),
                    Event::GotSchema(SchemaResponse(result))
                    | Event::GotUpdate(UpdateResponse(result))
                    | Event::GotResolve(ResolveResponse(result))
                    | Event::GotView(ViewResponse(result)) => render::render(result),
                }
            }
            fn view(&self, _model: &Self::Model) -> Self::ViewModel {}
        }
    }
    pub mod capabilities {
        pub mod render {
            use crux_core::{Command, Request, capability::Operation};
            use crate::Result;
            pub struct RenderRequest(pub Result<Vec<u8>>);
            impl Operation for RenderRequest {
                type Output = ();
            }
            pub fn render<Effect, Event>(
                result: Result<Vec<u8>>,
            ) -> Command<Effect, Event>
            where
                Effect: From<Request<RenderRequest>> + Send + 'static,
                Event: Send + 'static,
            {
                Command::notify_shell(RenderRequest(result)).build()
            }
        }
        pub mod resolve {
            use crux_core::{
                Command, Request, capability::Operation, command::RequestBuilder,
            };
            use crate::Result;
            pub struct ResolveRequest {
                pub effect_id: u32,
                pub data: Vec<u8>,
            }
            pub struct ResolveResponse(pub Result<Vec<u8>>);
            impl Operation for ResolveRequest {
                type Output = ResolveResponse;
            }
            pub fn call_resolve<Effect, Event>(
                req: ResolveRequest,
            ) -> RequestBuilder<Effect, Event, impl Future<Output = ResolveResponse>>
            where
                Effect: From<Request<ResolveRequest>> + Send + 'static,
                Event: Send + 'static,
            {
                Command::request_from_shell(req)
            }
        }
        pub mod schema {
            use crux_core::{
                Command, Request, capability::Operation, command::RequestBuilder,
            };
            use crate::Result;
            pub struct SchemaRequest;
            pub struct SchemaResponse(pub Result<Vec<u8>>);
            impl Operation for SchemaRequest {
                type Output = SchemaResponse;
            }
            pub fn get_schema<Effect, Event>() -> RequestBuilder<
                Effect,
                Event,
                impl Future<Output = SchemaResponse>,
            >
            where
                Effect: From<Request<SchemaRequest>> + Send + 'static,
                Event: Send + 'static,
            {
                Command::request_from_shell(SchemaRequest)
            }
        }
        pub mod update {
            use crux_core::{
                Command, Request, capability::Operation, command::RequestBuilder,
            };
            use crate::Result;
            pub struct UpdateRequest {
                pub data: Vec<u8>,
            }
            pub struct UpdateResponse(pub Result<Vec<u8>>);
            impl Operation for UpdateRequest {
                type Output = UpdateResponse;
            }
            pub fn call_update<Effect, Event>(
                data: Vec<u8>,
            ) -> RequestBuilder<Effect, Event, impl Future<Output = UpdateResponse>>
            where
                Effect: From<Request<UpdateRequest>> + Send + 'static,
                Event: Send + 'static,
            {
                Command::request_from_shell(UpdateRequest { data })
            }
        }
        pub mod view {
            use crux_core::{
                Command, Request, capability::Operation, command::RequestBuilder,
            };
            use crate::Result;
            pub struct ViewRequest;
            pub struct ViewResponse(pub Result<Vec<u8>>);
            impl Operation for ViewRequest {
                type Output = ViewResponse;
            }
            pub fn call_view<Effect, Event>() -> RequestBuilder<
                Effect,
                Event,
                impl Future<Output = ViewResponse>,
            >
            where
                Effect: From<Request<ViewRequest>> + Send + 'static,
                Event: Send + 'static,
            {
                Command::request_from_shell(ViewRequest)
            }
        }
    }
}
mod error {
    use thiserror::Error;
    pub enum Error {
        #[error("channel closed")]
        ChannelClosed,
        #[error(transparent)]
        Other(#[from] anyhow::Error),
    }
    #[allow(unused_qualifications)]
    #[automatically_derived]
    impl ::thiserror::__private::Error for Error {
        fn source(
            &self,
        ) -> ::core::option::Option<&(dyn ::thiserror::__private::Error + 'static)> {
            use ::thiserror::__private::AsDynError as _;
            #[allow(deprecated)]
            match self {
                Error::ChannelClosed { .. } => ::core::option::Option::None,
                Error::Other { 0: transparent } => {
                    ::thiserror::__private::Error::source(transparent.as_dyn_error())
                }
            }
        }
    }
    #[allow(unused_qualifications)]
    #[automatically_derived]
    impl ::core::fmt::Display for Error {
        fn fmt(&self, __formatter: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            #[allow(unused_variables, deprecated, clippy::used_underscore_binding)]
            match self {
                Error::ChannelClosed {} => __formatter.write_str("channel closed"),
                Error::Other(_0) => ::core::fmt::Display::fmt(_0, __formatter),
            }
        }
    }
    #[allow(
        deprecated,
        unused_qualifications,
        clippy::elidable_lifetime_names,
        clippy::needless_lifetimes,
    )]
    #[automatically_derived]
    impl ::core::convert::From<anyhow::Error> for Error {
        fn from(source: anyhow::Error) -> Self {
            Error::Other { 0: source }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Error {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Error::ChannelClosed => {
                    ::core::fmt::Formatter::write_str(f, "ChannelClosed")
                }
                Error::Other(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Other",
                        &__self_0,
                    )
                }
            }
        }
    }
    pub type Result<T> = std::result::Result<T, Error>;
}
mod event_loop {
    use std::string::String;
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
        error::Result, shell::host::Host,
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
                    let result = self
                        .host
                        .schema()
                        .map(String::into_bytes)
                        .map_err(Error::Other);
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
                    ::alloc::vec::Vec::new()
                }
            };
            for effect in effects {
                self.process_effect(effect, tx);
            }
        }
    }
}
mod shell {
    pub mod host {
        use std::{env, path::PathBuf, sync::{Arc, Mutex}};
        use anyhow::{Result, anyhow, bail};
        use wasmtime::{
            Engine, Store,
            component::{Component, Linker, ResourceTable, TypedFunc, bindgen},
        };
        use wasmtime_wasi::p2::{
            IoView, WasiCtx, WasiCtxBuilder, WasiView, add_to_linker_sync,
        };
        /// Auto-generated bindings for a pre-instantiated version of a
        /// component which implements the world `shared`.
        ///
        /// This structure is created through [`SharedPre::new`] which
        /// takes a [`InstancePre`](wasmtime::component::InstancePre) that
        /// has been created through a [`Linker`](wasmtime::component::Linker).
        ///
        /// For more information see [`Shared`] as well.
        pub struct SharedPre<T: 'static> {
            instance_pre: wasmtime::component::InstancePre<T>,
            indices: SharedIndices,
        }
        impl<T: 'static> Clone for SharedPre<T> {
            fn clone(&self) -> Self {
                Self {
                    instance_pre: self.instance_pre.clone(),
                    indices: self.indices.clone(),
                }
            }
        }
        impl<_T: 'static> SharedPre<_T> {
            /// Creates a new copy of `SharedPre` bindings which can then
            /// be used to instantiate into a particular store.
            ///
            /// This method may fail if the component behind `instance_pre`
            /// does not have the required exports.
            pub fn new(
                instance_pre: wasmtime::component::InstancePre<_T>,
            ) -> wasmtime::Result<Self> {
                let indices = SharedIndices::new(&instance_pre)?;
                Ok(Self { instance_pre, indices })
            }
            pub fn engine(&self) -> &wasmtime::Engine {
                self.instance_pre.engine()
            }
            pub fn instance_pre(&self) -> &wasmtime::component::InstancePre<_T> {
                &self.instance_pre
            }
            /// Instantiates a new instance of [`Shared`] within the
            /// `store` provided.
            ///
            /// This function will use `self` as the pre-instantiated
            /// instance to perform instantiation. Afterwards the preloaded
            /// indices in `self` are used to lookup all exports on the
            /// resulting instance.
            pub fn instantiate(
                &self,
                mut store: impl wasmtime::AsContextMut<Data = _T>,
            ) -> wasmtime::Result<Shared> {
                let mut store = store.as_context_mut();
                let instance = self.instance_pre.instantiate(&mut store)?;
                self.indices.load(&mut store, &instance)
            }
        }
        /// Auto-generated bindings for index of the exports of
        /// `shared`.
        ///
        /// This is an implementation detail of [`SharedPre`] and can
        /// be constructed if needed as well.
        ///
        /// For more information see [`Shared`] as well.
        pub struct SharedIndices {}
        #[automatically_derived]
        impl ::core::clone::Clone for SharedIndices {
            #[inline]
            fn clone(&self) -> SharedIndices {
                SharedIndices {}
            }
        }
        /// Auto-generated bindings for an instance a component which
        /// implements the world `shared`.
        ///
        /// This structure can be created through a number of means
        /// depending on your requirements and what you have on hand:
        ///
        /// * The most convenient way is to use
        ///   [`Shared::instantiate`] which only needs a
        ///   [`Store`], [`Component`], and [`Linker`].
        ///
        /// * Alternatively you can create a [`SharedPre`] ahead of
        ///   time with a [`Component`] to front-load string lookups
        ///   of exports once instead of per-instantiation. This
        ///   method then uses [`SharedPre::instantiate`] to
        ///   create a [`Shared`].
        ///
        /// * If you've instantiated the instance yourself already
        ///   then you can use [`Shared::new`].
        ///
        /// These methods are all equivalent to one another and move
        /// around the tradeoff of what work is performed when.
        ///
        /// [`Store`]: wasmtime::Store
        /// [`Component`]: wasmtime::component::Component
        /// [`Linker`]: wasmtime::component::Linker
        pub struct Shared {}
        const _: () = {
            #[allow(unused_imports)]
            use wasmtime::component::__internal::anyhow;
            impl SharedIndices {
                /// Creates a new copy of `SharedIndices` bindings which can then
                /// be used to instantiate into a particular store.
                ///
                /// This method may fail if the component does not have the
                /// required exports.
                pub fn new<_T>(
                    _instance_pre: &wasmtime::component::InstancePre<_T>,
                ) -> wasmtime::Result<Self> {
                    let _component = _instance_pre.component();
                    let _instance_type = _instance_pre.instance_type();
                    Ok(SharedIndices {})
                }
                /// Uses the indices stored in `self` to load an instance
                /// of [`Shared`] from the instance provided.
                ///
                /// Note that at this time this method will additionally
                /// perform type-checks of all exports.
                pub fn load(
                    &self,
                    mut store: impl wasmtime::AsContextMut,
                    instance: &wasmtime::component::Instance,
                ) -> wasmtime::Result<Shared> {
                    let _ = &mut store;
                    let _instance = instance;
                    Ok(Shared {})
                }
            }
            impl Shared {
                /// Convenience wrapper around [`SharedPre::new`] and
                /// [`SharedPre::instantiate`].
                pub fn instantiate<_T>(
                    store: impl wasmtime::AsContextMut<Data = _T>,
                    component: &wasmtime::component::Component,
                    linker: &wasmtime::component::Linker<_T>,
                ) -> wasmtime::Result<Shared> {
                    let pre = linker.instantiate_pre(component)?;
                    SharedPre::new(pre)?.instantiate(store)
                }
                /// Convenience wrapper around [`SharedIndices::new`] and
                /// [`SharedIndices::load`].
                pub fn new(
                    mut store: impl wasmtime::AsContextMut,
                    instance: &wasmtime::component::Instance,
                ) -> wasmtime::Result<Shared> {
                    let indices = SharedIndices::new(&instance.instance_pre(&store))?;
                    indices.load(&mut store, instance)
                }
                pub fn add_to_linker<T, D>(
                    linker: &mut wasmtime::component::Linker<T>,
                    host_getter: fn(&mut T) -> D::Data<'_>,
                ) -> wasmtime::Result<()>
                where
                    D: wasmtime::component::HasData,
                    for<'a> D::Data<'a>: crux::shared_lib::core::Host,
                    T: 'static,
                {
                    crux::shared_lib::core::add_to_linker::<T, D>(linker, host_getter)?;
                    Ok(())
                }
            }
        };
        pub mod crux {
            pub mod shared_lib {
                #[allow(clippy::all)]
                pub mod core {
                    #[allow(unused_imports)]
                    use wasmtime::component::__internal::{anyhow, Box};
                    pub enum Instance {}
                    pub trait HostInstance {
                        fn new(&mut self) -> wasmtime::component::Resource<Instance>;
                        fn update(
                            &mut self,
                            self_: wasmtime::component::Resource<Instance>,
                            data: wasmtime::component::__internal::Vec<u8>,
                        ) -> Result<
                            wasmtime::component::__internal::Vec<u8>,
                            wasmtime::component::__internal::String,
                        >;
                        fn resolve(
                            &mut self,
                            self_: wasmtime::component::Resource<Instance>,
                            effect_id: u32,
                            data: wasmtime::component::__internal::Vec<u8>,
                        ) -> Result<
                            wasmtime::component::__internal::Vec<u8>,
                            wasmtime::component::__internal::String,
                        >;
                        fn view(
                            &mut self,
                            self_: wasmtime::component::Resource<Instance>,
                        ) -> Result<
                            wasmtime::component::__internal::Vec<u8>,
                            wasmtime::component::__internal::String,
                        >;
                        fn schema(
                            &mut self,
                            self_: wasmtime::component::Resource<Instance>,
                        ) -> wasmtime::component::__internal::String;
                        fn drop(
                            &mut self,
                            rep: wasmtime::component::Resource<Instance>,
                        ) -> wasmtime::Result<()>;
                    }
                    impl<_T: HostInstance + ?Sized> HostInstance for &mut _T {
                        fn new(&mut self) -> wasmtime::component::Resource<Instance> {
                            HostInstance::new(*self)
                        }
                        fn update(
                            &mut self,
                            self_: wasmtime::component::Resource<Instance>,
                            data: wasmtime::component::__internal::Vec<u8>,
                        ) -> Result<
                            wasmtime::component::__internal::Vec<u8>,
                            wasmtime::component::__internal::String,
                        > {
                            HostInstance::update(*self, self_, data)
                        }
                        fn resolve(
                            &mut self,
                            self_: wasmtime::component::Resource<Instance>,
                            effect_id: u32,
                            data: wasmtime::component::__internal::Vec<u8>,
                        ) -> Result<
                            wasmtime::component::__internal::Vec<u8>,
                            wasmtime::component::__internal::String,
                        > {
                            HostInstance::resolve(*self, self_, effect_id, data)
                        }
                        fn view(
                            &mut self,
                            self_: wasmtime::component::Resource<Instance>,
                        ) -> Result<
                            wasmtime::component::__internal::Vec<u8>,
                            wasmtime::component::__internal::String,
                        > {
                            HostInstance::view(*self, self_)
                        }
                        fn schema(
                            &mut self,
                            self_: wasmtime::component::Resource<Instance>,
                        ) -> wasmtime::component::__internal::String {
                            HostInstance::schema(*self, self_)
                        }
                        fn drop(
                            &mut self,
                            rep: wasmtime::component::Resource<Instance>,
                        ) -> wasmtime::Result<()> {
                            HostInstance::drop(*self, rep)
                        }
                    }
                    pub trait Host: HostInstance {}
                    impl<_T: Host + ?Sized> Host for &mut _T {}
                    pub fn add_to_linker<T, D>(
                        linker: &mut wasmtime::component::Linker<T>,
                        host_getter: fn(&mut T) -> D::Data<'_>,
                    ) -> wasmtime::Result<()>
                    where
                        D: wasmtime::component::HasData,
                        for<'a> D::Data<'a>: Host,
                        T: 'static,
                    {
                        let mut inst = linker.instance("crux:shared-lib/core")?;
                        inst.resource(
                            "instance",
                            wasmtime::component::ResourceType::host::<Instance>(),
                            move |mut store, rep| -> wasmtime::Result<()> {
                                HostInstance::drop(
                                    &mut host_getter(store.data_mut()),
                                    wasmtime::component::Resource::new_own(rep),
                                )
                            },
                        )?;
                        inst.func_wrap(
                            "[constructor]instance",
                            move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                                let host = &mut host_getter(caller.data_mut());
                                let r = HostInstance::new(host);
                                Ok((r,))
                            },
                        )?;
                        inst.func_wrap(
                            "[method]instance.update",
                            move |
                                mut caller: wasmtime::StoreContextMut<'_, T>,
                                (
                                    arg0,
                                    arg1,
                                ): (
                                    wasmtime::component::Resource<Instance>,
                                    wasmtime::component::__internal::Vec<u8>,
                                )|
                            {
                                let host = &mut host_getter(caller.data_mut());
                                let r = HostInstance::update(host, arg0, arg1);
                                Ok((r,))
                            },
                        )?;
                        inst.func_wrap(
                            "[method]instance.resolve",
                            move |
                                mut caller: wasmtime::StoreContextMut<'_, T>,
                                (
                                    arg0,
                                    arg1,
                                    arg2,
                                ): (
                                    wasmtime::component::Resource<Instance>,
                                    u32,
                                    wasmtime::component::__internal::Vec<u8>,
                                )|
                            {
                                let host = &mut host_getter(caller.data_mut());
                                let r = HostInstance::resolve(host, arg0, arg1, arg2);
                                Ok((r,))
                            },
                        )?;
                        inst.func_wrap(
                            "[method]instance.view",
                            move |
                                mut caller: wasmtime::StoreContextMut<'_, T>,
                                (arg0,): (wasmtime::component::Resource<Instance>,)|
                            {
                                let host = &mut host_getter(caller.data_mut());
                                let r = HostInstance::view(host, arg0);
                                Ok((r,))
                            },
                        )?;
                        inst.func_wrap(
                            "[method]instance.schema",
                            move |
                                mut caller: wasmtime::StoreContextMut<'_, T>,
                                (arg0,): (wasmtime::component::Resource<Instance>,)|
                            {
                                let host = &mut host_getter(caller.data_mut());
                                let r = HostInstance::schema(host, arg0);
                                Ok((r,))
                            },
                        )?;
                        Ok(())
                    }
                }
            }
        }
        const _: &str = "package crux:shared-lib;\n\nworld shared {\n    import core;\n}\n";
        const _: &str = "package crux:shared-lib;\n\ninterface core {\n    resource instance {\n        constructor();\n        update: func(data: list<u8>) -> result<list<u8>, string>;\n        resolve: func(effect-id: u32, data: list<u8>) -> result<list<u8>, string>;\n        view: func() -> result<list<u8>, string>;\n        schema: func() -> string;\n    }\n}\n";
        pub struct ComponentRunStates {
            pub wasi_ctx: WasiCtx,
            pub resource_table: ResourceTable,
        }
        impl IoView for ComponentRunStates {
            fn table(&mut self) -> &mut ResourceTable {
                &mut self.resource_table
            }
        }
        impl WasiView for ComponentRunStates {
            fn ctx(&mut self) -> &mut WasiCtx {
                &mut self.wasi_ctx
            }
        }
        type Bytes = Vec<u8>;
        #[allow(clippy::type_complexity)]
        pub struct Host {
            store: Arc<Mutex<Option<Store<ComponentRunStates>>>>,
            schema: Option<TypedFunc<(), (String,)>>,
            update: Option<TypedFunc<(Bytes,), (Result<Bytes, String>,)>>,
            resolve: Option<TypedFunc<(u32, Bytes), (Result<Bytes, String>,)>>,
            view: Option<TypedFunc<(), (Result<Bytes, String>,)>>,
        }
        #[automatically_derived]
        #[allow(clippy::type_complexity)]
        impl ::core::default::Default for Host {
            #[inline]
            fn default() -> Host {
                Host {
                    store: ::core::default::Default::default(),
                    schema: ::core::default::Default::default(),
                    update: ::core::default::Default::default(),
                    resolve: ::core::default::Default::default(),
                    view: ::core::default::Default::default(),
                }
            }
        }
        impl Host {
            pub fn load(&mut self) -> Result<()> {
                let engine = Engine::default();
                let mut linker = Linker::new(&engine);
                add_to_linker_sync(&mut linker)?;
                let state = ComponentRunStates {
                    wasi_ctx: WasiCtxBuilder::new()
                        .inherit_stdio()
                        .inherit_args()
                        .build(),
                    resource_table: ResourceTable::new(),
                };
                let mut store = Store::new(&engine, state);
                let file = PathBuf::from(env::var("CRUX_COMPONENT")?);
                let component = Component::from_file(&engine, file)?;
                let instance = linker.instantiate(&mut store, &component)?;
                if let Some(func) = instance.get_func(&mut store, "schema") {
                    self.schema = Some(func.typed::<(), (String,)>(&store)?);
                } else {
                    return ::anyhow::__private::Err({
                        let error = ::anyhow::__private::format_err(
                            format_args!("schema function not found"),
                        );
                        error
                    });
                }
                if let Some(func) = instance.get_func(&mut store, "update") {
                    self.update = Some(
                        func.typed::<(Bytes,), (Result<Bytes, String>,)>(&store)?,
                    );
                } else {
                    return ::anyhow::__private::Err({
                        let error = ::anyhow::__private::format_err(
                            format_args!("update function not found"),
                        );
                        error
                    });
                }
                if let Some(func) = instance.get_func(&mut store, "resolve") {
                    self.resolve = Some(
                        func.typed::<(u32, Bytes), (Result<Bytes, String>,)>(&store)?,
                    );
                } else {
                    return ::anyhow::__private::Err({
                        let error = ::anyhow::__private::format_err(
                            format_args!("resolve function not found"),
                        );
                        error
                    });
                }
                if let Some(func) = instance.get_func(&mut store, "view") {
                    self.view = Some(
                        func.typed::<(), (Result<Bytes, String>,)>(&store)?,
                    );
                } else {
                    return ::anyhow::__private::Err({
                        let error = ::anyhow::__private::format_err(
                            format_args!("view function not found"),
                        );
                        error
                    });
                }
                self.store = Arc::new(Mutex::new(Some(store)));
                Ok(())
            }
            pub fn schema(&self) -> Result<String> {
                let Some(schema) = self.schema else {
                    return ::anyhow::__private::Err({
                        let error = ::anyhow::__private::format_err(
                            format_args!("no schema function"),
                        );
                        error
                    });
                };
                let mut store = self.store.lock().expect("failed to lock store");
                let Some(mut store) = store.as_mut() else {
                    return ::anyhow::__private::Err({
                        let error = ::anyhow::__private::format_err(
                            format_args!("no store"),
                        );
                        error
                    });
                };
                let (data,) = schema.call(&mut store, ())?;
                schema.post_return(&mut store)?;
                Ok(data)
            }
            pub fn update(&self, data: Vec<u8>) -> Result<Vec<u8>> {
                let Some(update) = self.update else {
                    return ::anyhow::__private::Err({
                        let error = ::anyhow::__private::format_err(
                            format_args!("no update function"),
                        );
                        error
                    });
                };
                let mut store = self.store.lock().expect("failed to lock store");
                let Some(mut store) = store.as_mut() else {
                    return ::anyhow::__private::Err({
                        let error = ::anyhow::__private::format_err(
                            format_args!("no store"),
                        );
                        error
                    });
                };
                let (data,) = update.call(&mut store, (data,))?;
                update.post_return(&mut store)?;
                data.map_err(|e| ::anyhow::__private::must_use({
                    let error = ::anyhow::__private::format_err(
                        format_args!("bridge error: {0}", e),
                    );
                    error
                }))
            }
            pub fn resolve(&self, effect_id: u32, data: Vec<u8>) -> Result<Vec<u8>> {
                let Some(resolve) = self.resolve else {
                    return ::anyhow::__private::Err({
                        let error = ::anyhow::__private::format_err(
                            format_args!("no update function"),
                        );
                        error
                    });
                };
                let mut store = self.store.lock().expect("failed to lock store");
                let Some(mut store) = store.as_mut() else {
                    return ::anyhow::__private::Err({
                        let error = ::anyhow::__private::format_err(
                            format_args!("no store"),
                        );
                        error
                    });
                };
                let (data,) = resolve.call(&mut store, (effect_id, data))?;
                resolve.post_return(&mut store)?;
                data.map_err(|e| ::anyhow::__private::must_use({
                    let error = ::anyhow::__private::format_err(
                        format_args!("bridge error: {0}", e),
                    );
                    error
                }))
            }
            pub fn view(&self) -> Result<Vec<u8>> {
                let Some(view) = &self.view else {
                    return ::anyhow::__private::Err({
                        let error = ::anyhow::__private::format_err(
                            format_args!("no view function"),
                        );
                        error
                    });
                };
                let mut store = self.store.lock().expect("failed to lock store");
                let Some(mut store) = store.as_mut() else {
                    return ::anyhow::__private::Err({
                        let error = ::anyhow::__private::format_err(
                            format_args!("no store"),
                        );
                        error
                    });
                };
                let (data,) = view.call(&mut store, ())?;
                view.post_return(&mut store)?;
                data.map_err(|e| ::anyhow::__private::must_use({
                    let error = ::anyhow::__private::format_err(
                        format_args!("bridge error: {0}", e),
                    );
                    error
                }))
            }
        }
    }
    pub mod mcp {
        pub mod handler {
            use std::result::Result;
            use anyhow::anyhow;
            use async_trait::async_trait;
            use rust_mcp_sdk::schema::{
                CallToolRequest, CallToolResult, ListToolsRequest, ListToolsResult,
                RpcError, schema_utils::CallToolError,
            };
            use rust_mcp_sdk::{McpServer, mcp_server::ServerHandler};
            use tokio::sync::mpsc;
            use crate::Error;
            use crate::event_loop::Core;
            use crate::shell::mcp::MyTools;
            pub struct MyServerHandler {
                core: Core,
            }
            impl MyServerHandler {
                pub fn new(core: Core) -> Self {
                    Self { core }
                }
            }
            #[allow(unused)]
            impl ServerHandler for MyServerHandler {
                #[allow(
                    elided_named_lifetimes,
                    clippy::async_yields_async,
                    clippy::diverging_sub_expression,
                    clippy::let_unit_value,
                    clippy::needless_arbitrary_self_type,
                    clippy::no_effect_underscore_binding,
                    clippy::shadow_same,
                    clippy::type_complexity,
                    clippy::type_repetition_in_bounds,
                    clippy::used_underscore_binding
                )]
                fn handle_list_tools_request<'life0, 'life1, 'async_trait>(
                    &'life0 self,
                    request: ListToolsRequest,
                    runtime: &'life1 dyn McpServer,
                ) -> ::core::pin::Pin<
                    Box<
                        dyn ::core::future::Future<
                            Output = Result<ListToolsResult, RpcError>,
                        > + ::core::marker::Send + 'async_trait,
                    >,
                >
                where
                    'life0: 'async_trait,
                    'life1: 'async_trait,
                    Self: 'async_trait,
                {
                    Box::pin(async move {
                        if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                            Result<ListToolsResult, RpcError>,
                        > {
                            #[allow(unreachable_code)] return __ret;
                        }
                        let __self = self;
                        let request = request;
                        let __ret: Result<ListToolsResult, RpcError> = {
                            Ok(ListToolsResult {
                                meta: None,
                                next_cursor: None,
                                tools: MyTools::tools(),
                            })
                        };
                        #[allow(unreachable_code)] __ret
                    })
                }
                /// Handles incoming CallToolRequest and processes it using the appropriate tool.
                #[allow(
                    elided_named_lifetimes,
                    clippy::async_yields_async,
                    clippy::diverging_sub_expression,
                    clippy::let_unit_value,
                    clippy::needless_arbitrary_self_type,
                    clippy::no_effect_underscore_binding,
                    clippy::shadow_same,
                    clippy::type_complexity,
                    clippy::type_repetition_in_bounds,
                    clippy::used_underscore_binding
                )]
                fn handle_call_tool_request<'life0, 'life1, 'async_trait>(
                    &'life0 self,
                    request: CallToolRequest,
                    _runtime: &'life1 dyn McpServer,
                ) -> ::core::pin::Pin<
                    Box<
                        dyn ::core::future::Future<
                            Output = Result<CallToolResult, CallToolError>,
                        > + ::core::marker::Send + 'async_trait,
                    >,
                >
                where
                    'life0: 'async_trait,
                    'life1: 'async_trait,
                    Self: 'async_trait,
                {
                    Box::pin(async move {
                        if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                            Result<CallToolResult, CallToolError>,
                        > {
                            #[allow(unreachable_code)] return __ret;
                        }
                        let __self = self;
                        let request = request;
                        let __ret: Result<CallToolResult, CallToolError> = {
                            let params = MyTools::try_from(request.params)?;
                            let (tx, mut rx) = mpsc::channel(1);
                            __self.core.update(params.into(), &tx);
                            match rx.recv().await {
                                Some(result) => {
                                    match result {
                                        Ok(data) => {
                                            let text = String::from_utf8(data)
                                                .map_err(CallToolError::new)?;
                                            Ok(
                                                CallToolResult::text_content(
                                                    <[_]>::into_vec(::alloc::boxed::box_new([text.into()])),
                                                ),
                                            )
                                        }
                                        Err(e) => {
                                            Err(
                                                CallToolError::new(
                                                    Error::Other(
                                                        ::anyhow::__private::must_use({
                                                            let error = ::anyhow::__private::format_err(
                                                                format_args!("{0}", e),
                                                            );
                                                            error
                                                        }),
                                                    ),
                                                ),
                                            )
                                        }
                                    }
                                }
                                None => Err(CallToolError::new(Error::ChannelClosed)),
                            }
                        };
                        #[allow(unreachable_code)] __ret
                    })
                }
            }
        }
        pub mod resolve {
            use rust_mcp_sdk::macros::{JsonSchema, mcp_tool};
            use serde::{Deserialize, Serialize};
            use crate::core::capabilities::resolve::ResolveRequest;
            impl Resolve {
                /// Returns the name of the tool as a string.
                pub fn tool_name() -> String {
                    "resolve".to_string()
                }
                /// Constructs and returns a `rust_mcp_schema::Tool` instance.
                ///
                /// The tool includes the name, description, input schema, meta, and title derived from
                /// the struct's attributes.
                pub fn tool() -> rust_mcp_sdk::schema::Tool {
                    let json_schema = &Resolve::json_schema();
                    let required: Vec<_> = match json_schema
                        .get("required")
                        .and_then(|r| r.as_array())
                    {
                        Some(arr) => {
                            arr.iter()
                                .filter_map(|item| item.as_str().map(String::from))
                                .collect()
                        }
                        None => Vec::new(),
                    };
                    let properties: Option<
                        std::collections::HashMap<
                            String,
                            serde_json::Map<String, serde_json::Value>,
                        >,
                    > = json_schema
                        .get("properties")
                        .and_then(|v| v.as_object())
                        .map(|properties| {
                            properties
                                .iter()
                                .filter_map(|(key, value)| {
                                    serde_json::to_value(value)
                                        .ok()
                                        .and_then(|v| {
                                            if let serde_json::Value::Object(obj) = v {
                                                Some(obj)
                                            } else {
                                                None
                                            }
                                        })
                                        .map(|obj| (key.to_string(), obj))
                                })
                                .collect()
                        });
                    rust_mcp_sdk::schema::Tool {
                        name: "resolve".to_string(),
                        description: Some(
                            "Calls the resolve function in a Crux app".to_string(),
                        ),
                        output_schema: None,
                        title: None,
                        meta: None,
                        annotations: Some(rust_mcp_sdk::schema::ToolAnnotations {
                            destructive_hint: Some(false),
                            idempotent_hint: Some(false),
                            open_world_hint: Some(false),
                            read_only_hint: Some(false),
                            title: None,
                        }),
                        input_schema: rust_mcp_sdk::schema::ToolInputSchema::new(
                            required,
                            properties,
                        ),
                    }
                }
            }
            pub struct Resolve {
                pub effect_id: u32,
                pub data: String,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Resolve {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "Resolve",
                        "effect_id",
                        &self.effect_id,
                        "data",
                        &&self.data,
                    )
                }
            }
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for Resolve {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field0,
                            __field1,
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    1u64 => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "effect_id" => _serde::__private::Ok(__Field::__field0),
                                    "data" => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"effect_id" => _serde::__private::Ok(__Field::__field0),
                                    b"data" => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<Resolve>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = Resolve;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct Resolve",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 = match _serde::de::SeqAccess::next_element::<
                                    u32,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct Resolve with 2 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field1 = match _serde::de::SeqAccess::next_element::<
                                    String,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                1usize,
                                                &"struct Resolve with 2 elements",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(Resolve {
                                    effect_id: __field0,
                                    data: __field1,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<u32> = _serde::__private::None;
                                let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                                while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "effect_id",
                                                    ),
                                                );
                                            }
                                            __field0 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<u32>(&mut __map)?,
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("data"),
                                                );
                                            }
                                            __field1 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                            );
                                        }
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(&mut __map)?;
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("effect_id")?
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("data")?
                                    }
                                };
                                _serde::__private::Ok(Resolve {
                                    effect_id: __field0,
                                    data: __field1,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &["effect_id", "data"];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "Resolve",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<Resolve>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for Resolve {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "Resolve",
                            false as usize + 1 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "effect_id",
                            &self.effect_id,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "data",
                            &self.data,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            impl Resolve {
                pub fn json_schema() -> serde_json::Map<String, serde_json::Value> {
                    let mut schema = serde_json::Map::new();
                    let mut properties = serde_json::Map::new();
                    let mut required = Vec::new();
                    properties
                        .insert(
                            "effect_id".to_string(),
                            serde_json::Value::Object({
                                let mut map = serde_json::Map::new();
                                map.insert(
                                    "type".to_string(),
                                    serde_json::Value::String("number".to_string()),
                                );
                                map
                            }),
                        );
                    properties
                        .insert(
                            "data".to_string(),
                            serde_json::Value::Object({
                                let mut map = serde_json::Map::new();
                                map.insert(
                                    "type".to_string(),
                                    serde_json::Value::String("string".to_string()),
                                );
                                map
                            }),
                        );
                    required.push("effect_id".to_string());
                    required.push("data".to_string());
                    schema
                        .insert(
                            "type".to_string(),
                            serde_json::Value::String("object".to_string()),
                        );
                    schema
                        .insert(
                            "properties".to_string(),
                            serde_json::Value::Object(properties),
                        );
                    if !required.is_empty() {
                        schema
                            .insert(
                                "required".to_string(),
                                serde_json::Value::Array(
                                    required
                                        .into_iter()
                                        .map(serde_json::Value::String)
                                        .collect(),
                                ),
                            );
                    }
                    schema
                }
            }
            impl From<Resolve> for ResolveRequest {
                fn from(resolve: Resolve) -> Self {
                    ResolveRequest {
                        effect_id: resolve.effect_id,
                        data: resolve.data.into_bytes(),
                    }
                }
            }
        }
        pub mod schema {
            use rust_mcp_sdk::macros::{JsonSchema, mcp_tool};
            use serde::{Deserialize, Serialize};
            use crate::core::capabilities::schema::SchemaRequest;
            impl Schema {
                /// Returns the name of the tool as a string.
                pub fn tool_name() -> String {
                    "schema".to_string()
                }
                /// Constructs and returns a `rust_mcp_schema::Tool` instance.
                ///
                /// The tool includes the name, description, input schema, meta, and title derived from
                /// the struct's attributes.
                pub fn tool() -> rust_mcp_sdk::schema::Tool {
                    let json_schema = &Schema::json_schema();
                    let required: Vec<_> = match json_schema
                        .get("required")
                        .and_then(|r| r.as_array())
                    {
                        Some(arr) => {
                            arr.iter()
                                .filter_map(|item| item.as_str().map(String::from))
                                .collect()
                        }
                        None => Vec::new(),
                    };
                    let properties: Option<
                        std::collections::HashMap<
                            String,
                            serde_json::Map<String, serde_json::Value>,
                        >,
                    > = json_schema
                        .get("properties")
                        .and_then(|v| v.as_object())
                        .map(|properties| {
                            properties
                                .iter()
                                .filter_map(|(key, value)| {
                                    serde_json::to_value(value)
                                        .ok()
                                        .and_then(|v| {
                                            if let serde_json::Value::Object(obj) = v {
                                                Some(obj)
                                            } else {
                                                None
                                            }
                                        })
                                        .map(|obj| (key.to_string(), obj))
                                })
                                .collect()
                        });
                    rust_mcp_sdk::schema::Tool {
                        name: "schema".to_string(),
                        description: Some("Gets the schema of a Crux app".to_string()),
                        output_schema: None,
                        title: None,
                        meta: None,
                        annotations: Some(rust_mcp_sdk::schema::ToolAnnotations {
                            destructive_hint: Some(false),
                            idempotent_hint: Some(false),
                            open_world_hint: Some(false),
                            read_only_hint: Some(false),
                            title: None,
                        }),
                        input_schema: rust_mcp_sdk::schema::ToolInputSchema::new(
                            required,
                            properties,
                        ),
                    }
                }
            }
            pub struct Schema {}
            #[automatically_derived]
            impl ::core::fmt::Debug for Schema {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(f, "Schema")
                }
            }
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for Schema {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<Schema>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = Schema;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct Schema",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                _: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                _serde::__private::Ok(Schema {})
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(&mut __map)?;
                                        }
                                    }
                                }
                                _serde::__private::Ok(Schema {})
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &[];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "Schema",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<Schema>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for Schema {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "Schema",
                            false as usize,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            impl Schema {
                pub fn json_schema() -> serde_json::Map<String, serde_json::Value> {
                    let mut schema = serde_json::Map::new();
                    let mut properties = serde_json::Map::new();
                    let mut required = Vec::new();
                    schema
                        .insert(
                            "type".to_string(),
                            serde_json::Value::String("object".to_string()),
                        );
                    schema
                        .insert(
                            "properties".to_string(),
                            serde_json::Value::Object(properties),
                        );
                    if !required.is_empty() {
                        schema
                            .insert(
                                "required".to_string(),
                                serde_json::Value::Array(
                                    required
                                        .into_iter()
                                        .map(serde_json::Value::String)
                                        .collect(),
                                ),
                            );
                    }
                    schema
                }
            }
            impl From<Schema> for SchemaRequest {
                fn from(_: Schema) -> Self {
                    Self
                }
            }
        }
        pub mod update {
            use rust_mcp_sdk::macros::{JsonSchema, mcp_tool};
            use serde::{Deserialize, Serialize};
            use crate::core::capabilities::update::UpdateRequest;
            impl Update {
                /// Returns the name of the tool as a string.
                pub fn tool_name() -> String {
                    "update".to_string()
                }
                /// Constructs and returns a `rust_mcp_schema::Tool` instance.
                ///
                /// The tool includes the name, description, input schema, meta, and title derived from
                /// the struct's attributes.
                pub fn tool() -> rust_mcp_sdk::schema::Tool {
                    let json_schema = &Update::json_schema();
                    let required: Vec<_> = match json_schema
                        .get("required")
                        .and_then(|r| r.as_array())
                    {
                        Some(arr) => {
                            arr.iter()
                                .filter_map(|item| item.as_str().map(String::from))
                                .collect()
                        }
                        None => Vec::new(),
                    };
                    let properties: Option<
                        std::collections::HashMap<
                            String,
                            serde_json::Map<String, serde_json::Value>,
                        >,
                    > = json_schema
                        .get("properties")
                        .and_then(|v| v.as_object())
                        .map(|properties| {
                            properties
                                .iter()
                                .filter_map(|(key, value)| {
                                    serde_json::to_value(value)
                                        .ok()
                                        .and_then(|v| {
                                            if let serde_json::Value::Object(obj) = v {
                                                Some(obj)
                                            } else {
                                                None
                                            }
                                        })
                                        .map(|obj| (key.to_string(), obj))
                                })
                                .collect()
                        });
                    rust_mcp_sdk::schema::Tool {
                        name: "update".to_string(),
                        description: Some(
                            "Calls the update function in a Crux app".to_string(),
                        ),
                        output_schema: None,
                        title: None,
                        meta: None,
                        annotations: Some(rust_mcp_sdk::schema::ToolAnnotations {
                            destructive_hint: Some(false),
                            idempotent_hint: Some(false),
                            open_world_hint: Some(false),
                            read_only_hint: Some(false),
                            title: None,
                        }),
                        input_schema: rust_mcp_sdk::schema::ToolInputSchema::new(
                            required,
                            properties,
                        ),
                    }
                }
            }
            pub struct Update {
                pub data: String,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Update {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Update",
                        "data",
                        &&self.data,
                    )
                }
            }
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for Update {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field0,
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "data" => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"data" => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<Update>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = Update;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct Update",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 = match _serde::de::SeqAccess::next_element::<
                                    String,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct Update with 1 element",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(Update { data: __field0 })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                                while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("data"),
                                                );
                                            }
                                            __field0 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                            );
                                        }
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(&mut __map)?;
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("data")?
                                    }
                                };
                                _serde::__private::Ok(Update { data: __field0 })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &["data"];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "Update",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<Update>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for Update {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "Update",
                            false as usize + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "data",
                            &self.data,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            impl Update {
                pub fn json_schema() -> serde_json::Map<String, serde_json::Value> {
                    let mut schema = serde_json::Map::new();
                    let mut properties = serde_json::Map::new();
                    let mut required = Vec::new();
                    properties
                        .insert(
                            "data".to_string(),
                            serde_json::Value::Object({
                                let mut map = serde_json::Map::new();
                                map.insert(
                                    "type".to_string(),
                                    serde_json::Value::String("string".to_string()),
                                );
                                map
                            }),
                        );
                    required.push("data".to_string());
                    schema
                        .insert(
                            "type".to_string(),
                            serde_json::Value::String("object".to_string()),
                        );
                    schema
                        .insert(
                            "properties".to_string(),
                            serde_json::Value::Object(properties),
                        );
                    if !required.is_empty() {
                        schema
                            .insert(
                                "required".to_string(),
                                serde_json::Value::Array(
                                    required
                                        .into_iter()
                                        .map(serde_json::Value::String)
                                        .collect(),
                                ),
                            );
                    }
                    schema
                }
            }
            impl From<Update> for UpdateRequest {
                fn from(update: Update) -> Self {
                    Self {
                        data: update.data.into_bytes(),
                    }
                }
            }
        }
        pub mod view {
            use rust_mcp_sdk::macros::{JsonSchema, mcp_tool};
            use serde::{Deserialize, Serialize};
            use crate::core::capabilities::view::ViewRequest;
            impl View {
                /// Returns the name of the tool as a string.
                pub fn tool_name() -> String {
                    "view".to_string()
                }
                /// Constructs and returns a `rust_mcp_schema::Tool` instance.
                ///
                /// The tool includes the name, description, input schema, meta, and title derived from
                /// the struct's attributes.
                pub fn tool() -> rust_mcp_sdk::schema::Tool {
                    let json_schema = &View::json_schema();
                    let required: Vec<_> = match json_schema
                        .get("required")
                        .and_then(|r| r.as_array())
                    {
                        Some(arr) => {
                            arr.iter()
                                .filter_map(|item| item.as_str().map(String::from))
                                .collect()
                        }
                        None => Vec::new(),
                    };
                    let properties: Option<
                        std::collections::HashMap<
                            String,
                            serde_json::Map<String, serde_json::Value>,
                        >,
                    > = json_schema
                        .get("properties")
                        .and_then(|v| v.as_object())
                        .map(|properties| {
                            properties
                                .iter()
                                .filter_map(|(key, value)| {
                                    serde_json::to_value(value)
                                        .ok()
                                        .and_then(|v| {
                                            if let serde_json::Value::Object(obj) = v {
                                                Some(obj)
                                            } else {
                                                None
                                            }
                                        })
                                        .map(|obj| (key.to_string(), obj))
                                })
                                .collect()
                        });
                    rust_mcp_sdk::schema::Tool {
                        name: "view".to_string(),
                        description: Some(
                            "Gets the ViewModel by calling the view function in a Crux app"
                                .to_string(),
                        ),
                        output_schema: None,
                        title: None,
                        meta: None,
                        annotations: Some(rust_mcp_sdk::schema::ToolAnnotations {
                            destructive_hint: Some(false),
                            idempotent_hint: Some(false),
                            open_world_hint: Some(false),
                            read_only_hint: Some(true),
                            title: None,
                        }),
                        input_schema: rust_mcp_sdk::schema::ToolInputSchema::new(
                            required,
                            properties,
                        ),
                    }
                }
            }
            pub struct View {}
            #[automatically_derived]
            impl ::core::fmt::Debug for View {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(f, "View")
                }
            }
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for View {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<View>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = View;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct View",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                _: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                _serde::__private::Ok(View {})
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(&mut __map)?;
                                        }
                                    }
                                }
                                _serde::__private::Ok(View {})
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &[];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "View",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<View>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for View {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "View",
                            false as usize,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            impl View {
                pub fn json_schema() -> serde_json::Map<String, serde_json::Value> {
                    let mut schema = serde_json::Map::new();
                    let mut properties = serde_json::Map::new();
                    let mut required = Vec::new();
                    schema
                        .insert(
                            "type".to_string(),
                            serde_json::Value::String("object".to_string()),
                        );
                    schema
                        .insert(
                            "properties".to_string(),
                            serde_json::Value::Object(properties),
                        );
                    if !required.is_empty() {
                        schema
                            .insert(
                                "required".to_string(),
                                serde_json::Value::Array(
                                    required
                                        .into_iter()
                                        .map(serde_json::Value::String)
                                        .collect(),
                                ),
                            );
                    }
                    schema
                }
            }
            impl From<View> for ViewRequest {
                fn from(_: View) -> Self {
                    Self
                }
            }
        }
        use resolve::Resolve;
        use schema::Schema;
        use update::Update;
        use view::View;
        use crate::core::app::Event;
        pub enum MyTools {
            Schema(Schema),
            Update(Update),
            Resolve(Resolve),
            View(View),
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for MyTools {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    MyTools::Schema(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Schema",
                            &__self_0,
                        )
                    }
                    MyTools::Update(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Update",
                            &__self_0,
                        )
                    }
                    MyTools::Resolve(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Resolve",
                            &__self_0,
                        )
                    }
                    MyTools::View(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "View",
                            &__self_0,
                        )
                    }
                }
            }
        }
        /// Returns the name of the tool as a String
        impl MyTools {
            pub fn tool_name(&self) -> String {
                match self {
                    MyTools::Schema(_) => Schema::tool_name(),
                    MyTools::Update(_) => Update::tool_name(),
                    MyTools::Resolve(_) => Resolve::tool_name(),
                    MyTools::View(_) => View::tool_name(),
                }
            }
            /// Returns a vector containing instances of all supported tools
            pub fn tools() -> Vec<rust_mcp_sdk::schema::Tool> {
                <[_]>::into_vec(
                    ::alloc::boxed::box_new([
                        Schema::tool(),
                        Update::tool(),
                        Resolve::tool(),
                        View::tool(),
                    ]),
                )
            }
            #[deprecated(since = "0.2.0", note = "Use `tools()` instead.")]
            pub fn get_tools() -> Vec<rust_mcp_sdk::schema::Tool> {
                <[_]>::into_vec(
                    ::alloc::boxed::box_new([
                        Schema::tool(),
                        Update::tool(),
                        Resolve::tool(),
                        View::tool(),
                    ]),
                )
            }
        }
        impl TryFrom<rust_mcp_sdk::schema::CallToolRequestParams> for MyTools {
            type Error = rust_mcp_sdk::schema::schema_utils::CallToolError;
            /// Attempts to convert a tool request into the appropriate tool variant
            fn try_from(
                value: rust_mcp_sdk::schema::CallToolRequestParams,
            ) -> Result<Self, Self::Error> {
                let v = serde_json::to_value(value.arguments.unwrap())
                    .map_err(rust_mcp_sdk::schema::schema_utils::CallToolError::new)?;
                match value.name {
                    name if name == Schema::tool_name().as_str() => {
                        Ok(
                            Self::Schema(
                                serde_json::from_value(v)
                                    .map_err(
                                        rust_mcp_sdk::schema::schema_utils::CallToolError::new,
                                    )?,
                            ),
                        )
                    }
                    name if name == Update::tool_name().as_str() => {
                        Ok(
                            Self::Update(
                                serde_json::from_value(v)
                                    .map_err(
                                        rust_mcp_sdk::schema::schema_utils::CallToolError::new,
                                    )?,
                            ),
                        )
                    }
                    name if name == Resolve::tool_name().as_str() => {
                        Ok(
                            Self::Resolve(
                                serde_json::from_value(v)
                                    .map_err(
                                        rust_mcp_sdk::schema::schema_utils::CallToolError::new,
                                    )?,
                            ),
                        )
                    }
                    name if name == View::tool_name().as_str() => {
                        Ok(
                            Self::View(
                                serde_json::from_value(v)
                                    .map_err(
                                        rust_mcp_sdk::schema::schema_utils::CallToolError::new,
                                    )?,
                            ),
                        )
                    }
                    _ => {
                        Err(
                            rust_mcp_sdk::schema::schema_utils::CallToolError::unknown_tool(
                                value.name.to_string(),
                            ),
                        )
                    }
                }
            }
        }
        impl From<MyTools> for Event {
            fn from(tool: MyTools) -> Self {
                match tool {
                    MyTools::Schema(params) => Event::Schema(params.into()),
                    MyTools::Update(params) => Event::Update(params.into()),
                    MyTools::Resolve(params) => Event::Resolve(params.into()),
                    MyTools::View(params) => Event::View(params.into()),
                }
            }
        }
    }
}
pub use error::{Error, Result};
fn main() -> SdkResult<()> {
    let body = async {
        let server_details = InitializeResult {
            server_info: Implementation {
                name: "Crux app MCP Server".to_string(),
                version: "0.1.0".to_string(),
                title: None,
            },
            capabilities: ServerCapabilities {
                tools: Some(ServerCapabilitiesTools {
                    list_changed: None,
                }),
                ..Default::default()
            },
            meta: None,
            instructions: Some("server instructions...".to_string()),
            protocol_version: LATEST_PROTOCOL_VERSION.to_string(),
        };
        let mut host = Host::default();
        host.load().expect("failed to load guest");
        let core = Core::new(host);
        let server = server_runtime::create_server(
            server_details,
            StdioTransport::new(TransportOptions::default())?,
            MyServerHandler::new(core),
        );
        server.start().await
    };
    #[allow(
        clippy::expect_used,
        clippy::diverging_sub_expression,
        clippy::needless_return
    )]
    {
        return tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
