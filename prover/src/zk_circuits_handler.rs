use anyhow::Result;
use async_trait::async_trait;
use prover::ProofType;

pub trait CircuitsHandler {
    fn get_vk(&self, task_type: ProofType) -> Option<Vec<u8>>;

    fn get_proof_data(&self, proof_type: ProofType, input: String, fork_name: String) -> Result<String>;
}