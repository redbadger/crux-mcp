use std::{
    env,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use anyhow::{Result, anyhow, bail};
use wasmtime::{
    Engine, Store,
    component::{Component, Linker, ResourceAny, ResourceTable, bindgen},
};

use self::exports::crux::shared_lib::core::Guest;
use wasmtime_wasi::p2::{IoView, WasiCtx, WasiCtxBuilder, WasiView, add_to_linker_sync};

bindgen!();

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

#[derive(Default)]
pub struct Host {
    store: Arc<Mutex<Option<Store<ComponentRunStates>>>>,
    shared: Option<Shared>,
    resource: Option<ResourceAny>,
}

impl Host {
    pub fn load(&mut self) -> Result<()> {
        let engine = Engine::default();

        let mut linker = Linker::new(&engine);
        add_to_linker_sync(&mut linker)?;

        let state = ComponentRunStates {
            wasi_ctx: WasiCtxBuilder::new().inherit_stdio().inherit_args().build(),
            resource_table: ResourceTable::new(),
        };
        let mut store = Store::new(&engine, state);

        let file = PathBuf::from(env::var("CRUX_COMPONENT")?);
        let component = Component::from_file(&engine, file)?;

        // Use the generated bindings to instantiate the component
        let shared = Shared::instantiate(&mut store, &component, &linker)?;

        // Create a new instance of the exported resource
        let resource = shared
            .crux_shared_lib_core()
            .instance()
            .call_constructor(&mut store)?;

        self.shared = Some(shared);
        self.resource = Some(resource);
        self.store = Arc::new(Mutex::new(Some(store)));

        Ok(())
    }

    fn with_guest<T, F>(&self, f: F) -> Result<T>
    where
        F: FnOnce(&Guest, &mut Store<ComponentRunStates>, ResourceAny) -> Result<T>,
    {
        let Some(shared) = &self.shared else {
            bail!("component not loaded");
        };
        let Some(resource) = &self.resource else {
            bail!("no instance created");
        };

        let mut store = self.store.lock().expect("failed to lock store");
        let Some(store) = store.as_mut() else {
            bail!("no store");
        };

        f(shared.crux_shared_lib_core(), store, *resource)
    }

    pub fn schema(&self) -> Result<String> {
        self.with_guest(|guest, store, resource| guest.instance().call_schema(store, resource))
    }

    pub fn update(&self, data: &[u8]) -> Result<Vec<u8>> {
        self.with_guest(|guest, store, resource| {
            guest
                .instance()
                .call_update(store, resource, data)?
                .map_err(|e| anyhow!("bridge error: {e}"))
        })
    }

    pub fn resolve(&self, effect_id: u32, data: &[u8]) -> Result<Vec<u8>> {
        self.with_guest(|guest, store, resource| {
            guest
                .instance()
                .call_resolve(store, resource, effect_id, data)?
                .map_err(|e| anyhow!("bridge error: {e}"))
        })
    }

    pub fn view(&self) -> Result<Vec<u8>> {
        self.with_guest(|guest, store, resource| {
            guest
                .instance()
                .call_view(store, resource)?
                .map_err(|e| anyhow!("bridge error: {e}"))
        })
    }
}
