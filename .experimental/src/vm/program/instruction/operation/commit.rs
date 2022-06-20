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

use crate::vm::{CircuitValue, Opcode, Operand, Program, Stack, StackValue};
use console::{
    network::prelude::*,
    program::{Literal, LiteralType, Plaintext, PlaintextType, Register, RegisterType},
};

use core::marker::PhantomData;

pub trait CommitOperation<N: Network, A: circuit::Aleo<Network = N>> {
    /// The opcode of the operation.
    const OPCODE: Opcode;

    /// Returns the result of committing to the given input and randomizer.
    fn evaluate(input: StackValue<N>, randomizer: StackValue<N>) -> Result<StackValue<N>>;

    /// Returns the result of committing to the given circuit input and randomizer.
    fn execute(input: CircuitValue<A>, randomizer: CircuitValue<A>) -> Result<CircuitValue<A>>;

    /// Returns the output type from the given input types.
    fn output_type() -> Result<RegisterType<N>>;
}

/// BHP256 is a collision-resistant function that processes inputs in 256-bit chunks.
pub type CommitBHP256<N, A> = CommitInstruction<N, A, BHPCommitOperation<N, A, 256>>;
/// BHP512 is a collision-resistant function that processes inputs in 512-bit chunks.
pub type CommitBHP512<N, A> = CommitInstruction<N, A, BHPCommitOperation<N, A, 512>>;
/// BHP768 is a collision-resistant function that processes inputs in 768-bit chunks.
pub type CommitBHP768<N, A> = CommitInstruction<N, A, BHPCommitOperation<N, A, 768>>;
/// BHP1024 is a collision-resistant function that processes inputs in 1024-bit chunks.
pub type CommitBHP1024<N, A> = CommitInstruction<N, A, BHPCommitOperation<N, A, 1024>>;

/// The BHP commitment operation template.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct BHPCommitOperation<N: Network, A: circuit::Aleo<Network = N>, const NUM_BITS: u16>(PhantomData<(N, A)>);

impl<N: Network, A: circuit::Aleo<Network = N>, const NUM_BITS: u16> CommitOperation<N, A>
    for BHPCommitOperation<N, A, NUM_BITS>
{
    /// The opcode of the operation.
    const OPCODE: Opcode = match NUM_BITS {
        256 => Opcode::Commit("commit.bhp256"),
        512 => Opcode::Commit("commit.bhp512"),
        768 => Opcode::Commit("commit.bhp768"),
        1024 => Opcode::Commit("commit.bhp1024"),
        _ => panic!("Invalid BHP commit instruction opcode"),
    };

    /// Returns the result of committing to the given input and randomizer.
    fn evaluate(input: StackValue<N>, randomizer: StackValue<N>) -> Result<StackValue<N>> {
        // Convert the input into bits.
        let preimage: Vec<bool> = match input {
            StackValue::Plaintext(Plaintext::Literal(literal, ..)) => {
                [literal.variant().to_bits_le(), literal.to_bits_le()].into_iter().flatten().collect()
            }
            StackValue::Plaintext(Plaintext::Interface(interface, ..)) => interface
                .into_iter()
                .flat_map(|(member_name, member_value)| {
                    [member_name.to_bits_le(), member_value.to_bits_le()].into_iter().flatten()
                })
                .collect(),
            StackValue::Record(record) => record
                .owner()
                .to_bits_le()
                .into_iter()
                .chain(record.balance().to_bits_le().into_iter())
                .chain(record.data().iter().flat_map(|(entry_name, entry_value)| {
                    [entry_name.to_bits_le(), entry_value.to_bits_le()].into_iter().flatten()
                }))
                .collect(),
        };

        // Retrieve the randomizer.
        let randomizer = match randomizer {
            StackValue::Plaintext(Plaintext::Literal(Literal::Scalar(randomizer), ..)) => randomizer,
            _ => bail!("Invalid randomizer type for BHP commit"),
        };

        // Compute the commitment.
        let output = match NUM_BITS {
            256 => N::commit_bhp256(&preimage, &randomizer)?,
            512 => N::commit_bhp512(&preimage, &randomizer)?,
            768 => N::commit_bhp768(&preimage, &randomizer)?,
            1024 => N::commit_bhp1024(&preimage, &randomizer)?,
            _ => bail!("Invalid BHP commitment variant: BHP{}", NUM_BITS),
        };
        // Return the output as a stack value.
        Ok(StackValue::Plaintext(Plaintext::Literal(Literal::Field(output), Default::default())))
    }

    /// Returns the result of committing to the given circuit input and randomizer.
    fn execute(input: CircuitValue<A>, randomizer: CircuitValue<A>) -> Result<CircuitValue<A>> {
        use circuit::ToBits;

        // Convert the input into bits.
        let preimage: Vec<circuit::types::Boolean<A>> = match input {
            CircuitValue::Plaintext(circuit::Plaintext::Literal(literal, ..)) => {
                [literal.variant().to_bits_le(), literal.to_bits_le()].into_iter().flatten().collect()
            }
            CircuitValue::Plaintext(circuit::Plaintext::Interface(interface, ..)) => interface
                .iter()
                .flat_map(|(member_name, member_value)| {
                    [member_name.to_bits_le(), member_value.to_bits_le()].into_iter().flatten()
                })
                .collect(),
            CircuitValue::Record(record) => record
                .owner()
                .to_bits_le()
                .into_iter()
                .chain(record.balance().to_bits_le().into_iter())
                .chain(record.data().iter().flat_map(|(entry_name, entry_value)| {
                    [entry_name.to_bits_le(), entry_value.to_bits_le()].into_iter().flatten()
                }))
                .collect(),
        };

        // Retrieve the randomizer.
        let randomizer = match randomizer {
            CircuitValue::Plaintext(circuit::Plaintext::Literal(circuit::Literal::Scalar(randomizer), ..)) => {
                randomizer
            }
            _ => bail!("Invalid randomizer type for BHP commit"),
        };

        // Compute the commitment.
        let output = match NUM_BITS {
            256 => A::commit_bhp256(&preimage, &randomizer),
            512 => A::commit_bhp512(&preimage, &randomizer),
            768 => A::commit_bhp768(&preimage, &randomizer),
            1024 => A::commit_bhp1024(&preimage, &randomizer),
            _ => bail!("Invalid BHP commitment variant: BHP{}", NUM_BITS),
        };
        // Return the output as a stack value.
        Ok(CircuitValue::Plaintext(circuit::Plaintext::Literal(circuit::Literal::Field(output), Default::default())))
    }

    /// Returns the output type from the given input types.
    fn output_type() -> Result<RegisterType<N>> {
        Ok(RegisterType::Plaintext(PlaintextType::Literal(LiteralType::Field)))
    }
}

