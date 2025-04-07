use std::sync::OnceLock;
use std::rc::Rc;
use std::sync::Mutex;
use std::cell::RefCell;

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

pub static ACTIVE_HANDLER: RefCell<(String, Rc<dyn CircuitsHandler>)> = RefCell::new(None);
pub static WORKSPACE_PATH: OnceLock<(&str)> = OnceLock::new();

pub fn init(workspace_path: &'static str) {
    WORKSPACE_PATH.set(workspace_path);
}

pub fn set_active_handler(hard_fork_name: &str) {
    let mut handler = ACTIVE_HANDLER.borrow_mut().unwrap();
    if let (name, _) = &*handler {
        if name == hard_fork_name {
            return;
        }
    }
    *handler = Some((hard_fork_name.to_string(), new_handler(hard_fork_name)));
}

fn new_handler(hard_fork_name: &str) -> Rc<dyn CircuitsHandler> {
    // Get the workspace path or return early if not initialized
    let workspace_path = WORKSPACE_PATH.get().ok_or_else(|| {
        panic!("WORKSPACE_PATH not initialized!");
    });

    match workspace_path {
        Ok(wp) => {
            match hard_fork_name {
                "euclid" => Rc::new(euclid::EuclidProver::new(
                    wp,
                )) as Rc<dyn CircuitsHandler>,
                "euclidV2" => Rc::new(euclidV2::EuclidV2Prover::new(
                    wp,
                )) as Rc<dyn CircuitsHandler>,
                _ => unreachable!("Wrong hard fork name"),
            }
        },
        _ => unreachable!("WORKSPACE_PATH not set"),
    }
    
}
