use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program::invoke,
    program_error::ProgramError, pubkey::Pubkey, system_instruction::transfer,
};

/// Processes the deposit instruction.
pub fn process(accounts: &[AccountInfo], lamports: u64) -> ProgramResult {
    // Validate the account array structure
    let [signer, vault, _system_program] = accounts else {
        return Err(ProgramError::InvalidAccountData);
    };

    // Derive the Program Derived Address (PDA) for the vault and validate it.
    let (pda, _bump) = Pubkey::try_find_program_address(&[signer.key.as_ref()], &crate::ID)
        .ok_or(ProgramError::InvalidSeeds)?;
    assert_eq!(&pda, vault.key);

    // Perform the transfer of lamports from the signer to the vault account.
    invoke(
        &transfer(signer.key, vault.key, lamports),
        accounts, // Pass account references required for the transfer
    )
}
