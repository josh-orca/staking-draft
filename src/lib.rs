use instructions::VaultInstructions;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::{entrypoint, pubkey};

mod instructions;
use instructions::*;

// Always define program IDs as constants using `pubkey!`.
// This avoids runtime costs of deriving keys dynamically.
const ID: Pubkey = pubkey!("11111111111111111111111111111111"); // Dummy program ID for development

entrypoint!(process_instruction); // Macro that declares `process_instruction` as the program's entry point.

// Main function to process instructions.
pub fn process_instruction(
    program_id: &Pubkey,      // Reference to the program ID.
    accounts: &[AccountInfo], // List of accounts involved in the transaction.
    data: &[u8],              // Serialized instruction data (byte array).
) -> ProgramResult {
    // Ensure the program ID matches the expected value. This prevents hijacking by another program.
    if program_id != &crate::ID {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Parse the instruction discriminator and its associated data.
    // 'split_first' separtes the first byte (discriminator) from the rest (payload).
    let (discriminator, data) = data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    let amount = u64::from_le_bytes([
        data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
    ]);

    match VaultInstructions::try_from(discriminator)? {
        VaultInstructions::Deposit => deposit::process(accounts, amount),
        VaultInstructions::Withdraw => withdraw::process(accounts, amount),
    }
}
