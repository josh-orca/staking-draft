use thiserror_no_std::Error;

#[derive(Error, Clone, Debug, Eq, PartialEq)]
pub enum ErrorCode {
    #[error("Unknown instruction discriminator")]
    UnknownInstructionDiscriminator = 6000, // 0x1770

    #[error("Incorrect program id")]
    IncorrectProgramId = 6001, // 0x1771
}

impl From<ErrorCode> for pinocchio::program_error::ProgramError {
    fn from(e: ErrorCode) -> Self {
        pinocchio::program_error::ProgramError::Custom(e as u32)
    }
}
