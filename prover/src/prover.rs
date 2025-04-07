use std::sync::OnceLock;
use std::sync::{Arc, RwLock};

pub mod euclid;
#[allow(non_snake_case)]
pub mod euclidV2;

use anyhow::{anyhow, Result};
use crate::zk_circuits_handler::CircuitsHandler;

#[derive(Clone, Debug)]
pub enum ProofType {
    Chunk,
    Batch,
    Bundle,
}

static ACTIVE_HANDLER: RwLock<Option<(String, Arc<dyn CircuitsHandler>)>> = RwLock::new(None);
static WORKSPACE_PATH: OnceLock<(&str)> = OnceLock::new();

pub fn init(workspace_path: &'static str) {
    WORKSPACE_PATH.set(workspace_path);
}

pub fn set_active_handler(hard_fork_name: &str) {
    let mut handler = ACTIVE_HANDLER.write().unwrap();
    if let Some((name, _)) = &*handler {
        if name == hard_fork_name {
            return;
        }
    }
    *handler = Some((hard_fork_name.to_string(), new_handler(hard_fork_name)));
}

fn new_handler(hard_fork_name: &str) -> Arc<dyn CircuitsHandler> {
    // Get the workspace path or return early if not initialized
    let workspace_path = WORKSPACE_PATH.get().ok_or_else(|| {
        panic!("WORKSPACE_PATH not initialized!");
    });

    match workspace_path {
        Some(wp) => {
            match hard_fork_name {
                "euclid" => Arc::new(Mutex::new(euclid::EuclidProver::new(
                    wp,
                ))) as Arc<dyn CircuitsHandler>,
                "euclidV2" => Arc::new(Mutex::new(euclidV2::EuclidV2Prover::new(
                    wp,
                ))) as Arc<dyn CircuitsHandler>,
                _ => unreachable!("Wrong hard fork name"),
            }
        },
        _ => unreachable!("WORKSPACE_PATH not set"),
    }
    
}