/// Commits the operand into the declared type.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CommitInstruction<N: Network, A: circuit::Aleo<Network = N>, O: CommitOperation<N, A>> {
    /// The operands as `(input, randomizer)`.
    operands: Vec<Operand<N>>,
    /// The destination register.
    destination: Register<N>,
    /// PhantomData.
    _phantom: PhantomData<(A, O)>,
}

impl<N: Network, A: circuit::Aleo<Network = N>, O: CommitOperation<N, A>> CommitInstruction<N, A, O> {
    /// Returns the opcode.
    #[inline]
    pub const fn opcode() -> Opcode {
        O::OPCODE
    }

    /// Returns the operands in the operation.
    #[inline]
    pub fn operands(&self) -> &[Operand<N>] {
        // Sanity check that the operands is exactly two inputs.
        debug_assert!(self.operands.len() == 2, "Commit operations must have two operands");
        // Return the operands.
        &self.operands
    }

    /// Returns the destination register.
    #[inline]
    pub fn destinations(&self) -> Vec<Register<N>> {
        vec![self.destination.clone()]
    }
}

impl<N: Network, A: circuit::Aleo<Network = N>, O: CommitOperation<N, A>> CommitInstruction<N, A, O> {
    /// Evaluates the instruction.
    #[inline]
    pub fn evaluate(&self, stack: &mut Stack<N, A>) -> Result<()> {
        // Ensure the number of operands is correct.
        if self.operands.len() != 2 {
            bail!("Instruction '{}' expects 2 operands, found {} operands", Self::opcode(), self.operands.len())
        }
        // Load the operands values.
        let inputs: Vec<_> = self.operands.iter().map(|operand| stack.load(operand)).try_collect()?;
        // Retrieve the input and randomizer.
        let (input, randomizer) = (inputs[0].clone(), inputs[1].clone());
        // Compute the commitment.
        let commitment = O::evaluate(input, randomizer)?;
        // Store the commitment.
        stack.store(&self.destination, commitment)
    }

    /// Executes the instruction.
    #[inline]
    pub fn execute(&self, stack: &mut Stack<N, A>) -> Result<()> {
        // Ensure the number of operands is correct.
        if self.operands.len() != 2 {
            bail!("Instruction '{}' expects 2 operands, found {} operands", Self::opcode(), self.operands.len())
        }
        // Load the operands values.
        let inputs: Vec<_> = self.operands.iter().map(|operand| stack.load_circuit(operand)).try_collect()?;
        // Retrieve the input and randomizer.
        let (input, randomizer) = (inputs[0].clone(), inputs[1].clone());
        // Compute the commitment.
        let commitment = O::execute(input, randomizer)?;
        // Store the commitment.
        stack.store_circuit(&self.destination, commitment)
    }

    /// Returns the output type from the given program and input types.
    #[inline]
    pub fn output_types(
        &self,
        _program: &Program<N, A>,
        input_types: &[RegisterType<N>],
    ) -> Result<Vec<RegisterType<N>>> {
        // Ensure the number of input types is correct.
        if input_types.len() != 2 {
            bail!("Instruction '{}' expects 2 inputs, found {} inputs", Self::opcode(), input_types.len())
        }
        // Ensure the number of operands is correct.
        if self.operands.len() != 2 {
            bail!("Instruction '{}' expects 2 operands, found {} operands", Self::opcode(), self.operands.len())
        }

        // TODO (howardwu): If the operation is Pedersen, check that it is within the number of bits.

        Ok(vec![O::output_type()?])
    }
}

