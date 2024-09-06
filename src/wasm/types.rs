use super::error::WasmError;
use crate::config::Config;
use std::collections::HashMap;

pub type WasmResult = Result<Vec<u8>, WasmError>;

#[derive(Debug, Clone)]
pub struct WasmConfig {
    module_paths: HashMap<String, String>,
    memory_limit: Option<usize>,
    execution_timeout: Option<std::time::Duration>,
}

impl WasmConfig {
    pub fn from_config(config: &Config) -> Result<Self, WasmError> {
        let module_paths = config.wasm_modules.clone().ok_or_else(|| {
            WasmError::Config("WASM modules configuration is missing".to_string())
        })?;

        Ok(Self {
            module_paths,
            memory_limit: config.wasm_memory_limit,
            execution_timeout: config
                .wasm_execution_timeout
                .map(std::time::Duration::from_secs),
        })
    }

    pub fn get_module_path(&self, module_name: &str) -> Result<&str, WasmError> {
        self.module_paths
            .get(module_name)
            .map(String::as_str)
            .ok_or_else(|| WasmError::ModuleNotFound(module_name.to_string()))
    }

    pub fn memory_limit(&self) -> Option<usize> {
        self.memory_limit
    }

    pub fn execution_timeout(&self) -> Option<std::time::Duration> {
        self.execution_timeout
    }
}

#[derive(Debug, Clone)]
pub struct WasmModuleInfo {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
}

#[derive(Debug)]
pub struct WasmExecutionContext {
    pub module_name: String,
    pub function_name: String,
    pub args: Vec<String>,
}
