use crate::processor::EscrowProcessor;
use solana_program::{
    account_info::AccountInfo,
    entrypoint::{self, ProgramResult},
    pubkey::Pubkey,
};

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    EscrowProcessor::process(program_id, accounts, instruction_data);
    Ok(())
}
