use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program::invoke_signed,
    program_error::ProgramError, pubkey::Pubkey, system_instruction::transfer,
};

// Processes the withdraw instruction.
pub fn process(accounts: &[AccountInfo], lamports: u64) -> ProgramResult {
    // Validate the account array structure
    let [vault, signer, _system_program] = accounts else {
        return Err(ProgramError::InvalidAccountData);
    };

    // Derive the Program Derived Address (PDA) for the vault and validate it.
    let (pda, bump) = Pubkey::try_find_program_address(&[signer.key.as_ref()], &crate::ID)
        .ok_or(ProgramError::InvalidSeeds)?;
    assert_eq!(&pda, vault.key); // Ensure the PDA matches the vault's public key.

    // Perform the transfer of lamports from the vault back to the signer account.
    invoke_signed(
        &transfer(vault.key, signer.key, lamports),
        accounts, // Pass account references required for the transfer/
        &[&[signer.key.as_ref(), &[bump]]], // Include the PDA seeds for signing.
    )
}
