pub mod deposit; // Deposit handler
pub mod withdraw; // Withdraw handler

use solana_program::program_error::ProgramError;

// Enum representing possible vault instructions
pub enum VaultInstructions {
    Deposit,  // Instruction to deposit SPL token into the vault
    Withdraw, // Instruction to withdraw SPL token from the vault
}

// Convert a discriminator byte into a 'VaultInstructions' variant
impl TryFrom<&u8> for VaultInstructions {
    type Error = ProgramError;

    fn try_from(discriminator: &u8) -> Result<Self, Self::Error> {
        match discriminator {
            0 => Ok(VaultInstructions::Deposit),
            1 => Ok(VaultInstructions::Withdraw),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
