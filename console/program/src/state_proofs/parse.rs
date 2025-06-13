// Copyright (c) 2019-2025 Provable Inc.
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

use super::*;
use core::str::FromStr;

impl<N: Network> FromStr for StateProofs<N> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Parse JSON format
        serde_json::from_str(s).map_err(|e| format!("Failed to parse StateProofs: {}", e))
    }
}

impl<N: Network> Display for StateProofs<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "StateProofs {{ block_height: {}, global_state_root: {}, state_paths: [{}] }}",
            self.block_height,
            self.global_state_root,
            self.state_paths.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(", ")
        )
    }
}

impl<N: Network> Debug for StateProofs<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}
