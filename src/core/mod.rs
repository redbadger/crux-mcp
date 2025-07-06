use std::{
    env,
    sync::{Arc, Mutex},
};

use anyhow::{Result, bail};
use wasmtime::{
    Engine, Store,
    component::{Component, Linker, TypedFunc},
};
use wasmtime_wasi::{
    ResourceTable,
    p2::{IoView, WasiCtx, WasiCtxBuilder, WasiView, add_to_linker_sync},
};

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
pub struct Core {
    store: Arc<Mutex<Option<Store<ComponentRunStates>>>>,
    view: Option<TypedFunc<(), (Vec<u8>,)>>,
}

impl Core {
    pub fn load(&mut self) -> Result<()> {
        let engine = Engine::default();

        let mut linker = Linker::new(&engine);
        add_to_linker_sync(&mut linker)?;

        let state = ComponentRunStates {
            wasi_ctx: WasiCtxBuilder::new().inherit_stdio().inherit_args().build(),
            resource_table: ResourceTable::new(),
        };
        let mut store = Store::new(&engine, state);

        let file = "../crux/examples/counter-next/target/wasm32-wasip2/debug/shared.wasm";
        let file = env::current_dir()?.join(file);
        let component = Component::from_file(&engine, file)?;
        let instance = linker.instantiate(&mut store, &component)?;

        let Some(func) = instance.get_func(&mut store, "view") else {
            bail!("view function not found");
        };

        self.view = Some(func.typed::<(), (Vec<u8>,)>(&store)?);
        self.store = Arc::new(Mutex::new(Some(store)));

        Ok(())
    }

    pub fn view(&self) -> Result<String> {
        let Some(view) = &self.view else {
            bail!("no view function");
        };

        let mut store = self.store.lock().expect("failed to lock store");
        let Some(mut store) = store.as_mut() else {
            bail!("no store");
        };

        let (data,) = view.call(&mut store, ())?;
        view.post_return(&mut store)?;

        let data = String::from_utf8(data)?;

        Ok(data)
    }
}
