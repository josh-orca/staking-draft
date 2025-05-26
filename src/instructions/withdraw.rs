use solana_program::{
    account_info::{AccountInfo, next_account_info}, // Import next_account_info
    entrypoint::ProgramResult,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use spl_token::{self, ID as SPL_TOKEN_PROGRAM_ID, instruction::transfer};

// Processes the withdraw instruction for SPL tokens.
pub fn process(accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    // Create an iterator for the accounts to safely extract them.
    let accounts_iter = &mut accounts.iter();

    // Extract and label each required account using next_account_info.
    let vault_token_account = next_account_info(accounts_iter)?; // The program's vault token account (PDA)
    let signer_token_account = next_account_info(accounts_iter)?; // User's token account for the stake token
    let signer_authority = next_account_info(accounts_iter)?; // The user's authority account (used as a seed for PDA)
    let spl_token_program_account = next_account_info(accounts_iter)?; // SPL Token Program ID

    // Ensure the token program ID is correct
    if spl_token_program_account.key != &SPL_TOKEN_PROGRAM_ID {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Derive the Program Derived Address (PDA) for the signer authority and validate it.
    // This PDA will be the authority for the vault's token account, allowing the program
    // to sign the transfer instruction.
    let (pda, bump) =
        Pubkey::try_find_program_address(&[signer_authority.key.as_ref()], &crate::ID)
            .ok_or(ProgramError::InvalidSeeds)?;

    // Perform the transfer of SPL tokens from the vault's token account back to the signer's token account.
    // The `invoke_signed` function is used because the transfer is authorized by the PDA,
    // which requires the program to sign using its seeds.
    invoke_signed(
        &transfer(
            spl_token_program_account.key, // SPL Token program ID
            vault_token_account.key,       // Source token account (the vault's)
            signer_token_account.key,      // Destination token account (the signer's)
            &pda,                          // Authority for the source token account (the PDA)
            &[],    // Signers (empty because we're using invoke_signed with seeds)
            amount, // Amount of tokens to transfer
        )?,
        &[
            vault_token_account.clone(),
            signer_token_account.clone(),
            signer_authority.clone(), // This account is crucial as its key is used as a seed for the PDA
            spl_token_program_account.clone(),
        ], // Pass all required account references for the transfer instruction
        &[&[signer_authority.key.as_ref(), &[bump]]], // Include the PDA seeds for signing the instruction.
                                                      // The bump is necessary to ensure the PDA is unique and valid.
    )?;

    Ok(())
}
