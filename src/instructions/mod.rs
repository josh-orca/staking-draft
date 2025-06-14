pub mod deposit;
pub mod initialize_staking_pool;
pub mod withdraw;

use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankInstruction;
use strum::{Display, EnumDiscriminants, FromRepr};

#[derive(
    Debug, Clone, BorshSerialize, BorshDeserialize, ShankInstruction, Display, EnumDiscriminants,
)]
#[strum_discriminants(
    name(InstructionDiscriminator),
    derive(BorshSerialize, BorshDeserialize, FromRepr)
)]
pub enum Instruction {
    InitializeStakingPool,
    Deposit { amount: u64 },
    Withdraw { amount: u64 },
}

impl InstructionDiscriminator {
    pub fn to_bytes(&self) -> &[u8; 1] {
        unsafe { &*(self as *const _ as *const [u8; 1]) }
    }
}
