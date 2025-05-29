use borsh::de::BorshDeserialize;
use pinocchio::{ProgramResult, account_info::AccountInfo, pubkey::Pubkey};
use pinocchio_log::log;

use crate::{
    error::ErrorCode,
    instructions::{self, Instruction},
};

#[cfg(target_os = "solana")]
use pinocchio::{default_allocator, default_panic_handler, program_entrypoint};

#[cfg(target_os = "solana")]
program_entrypoint!(process_instruction);

#[cfg(target_os = "solana")]
default_allocator!();

#[cfg(target_os = "solana")]
default_panic_handler!();

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if program_id != &crate::ID {
        return Err(ErrorCode::IncorrectProgramId.into());
    }
    let mut instruction_data = instruction_data;
    let instruction = Instruction::deserialize(&mut instruction_data)
        .map_err(|_| ErrorCode::UnknownInstructionDiscriminator)?;
    log!("Instruction: {}", instruction.to_string().as_str());
    match &instruction {
        Instruction::InitializeStakingPool => {
            instructions::initialize_staking_pool::process_instruction(accounts)?;
        }
        Instruction::Deposit => {
            instructions::deposit::process_instruction()?;
        }
        Instruction::Withdraw {} => {
            instructions::withdraw::process_instruction()?;
        }
    }
    Ok(())
}
