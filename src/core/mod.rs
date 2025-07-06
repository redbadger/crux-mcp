use std::{
    env,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use anyhow::{Result, bail};
use wasmtime::{
    Engine, Store,
    component::{Component, Linker, ResourceTable, TypedFunc},
};
use wasmtime_wasi::p2::{IoView, WasiCtx, WasiCtxBuilder, WasiView, add_to_linker_sync};

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

#[derive(Default)]
pub struct Core {
    store: Arc<Mutex<Option<Store<ComponentRunStates>>>>,
    schema: Option<TypedFunc<(), (String,)>>,
    update: Option<TypedFunc<(Bytes,), (Bytes,)>>,
    resolve: Option<TypedFunc<(u32, Bytes), (Bytes,)>>,
    view: Option<TypedFunc<(), (Bytes,)>>,
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

        let file = PathBuf::from(env::var("CRUX_COMPONENT")?);
        let component = Component::from_file(&engine, file)?;
        let instance = linker.instantiate(&mut store, &component)?;

        if let Some(func) = instance.get_func(&mut store, "schema") {
            self.schema = Some(func.typed::<(), (String,)>(&store)?);
        } else {
            bail!("schema function not found");
        }

        if let Some(func) = instance.get_func(&mut store, "update") {
            self.update = Some(func.typed::<(Bytes,), (Bytes,)>(&store)?);
        } else {
            bail!("update function not found");
        }

        if let Some(func) = instance.get_func(&mut store, "resolve") {
            self.resolve = Some(func.typed::<(u32, Bytes), (Bytes,)>(&store)?);
        } else {
            bail!("resolve function not found");
        }

        if let Some(func) = instance.get_func(&mut store, "view") {
            self.view = Some(func.typed::<(), (Bytes,)>(&store)?);
        } else {
            bail!("view function not found");
        }

        self.store = Arc::new(Mutex::new(Some(store)));

        Ok(())
    }

    pub fn schema(&self) -> Result<String> {
        let Some(schema) = self.schema else {
            bail!("no schema function");
        };

        let mut store = self.store.lock().expect("failed to lock store");
        let Some(mut store) = store.as_mut() else {
            bail!("no store");
        };

        let (data,) = schema.call(&mut store, ())?;
        schema.post_return(&mut store)?;

        Ok(data)
    }

    pub fn update(&self, data: Vec<u8>) -> Result<Vec<u8>> {
        let Some(update) = self.update else {
            bail!("no update function");
        };

        let mut store = self.store.lock().expect("failed to lock store");
        let Some(mut store) = store.as_mut() else {
            bail!("no store");
        };

        let (data,) = update.call(&mut store, (data,))?;
        update.post_return(&mut store)?;

        Ok(data)
    }

    pub fn resolve(&self, effect_id: u32, data: Vec<u8>) -> Result<Vec<u8>> {
        let Some(resolve) = self.resolve else {
            bail!("no update function");
        };

        let mut store = self.store.lock().expect("failed to lock store");
        let Some(mut store) = store.as_mut() else {
            bail!("no store");
        };

        let (data,) = resolve.call(&mut store, (effect_id, data))?;
        resolve.post_return(&mut store)?;

        Ok(data)
    }

    pub fn view(&self) -> Result<Vec<u8>> {
        let Some(view) = &self.view else {
            bail!("no view function");
        };

        let mut store = self.store.lock().expect("failed to lock store");
        let Some(mut store) = store.as_mut() else {
            bail!("no store");
        };

        let (data,) = view.call(&mut store, ())?;
        view.post_return(&mut store)?;

        Ok(data)
    }
}