impl<N: Network, A: circuit::Aleo<Network = N>, O: CommitOperation<N, A>> Parser for CommitInstruction<N, A, O> {
    /// Parses a string into an operation.
    #[inline]
    fn parse(string: &str) -> ParserResult<Self> {
        // Parse the opcode from the string.
        let (string, _) = tag(*Self::opcode())(string)?;
        // Parse the whitespace from the string.
        let (string, _) = Sanitizer::parse_whitespaces(string)?;
        // Parse the first operand from the string.
        let (string, first) = Operand::parse(string)?;
        // Parse the whitespace from the string.
        let (string, _) = Sanitizer::parse_whitespaces(string)?;
        // Parse the second operand from the string.
        let (string, second) = Operand::parse(string)?;
        // Parse the whitespace from the string.
        let (string, _) = Sanitizer::parse_whitespaces(string)?;
        // Parse the "into" from the string.
        let (string, _) = tag("into")(string)?;
        // Parse the whitespace from the string.
        let (string, _) = Sanitizer::parse_whitespaces(string)?;
        // Parse the destination register from the string.
        let (string, destination) = Register::parse(string)?;

        Ok((string, Self { operands: vec![first, second], destination, _phantom: PhantomData }))
    }
}

impl<N: Network, A: circuit::Aleo<Network = N>, O: CommitOperation<N, A>> FromStr for CommitInstruction<N, A, O> {
    type Err = Error;

    /// Parses a string into an operation.
    #[inline]
    fn from_str(string: &str) -> Result<Self> {
        match Self::parse(string) {
            Ok((remainder, object)) => {
                // Ensure the remainder is empty.
                ensure!(remainder.is_empty(), "Failed to parse string. Found invalid character in: \"{remainder}\"");
                // Return the object.
                Ok(object)
            }
            Err(error) => bail!("Failed to parse string. {error}"),
        }
    }
}

impl<N: Network, A: circuit::Aleo<Network = N>, O: CommitOperation<N, A>> Debug for CommitInstruction<N, A, O> {
    /// Prints the operation as a string.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl<N: Network, A: circuit::Aleo<Network = N>, O: CommitOperation<N, A>> Display for CommitInstruction<N, A, O> {
    /// Prints the operation to a string.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // Ensure the number of operands is 2.
        if self.operands.len() != 2 {
            eprintln!("The number of operands must be 2, found {}", self.operands.len());
            return Err(fmt::Error);
        }
        // Print the operation.
        write!(f, "{} ", Self::opcode())?;
        self.operands.iter().try_for_each(|operand| write!(f, "{} ", operand))?;
        write!(f, "into {}", self.destination)
    }
}

impl<N: Network, A: circuit::Aleo<Network = N>, O: CommitOperation<N, A>> FromBytes for CommitInstruction<N, A, O> {
    /// Reads the operation from a buffer.
    fn read_le<R: Read>(mut reader: R) -> IoResult<Self> {
        // Initialize the vector for the operands.
        let mut operands = Vec::with_capacity(2);
        // Read the operands.
        for _ in 0..2 {
            operands.push(Operand::read_le(&mut reader)?);
        }
        // Read the destination register.
        let destination = Register::read_le(&mut reader)?;

        // Return the operation.
        Ok(Self { operands, destination, _phantom: PhantomData })
    }
}

impl<N: Network, A: circuit::Aleo<Network = N>, O: CommitOperation<N, A>> ToBytes for CommitInstruction<N, A, O> {
    /// Writes the operation to a buffer.
    fn write_le<W: Write>(&self, mut writer: W) -> IoResult<()> {
        // Ensure the number of operands is 2.
        if self.operands.len() != 2 {
            return Err(error(format!("The number of operands must be 2, found {}", self.operands.len())));
        }
        // Write the operands.
        self.operands.iter().try_for_each(|operand| operand.write_le(&mut writer))?;
        // Write the destination register.
        self.destination.write_le(&mut writer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use circuit::network::AleoV0;
    use console::network::Testnet3;

    type CurrentNetwork = Testnet3;
    type CurrentAleo = AleoV0;

    #[test]
    fn test_parse() {
        let (string, commit) =
            CommitBHP512::<CurrentNetwork, CurrentAleo>::parse("commit.bhp512 r0 r1 into r2").unwrap();
        assert!(string.is_empty(), "Parser did not consume all of the string: '{string}'");
        assert_eq!(commit.operands.len(), 2, "The number of operands is incorrect");
        assert_eq!(commit.operands[0], Operand::Register(Register::Locator(0)), "The first operand is incorrect");
        assert_eq!(commit.operands[1], Operand::Register(Register::Locator(1)), "The second operand is incorrect");
        assert_eq!(commit.destination, Register::Locator(2), "The destination register is incorrect");
    }
}
