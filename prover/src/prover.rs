use std::sync::OnceLock;
use std::rc::Rc;
use once_cell::unsync::OnceCell;

pub mod euclid;
#[allow(non_snake_case)]
pub mod euclidV2;

use crate::zk_circuits_handler::CircuitsHandler;

#[derive(Clone, Debug)]
pub enum ProofType {
    Chunk,
    Batch,
    Bundle,
}

pub static mut ACTIVE_HANDLER: OnceCell<Option<(String, Rc<dyn CircuitsHandler>)>> = OnceCell::new();
pub static WORKSPACE_PATH: OnceLock<&str> = OnceLock::new();

pub fn init(workspace_path: &'static str) {
    WORKSPACE_PATH.set(workspace_path);
}

pub fn set_active_handler(hard_fork_name: &str) {
    unsafe { let prover = ACTIVE_HANDLER.get(); }
    if let Some(h) = &*prover {
        if h.0 == hard_fork_name {
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
