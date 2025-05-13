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

use super::*;
use snarkvm_utilities::{FromBytes, ToBytes};

impl<N: Network> Serialize for StateProofs<N> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match serializer.is_human_readable() {
            true => serializer.collect_str(self),
            false => ToBytesSerializer::serialize_with_size_encoding(self, serializer),
        }
    }
}

impl<'de, N: Network> Deserialize<'de> for StateProofs<N> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        match deserializer.is_human_readable() {
            true => FromStr::from_str(&String::deserialize(deserializer)?).map_err(de::Error::custom),
            false => FromBytesDeserializer::<Self>::deserialize_with_size_encoding(deserializer, "state proofs"),
        }
    }
}

impl<N: Network> ToBytes for StateProofs<N> {
    fn write_le<W: Write>(&self, mut writer: W) -> IoResult<()> {
        self.block_height.write_le(&mut writer)?;
        self.global_state_root.write_le(&mut writer)?;
        // Write the number of state paths
        (self.state_paths.len() as u32).write_le(&mut writer)?;
        // Write each state path
        for path in &self.state_paths {
            path.write_le(&mut writer)?;
        }
        Ok(())
    }
}

impl<N: Network> FromBytes for StateProofs<N> {
    fn read_le<R: Read>(mut reader: R) -> IoResult<Self> {
        let block_height = u32::read_le(&mut reader)?;
        let global_state_root = N::StateRoot::read_le(&mut reader)?;
        // Read the number of state paths
        let num_paths = u32::read_le(&mut reader)? as usize;
        // Read each state path
        let mut state_paths = Vec::with_capacity(num_paths);
        for _ in 0..num_paths {
            state_paths.push(StatePath::<N>::read_le(&mut reader)?);
        }
        Ok(Self::new(block_height, global_state_root, state_paths))
    }
}
