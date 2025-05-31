// cross program Invocation
use solana_program::{account_info::{self, next_account_info, AccountInfo},entrypoint,entrypoint::{ProgramResult}, pubkey::Pubkey};
use borsh::{BorshDeserialize,BorshSerialize};
entrypoint!(process_instruction);


#[derive(BorshSerialize,BorshDeserialize)]
pub struct OnchainData {
    count:u32
}

fn process_instruction(
    _program_id:&Pubkey,
    accounts:&[AccountInfo],
    instruction_data:&[u8]
)->ProgramResult{
    let mut account_info_iter = accounts.iter();
    let data_account = next_account_info(&mut account_info_iter)?;
    
    // match data_account {
    //     Ok(data_account)=>{},
    //     Err(err)=>{return Err(err)}
    // }

    let mut counter = OnchainData::try_from_slice(& data_account.data.borrow_mut())?;

    if counter.count == 0 {
        counter.count = 1;
    }else {
        counter.count = counter.count*2;
    }
    counter.serialize(&mut *data_account.data.borrow_mut());
    return Ok(())
}