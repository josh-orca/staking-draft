use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_program,
};
use spl_token::{self, ID as SPL_TOKEN_PROGRAM_ID, instruction as spl_instruction}; // Import SPL Token program ID and instruction

/// Processes the deposit instruction for SPL tokens.
pub fn process(accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    // 1. Get accounts from the `accounts` array
    //    Use `next_account_info` for robust parsing and clear error messages.
    let accounts_iter = &mut accounts.iter();

    let signer_account = next_account_info(accounts_iter)?; // The user initiating the deposit
    let signer_token_account = next_account_info(accounts_iter)?; // User's ATA for the stake token
    let stake_token_mint = next_account_info(accounts_iter)?; // The mint of the token being staked
    let vault_token_account = next_account_info(accounts_iter)?; // The program's vault token account (PDA)
    let system_program_account = next_account_info(accounts_iter)?; // System Program for creating accounts (if needed)
    let spl_token_program_account = next_account_info(accounts_iter)?; // SPL Token Program ID

    // 2. Validate input accounts
    // Ensure signer is actually a signer
    if !signer_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Ensure the token program ID is correct
    if spl_token_program_account.key != &SPL_TOKEN_PROGRAM_ID {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Ensure the system program ID is correct (if you need it)
    if system_program_account.key != &system_program::ID {
        return Err(ProgramError::IncorrectProgramId);
    }

    // 3. Derive and validate the Program Derived Address (PDA) for the vault token account.
    let (expected_vault_pda, _vault_bump) = Pubkey::try_find_program_address(
        &[
            b"stake_vault",                // A custom seed for your stake vault
            stake_token_mint.key.as_ref(), // The mint of the token this vault holds
        ],
        &crate::ID, // Your program ID
    )
    .ok_or(ProgramError::InvalidSeeds)?;

    // Validate that the provided vault_token_account matches the derived PDA
    if &expected_vault_pda != vault_token_account.key {
        return Err(ProgramError::InvalidAccountData); // Or a more specific error
    }

    // 4. Perform the token transfer from the signer's token account to the vault's token account.
    let transfer_instruction = spl_instruction::transfer(
        &SPL_TOKEN_PROGRAM_ID,    // SPL Token Program ID
        signer_token_account.key, // Source token account
        vault_token_account.key,  // Destination token account
        signer_account.key,       // Authority (owner) of the source account
        &[&signer_account.key],   // Signers needed for this instruction (just the user)
        amount,                   // Amount of tokens to transfer
    )?;

    // Invoke the instruction
    invoke(
        &transfer_instruction,
        &[
            signer_token_account.clone(),      // Source account
            vault_token_account.clone(),       // Destination account
            signer_account.clone(),            // Authority (owner) of the source account
            spl_token_program_account.clone(), // SPL Token Program account
        ],
    )?;

    Ok(())
}
