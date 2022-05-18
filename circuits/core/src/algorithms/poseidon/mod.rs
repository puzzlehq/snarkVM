// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

mod hash;
mod hash_many;
mod hash_to_scalar;
mod prf;

#[cfg(test)]
use snarkvm_circuits_types::environment::assert_scope;

use crate::algorithms::{Hash, HashMany, HashToScalar, PRF};
use snarkvm_algorithms::DuplexSpongeMode;
use snarkvm_circuits_types::{environment::prelude::*, Field, Scalar};
use snarkvm_fields::PoseidonDefaultField;

const CAPACITY: usize = 1;

/// Poseidon2 is a cryptographic hash function of input rate 2.
pub type Poseidon2<E> = Poseidon<E, 2>;
/// Poseidon4 is a cryptographic hash function of input rate 4.
pub type Poseidon4<E> = Poseidon<E, 4>;
/// Poseidon8 is a cryptographic hash function of input rate 8.
pub type Poseidon8<E> = Poseidon<E, 8>;

#[derive(Clone)]
pub struct Poseidon<E: Environment, const RATE: usize> {
    /// The number of rounds in a full-round operation.
    full_rounds: usize,
    /// The number of rounds in a partial-round operation.
    partial_rounds: usize,
    /// The exponent used in S-boxes.
    alpha: Field<E>,
    /// The additive round keys. These are added before each MDS matrix application to make it an affine shift.
    /// They are indexed by `ark[round_number][state_element_index]`
    ark: Vec<Vec<Field<E>>>,
    /// The Maximally Distance Separating (MDS) matrix.
    mds: Vec<Vec<Field<E>>>,
}

impl<E: Environment, const RATE: usize> Poseidon<E, RATE> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        match E::BaseField::default_poseidon_parameters::<RATE>() {
            Some(parameters) => {
                let full_rounds = parameters.full_rounds;
                let partial_rounds = parameters.partial_rounds;
                let alpha = Field::constant(E::BaseField::from(parameters.alpha as u128));
                // Cache the bits for the field element.
                alpha.to_bits_le();
                let ark = parameters
                    .ark
                    .into_iter()
                    .take(full_rounds + partial_rounds)
                    .map(|round| round.into_iter().take(RATE + 1).map(Field::constant).collect())
                    .collect();
                let mds = parameters
                    .mds
                    .into_iter()
                    .take(RATE + 1)
                    .map(|round| round.into_iter().take(RATE + 1).map(Field::constant).collect())
                    .collect();

                Self { full_rounds, partial_rounds, alpha, ark, mds }
            }
            None => E::halt("Failed to initialize the Poseidon hash function"),
        }
    }
}
