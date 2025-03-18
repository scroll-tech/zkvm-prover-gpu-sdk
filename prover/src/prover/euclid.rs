use std::path::Path;

use anyhow::Result;
use euclid_prover::{
    task::{batch::BatchProvingTask, bundle::BundleProvingTask, chunk::ChunkProvingTask},
    BatchProver, BundleProver, ChunkProver, ProverConfig,
};

use super::ProofType;

pub struct EuclidProver {
    chunk_prover: ChunkProver,
    batch_prover: BatchProver,
    bundle_prover: BundleProver,
}

impl EuclidProver {
    pub fn new(workspace_path: &str) -> Self {
        let workspace_path = Path::new(workspace_path);

        let cache_dir = workspace_path.join("cache");
        let chunk_exe = workspace_path.join("chunk/app.vmexe");
        let chunk_app_config = workspace_path.join("chunk/openvm.toml");
        let chunk_prover = ChunkProver::setup(
            chunk_exe,
            chunk_app_config,
            Some(cache_dir.clone()),
            ProverConfig {
                segment_len: Some((1 << 22) - 100),
            },
        )
        .expect("Failed to setup chunk prover");

        let batch_exe = workspace_path.join("batch/app.vmexe");
        let batch_app_config = workspace_path.join("batch/openvm.toml");
        let batch_prover = BatchProver::setup(
            batch_exe,
            batch_app_config,
            Some(cache_dir.clone()),
            ProverConfig {
                segment_len: Some((1 << 22) - 100),
            },
        )
        .expect("Failed to setup batch prover");

        let bundle_exe = workspace_path.join("bundle/app.vmexe");
        let bundle_app_config = workspace_path.join("bundle/openvm.toml");
        let bundle_prover = BundleProver::setup(
            bundle_exe,
            bundle_app_config,
            Some(cache_dir),
            ProverConfig {
                segment_len: Some((1 << 22) - 100),
            },
        )
        .expect("Failed to setup bundle prover");

        Self {
            chunk_prover,
            batch_prover,
            bundle_prover,
        }
    }

    #[allow(dead_code)]
    pub fn get_vk(&self, task_type: ProofType) -> Option<Vec<u8>> {
        Some(match task_type {
            ProofType::Chunk => self.chunk_prover.get_app_vk(),
            ProofType::Batch => self.batch_prover.get_app_vk(),
            ProofType::Bundle => self.bundle_prover.get_app_vk(),
        })
    }

    pub fn get_proof_data(&self, proof_type: ProofType, input: String) -> Result<String> {
        match proof_type {
            ProofType::Chunk => {
                let witnesses: Vec<sbv_primitives::types::BlockWitness> =
                    serde_json::from_str(&input)?;

                let proof = self.chunk_prover.gen_proof(&ChunkProvingTask {
                    block_witnesses: witnesses,
                    prev_msg_queue_hash: Default::default(),
                })?;

                Ok(serde_json::to_string(&proof)?)
            }
            ProofType::Batch => {
                let task: BatchProvingTask = serde_json::from_str(&input)?;
                let proof = self.batch_prover.gen_proof(&task)?;

                Ok(serde_json::to_string(&proof)?)
            }
            ProofType::Bundle => {
                let batch_proofs: BundleProvingTask = serde_json::from_str(&input)?;
                let proof = self.bundle_prover.gen_proof_evm(&batch_proofs)?;

                Ok(serde_json::to_string(&proof)?)
            }
        }
    }
}
