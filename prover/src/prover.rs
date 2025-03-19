use std::{cell::OnceCell, rc::Rc};

use anyhow::{anyhow, Result};
use euclid::EuclidProver;

mod euclid;

#[derive(Clone, Debug)]
pub enum ProofType {
    Chunk,
    Batch,
    Bundle,
}

static mut PROVER: OnceCell<Rc<Box<EuclidProver>>> = OnceCell::new();

pub fn init(workspace_path: String) {
    let prover = Rc::new(Box::new(EuclidProver::new(workspace_path.as_str())));

    unsafe {
        PROVER.set(prover).unwrap_unchecked();
    }
}

pub fn get_prover() -> Result<Rc<Box<EuclidProver>>> {
    unsafe {
        PROVER
            .get()
            .cloned()
            .ok_or_else(|| anyhow!("Prover is not initialized"))
    }
}
