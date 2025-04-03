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
use snarkvm_utilities::{FromBytes, ToBytes}; // 👈 This is the only addition you need

impl<N: Network> Serialize for StateProofsResponse<N> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match serializer.is_human_readable() {
            true => serializer.collect_str(self),
            false => ToBytesSerializer::serialize_with_size_encoding(self, serializer),
        }
    }
}

impl<'de, N: Network> Deserialize<'de> for StateProofsResponse<N> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        match deserializer.is_human_readable() {
            true => FromStr::from_str(&String::deserialize(deserializer)?).map_err(de::Error::custom),
            false => FromBytesDeserializer::<Self>::deserialize_with_size_encoding(deserializer, "state proofs response"),
        }
    }
}

impl<N: Network> Display for StateProofsResponse<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "StateProofsResponse {{ block_height: {}, global_state_root: {}, state_paths: [{}] }}",
            self.block_height,
            self.global_state_root,
            self.state_paths
                .iter()
                .map(|path| path.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl<N: Network> FromStr for StateProofsResponse<N> {
    type Err = String;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        Err("FromStr not implemented for StateProofsResponse".to_string())
    }
}


impl<N: Network> ToBytes for StateProofsResponse<N> {
    fn write_le<W: Write>(&self, mut writer: W) -> IoResult<()> {
        self.block_height.write_le(&mut writer)?;
        self.global_state_root.write_le(&mut writer)?;
        self.state_paths.write_le(&mut writer)?;
        Ok(())
    }
}

impl<N: Network> FromBytes for StateProofsResponse<N> {
    fn read_le<R: Read>(mut reader: R) -> IoResult<Self> {
        let block_height = u32::read_le(&mut reader)?;
        let global_state_root = N::StateRoot::read_le(&mut reader)?;

        // Manually read the length of the Vec<StatePath<N>>
        let num_paths = u64::read_le(&mut reader)? as usize;

        let mut state_paths = Vec::with_capacity(num_paths);
        for _ in 0..num_paths {
            state_paths.push(StatePath::<N>::read_le(&mut reader)?);
        }

        Ok(Self {
            block_height,
            global_state_root,
            state_paths,
        })
    }
}


impl<N: Network> Serialize for StatePath<N> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match serializer.is_human_readable() {
            true => serializer.collect_str(self),
            false => ToBytesSerializer::serialize_with_size_encoding(self, serializer),
        }
    }
}

impl<'de, N: Network> Deserialize<'de> for StatePath<N> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        match deserializer.is_human_readable() {
            true => FromStr::from_str(&String::deserialize(deserializer)?).map_err(de::Error::custom),
            false => FromBytesDeserializer::<Self>::deserialize_with_size_encoding(deserializer, "state path"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use snarkvm_console_network::MainnetV0;

    type CurrentNetwork = MainnetV0;

    const ITERATIONS: usize = 100;

    #[test]
    fn test_serde_json() -> Result<()> {
        let mut rng = TestRng::default();

        for _ in 0..ITERATIONS {
            let expected = crate::state_path::test_helpers::sample_global_state_path::<CurrentNetwork>(None, &mut rng)?;

            let expected_string = &expected.to_string();
            let candidate_string = serde_json::to_string(&expected)?;
            assert_eq!(expected_string, serde_json::Value::from_str(&candidate_string)?.as_str().unwrap());

            assert_eq!(expected, StatePath::from_str(expected_string)?);
            assert_eq!(expected, serde_json::from_str(&candidate_string)?);
        }
        Ok(())
    }

    #[test]
    fn test_bincode() -> Result<()> {
        let mut rng = TestRng::default();

        for _ in 0..ITERATIONS {
            let expected = crate::state_path::test_helpers::sample_global_state_path::<CurrentNetwork>(None, &mut rng)?;

            let expected_bytes = expected.to_bytes_le()?;
            let expected_bytes_with_size_encoding = bincode::serialize(&expected)?;
            assert_eq!(&expected_bytes[..], &expected_bytes_with_size_encoding[8..]);

            assert_eq!(expected, StatePath::read_le(&expected_bytes[..])?);
            assert_eq!(expected, bincode::deserialize(&expected_bytes_with_size_encoding[..])?);
        }
        Ok(())
    }
}
