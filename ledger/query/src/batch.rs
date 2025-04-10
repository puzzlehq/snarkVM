// Copyright 2024-2025 Aleo Network Foundation
// This file is part of the snarkVM library.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at:

// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::{collections::HashMap, sync::Arc};

use console::{
    network::prelude::*,
    prelude::{Error, Result, anyhow, bail},
    program::StatePath,
    types::Field,
};

use crate::QueryTrait;

#[derive(Clone)]
pub struct InMemoryStateQuery<N: Network> {
    state_root: N::StateRoot,
    block_height: u32,
    state_paths: Arc<HashMap<Field<N>, StatePath<N>>>,
}

impl<N: Network> InMemoryStateQuery<N> {
    pub fn new(block_height: u32, state_root: N::StateRoot, paths: Vec<StatePath<N>>) -> Self {
        let state_paths = paths
            .iter()
            .map(|p| {
                let record_commitment = p.transition_leaf().id();
                (record_commitment, p.clone())
            })
            .collect::<HashMap<_, _>>();

        println!("🔍 State paths (transition commitments): {:?}", state_paths);

        Self { block_height, state_root, state_paths: Arc::new(state_paths) }
    }
}

#[cfg_attr(feature = "async", async_trait::async_trait(?Send))]
impl<N: Network> QueryTrait<N> for InMemoryStateQuery<N> {
    fn current_state_root(&self) -> Result<N::StateRoot> {
        Ok(self.state_root)
    }

    fn current_block_height(&self) -> Result<u32> {
        Ok(self.block_height)
    }

    fn get_state_path_for_commitment(&self, commitment: &Field<N>) -> Result<StatePath<N>> {
        println!("🔍 Looking up commitment: {}", commitment);
        println!("📦 Available keys (normalized tcm):");
        for key in self.state_paths.keys() {
            println!("  🔑 {}", key);
        }

        let normalized = Field::<N>::from_str(&commitment.to_string()).unwrap();
        println!("🔎 Normalized lookup key: {}", normalized);

        match self.state_paths.get(&normalized) {
            Some(path) => {
                println!("✅ Match found for commitment {}", commitment);
                Ok(path.clone())
            }
            None => {
                println!("❌ No match for commitment {} (normalized: {})", commitment, normalized);
                bail!("Commitment not found in fetched state paths")
            }
        }
    }

    #[cfg(feature = "async")]
    async fn current_state_root_async(&self) -> Result<N::StateRoot> {
        Ok(self.state_root)
    }

    #[cfg(feature = "async")]
    async fn current_block_height_async(&self) -> Result<u32> {
        Ok(self.block_height)
    }

    #[cfg(feature = "async")]
    async fn get_state_path_for_commitment_async(&self, commitment: &Field<N>) -> Result<StatePath<N>> {
        self.state_paths
            .get(commitment)
            .cloned()
            .ok_or_else(|| anyhow!("Missing state path for commitment: {commitment}"))
    }
}
