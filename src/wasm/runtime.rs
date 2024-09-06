use wasmer::{Instance, Module, Store};
use wasmer_wasi::WasiState;

pub struct WasmRuntime {
    store: Store,
}

impl WasmRuntime {
    pub fn new() -> Self {
        let store = Store::default();
        Self { store }
    }

    pub fn run_module(
        &mut self,
        wasm_bytes: &[u8],
        func_name: &str,
        args: &[&str],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let module = Module::new(&self.store, wasm_bytes)?;
        let wasi_env = WasiState::new(func_name).args(args).finalize()?;

        let import_object = wasi_env.import_object(&mut self.store, &module)?;
        let instance = Instance::new(&mut self.store, &module, &import_object)?;

        let run = instance.exports.get_function(func_name)?;
        run.call(&mut self.store, &[])?;

        Ok(())
    }
}
