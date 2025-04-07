use std::sync::OnceLock;
use std::sync::{Arc, RwLock};

pub mod euclid;
#[allow(non_snake_case)]
pub mod euclidV2;

use anyhow::{anyhow, Result};
use euclidV2::EuclidV2Prover;

#[derive(Clone, Debug)]
pub enum ProofType {
    Chunk,
    Batch,
    Bundle,
}

static ACTIVE_HANDLER: RwLock<Option<(String, Arc<dyn CircuitsHandler>)>> = RwLock::new(None);
static WORKSPACE_PATH: OnceCell<(String)> = OnceCell::new();

pub fn init(workspace_path: String) {
    WORKSPACE_PATH.set(workspace_path)
}

pub fn set_active_handler(hard_fork_name: &str) {
    let mut handler = ACTIVE_HANDLER.write().unwrap(); // 获取写锁
    if let Some((name, _)) = &*handler {
        if name == hard_fork_name {
            return;
        }
    }
    *handler = Some((hard_fork_name.to_string(), new_handler(hard_fork_name)));
}

fn new_handler(hard_fork_name: &str) -> Arc<dyn CircuitsHandler> {
    if WORKSPACE_PATH.get().is_none() {
        println!("WORKSPACE_PATH not initialized yet!");
    }
    match hard_fork_name {
        "euclid" => Arc::new(Arc::new(Mutex::new(euclid::EuclidProver::new(
            WORKSPACE_PATH,
        )))) as Arc<dyn CircuitsHandler>,
        "euclidV2" => Arc::new(Arc::new(Mutex::new(euclidV2::EuclidV2Prover::new(
            WORKSPACE_PATH,
        )))) as Arc<dyn CircuitsHandler>,
        _ => unreachable!(),
    }
}
