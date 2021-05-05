#![allow(dead_code)]

use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use solana_program::{
    instruction::{Instruction as SolanaInstruction},
    program_error::ProgramError,
    pubkey::Pubkey,
};

/// Instructions supported by the Fee-Relayer program.
#[repr(C)]
#[derive(Debug, PartialEq, BorshSerialize, BorshDeserialize, BorshSchema)]
pub enum Instruction {
    /// Instruction to initialize
    Initialize,
}

/// Creates an 'initialize' instruction (`Instruction::Initialize`)
pub fn initialize(program_id: Pubkey) -> Result<SolanaInstruction, ProgramError> {
    let init_data = Instruction::Initialize;
    let data = init_data.try_to_vec()?;
    let accounts = vec![];
    Ok(SolanaInstruction {
        program_id,
        accounts,
        data,
    })
}
