
use borsh::{
    BorshDeserialize,
    BorshSerialize
};
use solana_program::{
    entrypoint, 
    account_info::{
        next_account_info,
        AccountInfo
    }, 
    entrypoint::ProgramResult, 
    pubkey::Pubkey,
    program::invoke,
    instruction::{Instruction, AccountMeta}
};

entrypoint!(process_instruction);

fn process_instruction(
    _program_id:&Pubkey,
    accounts:&[AccountInfo],
    instruction_data:&[u8]
)->ProgramResult {    

    // i need to make a contract which calls another contract while performing
    let mut account_info_iter = accounts.iter();
    let data_account = next_account_info(&mut account_info_iter)?;
    let contract_account = next_account_info(&mut account_info_iter)?;

    let instruction = Instruction {
        program_id:*contract_account.key,
        accounts: vec![
            AccountMeta{
                is_signer:true,
                is_writable:true,
                pubkey:*data_account.key
            }
        ],
        data:vec![],
    };

    invoke(&instruction,&[data_account.clone()])?;

    
    Ok(())
}