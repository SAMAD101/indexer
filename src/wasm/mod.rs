mod error;
mod runtime;
mod types;

use wasmer::{Instance, Module, Store};
use wasmer_wasi::WasiState;

pub use error::WasmError;
pub use runtime::WasmRuntime;
pub use types::{WasmConfig, WasmResult};

use crate::config::Config;

pub struct WasmManager {
    runtime: WasmRuntime,
    config: WasmConfig,
}

impl WasmManager {
    pub fn new(config: &Config) -> Result<Self, WasmError> {
        let wasm_config = WasmConfig::from_config(config);
        let runtime = WasmRuntime::new()?;

        Ok(Self {
            runtime,
            config: wasm_config,
        })
    }

    pub fn load_module(&mut self, module_name: &str) -> Result<(), WasmError> {
        let module_path = self.config.get_module_path(module_name)?;
        let wasm_bytes = std::fs::read(module_path)?;
        self.runtime.load_module(module_name, &wasm_bytes)?;
        Ok(())
    }

    pub async fn execute_function(
        &mut self,
        module_name: &str,
        function_name: &str,
        args: &[&str],
    ) -> WasmResult {
        self.runtime
            .execute_function(module_name, function_name, args)
            .await
    }

    pub fn get_exported_memory(&self, module_name: &str) -> Result<wasmer::Memory, WasmError> {
        self.runtime.get_exported_memory(module_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use std::time::Duration;

    fn create_test_config() -> Config {
        Config {
            wasm_modules: Some(HashMap::from([(
                "test_module".to_string(),
                "./tests/test_module.wasm".to_string(),
            )])),
            wasm_memory_limit: Some(1024 * 1024),
            wasm_execution_timeout: Some(5),
            solana_rpc_url: todo!(),
            clickhouse_url: todo!(),
            scylla_nodes: todo!(),
            redis_url: todo!(),
            ipfs_api_url: todo!(),
            wasm_module_path: todo!(),
            geyser_plugin_config: todo!(),
            rpc_poll_interval: todo!(),
            websocket_url: todo!(),
        }
    }

    #[test]
    fn test_wasm_manager_creation() {
        let config = create_test_config();
        let result = WasmManager::new(&config);
        assert!(
            result.is_ok(),
            "Failed to create WasmManager: {:?}",
            result.err()
        );
    }

    #[test]
    fn test_load_module() {
        let config = create_test_config();
        let mut manager = WasmManager::new(&config).expect("Failed to create WasmManager");

        let result = manager.load_module("test_module");
        assert!(result.is_ok(), "Failed to load module: {:?}", result.err());

        let result = manager.load_module("non_existent_module");
        assert!(matches!(result, Err(WasmError::ModuleNotFound(_))));
    }

    #[tokio::test]
    async fn test_execute_function() {
        let config = create_test_config();
        let mut manager = WasmManager::new(&config).expect("Failed to create WasmManager");
        manager
            .load_module("test_module")
            .expect("Failed to load test module");

        let result = manager
            .execute_function("test_module", "test_function", &[])
            .await;
        assert!(
            result.is_ok(),
            "Failed to execute function: {:?}",
            result.err()
        );

        let output = result.unwrap();
        assert_eq!(output.len(), 4, "Expected 4 bytes output for u32");

        let value = u32::from_le_bytes(output.try_into().unwrap());
        assert_eq!(value, 42, "Expected output value to be 42");

        let result = manager
            .execute_function("test_module", "non_existent_function", &[])
            .await;
        assert!(matches!(result, Err(WasmError::FunctionNotFound(_))));
    }

    #[test]
    fn test_get_exported_memory() {
        let config = create_test_config();
        let mut manager = WasmManager::new(&config).expect("Failed to create WasmManager");
        manager
            .load_module("test_module")
            .expect("Failed to load test module");

        let result = manager.get_exported_memory("test_module");
        assert!(
            result.is_ok(),
            "Failed to get exported memory: {:?}",
            result.err()
        );

        let result = manager.get_exported_memory("non_existent_module");
        assert!(matches!(result, Err(WasmError::ModuleNotFound(_))));
    }
}
