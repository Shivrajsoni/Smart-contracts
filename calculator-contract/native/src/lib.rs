
use solana_program::{
    entrypoint, 
    account_info::{
        next_account_info,
        AccountInfo
    }, 
    entrypoint::ProgramResult, 
    pubkey::Pubkey,
    program_error::ProgramError,
};
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize)]
pub struct CounterState {
    pub count: u32,
}

#[derive(BorshDeserialize, BorshSerialize)]
enum Instruction {
    Init,
    Half,
    Double,
    Add {amount:u32},
    Subtract {amount :u32},
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = Instruction::try_from_slice(instruction_data)?;

    let mut iter = accounts.iter();
    let data_account = next_account_info(&mut iter)?;

    if !data_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    let mut counter_state = CounterState::try_from_slice(&data_account.data.borrow())?;

    match instruction {
        Instruction::Init => {
            counter_state.count = 1;
        },
        Instruction::Double => {
            counter_state.count = counter_state.count.saturating_mul(2);
        },
        Instruction::Half => {
            counter_state.count = counter_state.count / 2;
        },
        Instruction::Add { amount } => {
            counter_state.count = counter_state.count.saturating_add(amount);
        },
        Instruction::Subtract { amount } => {
            counter_state.count = counter_state.count.saturating_sub(amount);
        },
        
    };
Ok(())
}
    
    

