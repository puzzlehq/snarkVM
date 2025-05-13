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

mod parse;
mod serialize;

use crate::state_path::StatePath;
use snarkvm_console_network::prelude::*;

#[derive(Clone, PartialEq, Eq)]
pub struct StateProofs<N: Network> {
    /// The block height.
    block_height: u32,
    /// The global state root.
    global_state_root: N::StateRoot,
    /// The state paths.
    state_paths: Vec<StatePath<N>>,
}

impl<N: Network> StateProofs<N> {
    /// Initializes a new instance of `StateProofs`.
    pub const fn new(block_height: u32, global_state_root: N::StateRoot, state_paths: Vec<StatePath<N>>) -> Self {
        Self { block_height, global_state_root, state_paths }
    }

    /// Returns the block height.
    pub const fn block_height(&self) -> u32 {
        self.block_height
    }

    /// Returns the global state root.
    pub const fn global_state_root(&self) -> N::StateRoot {
        self.global_state_root
    }

    /// Returns the state paths.
    pub const fn state_paths(&self) -> &Vec<StatePath<N>> {
        &self.state_paths
    }
}
