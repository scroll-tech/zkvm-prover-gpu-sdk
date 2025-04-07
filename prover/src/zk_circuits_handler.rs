use anyhow::Result;
use async_trait::async_trait;
use prover::ProofType;

#[async_trait]
pub trait CircuitsHandler: Sync + Send {
    async fn get_vk(&self, task_type: ProofType) -> Option<Vec<u8>>;

    async fn get_proof_data(&self, proof_type: ProofType, input: String, fork_name: String) -> Result<String>;
}