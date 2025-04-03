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

impl<N: Network> Record<N, Plaintext<N>> {
    /// Returns the record commitment.
    pub fn to_commitment(&self, program_id: &ProgramID<N>, record_name: &Identifier<N>) -> Result<Field<N>> {
        println!("🔍 Generating record commitment:");
        println!("  - Program ID: {}", program_id);
        println!("  - Record name: {}", record_name);
        println!("  - Record: {}", self);

        // Construct the input as `(program_id || record_name || record)`.
        let input = to_bits_le![program_id, record_name, self];

        // Compute the BHP hash of the program record.
        let commitment = N::hash_bhp1024(&input)?;
        println!("✨ Generated commitment: {}", commitment);
        Ok(commitment)
    }
}

impl<N: Network> Record<N, Ciphertext<N>> {
    /// Returns the record commitment.
    pub fn to_commitment(&self, _program_id: &ProgramID<N>, _record_name: &Identifier<N>) -> Result<Field<N>> {
        println!("❌ Attempted to generate commitment for Ciphertext record");
        bail!("Illegal operation: Record::to_commitment() cannot be invoked on the `Ciphertext` variant.")
    }
}
