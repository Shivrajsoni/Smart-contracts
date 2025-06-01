// create a PDA account and also a CPI Program Contract
//and a user Account also required

use std::iter::Inspect;

use borsh::{BorshDeserialize, BorshSerialize};

use solana_program::{
    account_info::{next_account_info, AccountInfo}, entrypoint::{self, ProgramResult}, program::invoke_signed, pubkey::Pubkey, system_instruction, system_program
};

entrypoint!(process_instruction);

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let iter = &mut accounts.iter();
    let pda_acc = next_account_info(iter)?;
    let user_acc = next_account_info(iter)?;
    let system_program = next_account_info(iter)?;

    let ix = system_instruction::create_account(
        user_acc.key,
        pda_acc.key,
        1000_000_000,
        8,
        _program_id
    );

    let seeds = &[user_acc.key.as_ref(),b"user_acc_1"];
    let (pda,bump) = Pubkey::find_program_address(seeds, _program_id);
    // msg!("Pda is {} and bump is {}",pda,bump);
    invoke_signed(&ix, accounts, &[seeds,&[bump]]);
    
    Ok(())
}
