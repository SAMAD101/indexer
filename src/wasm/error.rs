use thiserror::Error;
use wasmer::error::CompileError;

#[derive(Debug, Error)]
pub enum WasmError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Wasmer error: {0}")]
    Wasmer(#[from] wasmer::error::Error),

    #[error("Wasmer compile error: {0}")]
    WasmerCompile(#[from] CompileError),

    #[error("Module not found: {0}")]
    ModuleNotFound(String),

    #[error("Function not found: {0}")]
    FunctionNotFound(String),

    #[error("Execution error: {0}")]
    Execution(String),

    #[error("Memory access error: {0}")]
    MemoryAccess(String),

    #[error("Type conversion error: {0}")]
    TypeConversion(String),

    #[error("Configuration error: {0}")]
    Config(String),
}

impl From<Box<dyn std::error::Error>> for WasmError {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        WasmError::Execution(error.to_string())
    }
}
