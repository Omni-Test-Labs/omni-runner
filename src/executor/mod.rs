pub mod python;
pub mod binary;
pub mod shell;
pub mod api;

pub use python::PythonExecutor;
pub use binary::BinaryExecutor;
pub use shell::ShellExecutor;
pub use api::ApiExecutor;

/// Trait for executing different types of pipeline steps
#[async_trait::async_trait]
pub trait Executor {
    async fn execute(&self, step: &crate::models::PipelineStep)
        -> Result<crate::models::StepResult, anyhow::Error>;
}

/// Wrapper enum for executor types to enable dynamic dispatch
pub enum ExecutorType {
    Python(PythonExecutor),
    Binary(BinaryExecutor),
    Shell(ShellExecutor),
    Api(ApiExecutor),
}

#[async_trait::async_trait]
impl Executor for ExecutorType {
    async fn execute(&self, step: &crate::models::PipelineStep) -> Result<crate::models::StepResult, anyhow::Error> {
        match self {
            ExecutorType::Python(e) => e.execute(step).await,
            ExecutorType::Binary(e) => e.execute(step).await,
            ExecutorType::Shell(e) => e.execute(step).await,
            ExecutorType::Api(e) => e.execute(step).await,
        }
    }
}
