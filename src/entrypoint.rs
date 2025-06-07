use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

use crate::{instruction::FenerbahceInstruction, processor::Processor};

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = FenerbahceInstruction::unpack(instruction_data)?;
    Processor::process(program_id, accounts, instruction)
}