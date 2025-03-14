pub use models::*;
use nject::{injectable, module};
use repository::Repository;
pub use service::UserService;
pub use service::CpuIntensiveService;
mod models;
mod repository;
mod service;

#[module]
#[injectable]
pub struct Module {
    #[export(dyn Repository)]
    repository: repository::memory::MemoryRepository,
}
