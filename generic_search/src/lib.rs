pub mod search {
    tonic::include_proto!("search");
}

pub mod defs;
pub mod common;
pub mod producer;
pub mod local_consumer;
pub mod remote_consumer;
pub mod local_orchestrator;
pub mod remote_orchestrator;
