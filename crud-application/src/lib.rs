use borsh::{BorshDeserialize,BorshSerialize};

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
    program_error::ProgramError,
    program::{invoke_signed},
    system_instruction,
    rent::Rent,
    sysvar::Sysvar,
};

// journal Entry Structure
#[derive(Debug,BorshSerialize,BorshDeserialize)]
pub struct JournalEntryState {
    pub owner:Pubkey,
    pub topic:String,
    pub description:String,
}

// Entry Point
entrypoint!(process_instruction);

fn process_instruction(
    _program_id:&Pubkey,
    accounts:&[AccountInfo],
    instruction_data:&[u8],
)->ProgramResult{
    let (tag,rest) = instruction_data
    .split_first()
    .ok_or(ProgramError::InvalidInstructionData)?;

    match tag {
        0 => create_journal_entry(_program_id,accounts,rest),
        1 => update_journal(_program_id,accounts,rest),
        2 => delete_journal(_program_id,accounts,rest),
        _ =>Err(ProgramError::InvalidInstructionData),
    }
}

// creating a function which will parse the data from instruction data -- topic and description
fn parse_topic_description(input:&[u8])->Result<(String,String),ProgramError>{
    let mut input = input;
    let topic_len = u32::from_le_bytes(input[..4].try_into().unwrap()) as usize;
    input = &input[4..];
    let topic = String::from_utf8(input[..topic_len].to_vec()).unwrap();
    input = &input[topic_len..];

    let desc_len = u32::from_le_bytes(input[..4].try_into().unwrap()) as usize ;

    input = &input[4..];
    let description = String::from_utf8(input[..desc_len].to_vec()).unwrap();
    Ok((topic,description))
}

fn create_journal_entry(
    _program_id:&Pubkey,
    accounts:&[AccountInfo],
    input:&[u8],
)->ProgramResult{

    let account_info_iter = &mut accounts.iter();
    let journal_account = next_account_info(account_info_iter)?;
    let owner_account = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    let (topic,description) = parse_topic_description(input)?;

    msg!("Journal Entry Created");
    msg!("Title is {} ",topic);
    msg!("Message is {} ",description);

    let journal_data = JournalEntryState {
        owner:*owner_account.key,
        topic,
        description,
    };
    let space = 32 + 4 + journal_data.topic.len() + 4 + journal_data.description.len();
    let rent = Rent::get()?.minimum_balance(space);

    invoke_signed(
        &system_instruction::create_account(
            owner_account.key, journal_account.key, rent, space as u64, program_id)
            , &[owner_account.clone(),journal_account.clone(),system_program.clone()],
             &[&[journal_data.topic.as_bytes(),owner_account.key.as_ref(),&[find_bump(&journal_data.topic,owner_account.key,program_id)?]]]
    )?;

    journal_data.serialize(&mut &mut journal_account.data.borrow_mut()[..])?;
    Ok(())
}

fn update_journal(
    _program_id:&Pubkey,
    accounts:&[AccountInfo],
    input:&[u8],
)->ProgramResult{
    let account_info_iter = &mut accounts.iter();
    let journal_account = next_account_info(account_info_iter)?;
    let owner_account = next_account_info(account_info_iter)?;

    let (_topic,description) = parse_topic_description(input)?;
    msg!("Updating the Journal");
    msg!("New Description: {}", description);
    
    let mut journal_data = JournalEntryState::try_from_slice(&journal_account.data.borrow())?;
    if journal_data.owner != *owner_account.key {
        return Err(ProgramError::IllegalOwner);
    }

    journal_data.description = description;
    journal_data.serialize(&mut &mut journal_account.data.borrow_mut()[..])?;
    Ok(())
}

fn delete_journal(
    _program_id:&Pubkey,
    accounts:&[AccountInfo],
    input:&[u8]
)->ProgramResult{
    let account_info_iter = &mut accounts.iter();
    let journal_account = next_account_info(account_info_iter)?;
    let owner_account = next_account_info(account_info_iter)?;
    
    let (topic,_) = parse_topic_description(input)?;


    msg!("Deleting the Journal");
    msg!("Title is {}", topic);
    if *owner_account.key != JournalEntryState::try_from_slice(&journal_account.data.borrow())?.owner {
        return Err(ProgramError::IllegalOwner);
    }

    **owner_account.lamports.borrow_mut() +=**journal_account.lamports.borrow();
    **journal_account.lamports.borrow_mut() =0;
    journal_account.data.borrow_mut().fill(0);
    Ok(())
}

// Dummy bump finder
fn find_bump(topic: &str, owner: &Pubkey, program_id: &Pubkey) -> Result<u8, ProgramError> {
    for bump in 0..=255 {
        let seed = &[topic.as_bytes(), owner.as_ref(), &[bump]].concat();
        if Pubkey::create_program_address(&[&seed[..topic.len()], owner.as_ref(), &[bump]], program_id).is_ok() {
            return Ok(bump);
        }
    }
    Err(ProgramError::InvalidSeeds)
}