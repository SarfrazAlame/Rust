use borsh::{BorshSerialize,BorshDeserialize};
use solana_program :: {
    account_info::{next_account_info, AccountInfo}, entrypoint, entrypoint::{self,  ProgramResult}, msg, pubkey::Pubkey
};

#[derive(BorshSerialize, BorshDeserialize)]
enum InstructionType {
    Increment(u32),
    Decrement(u32)
}

#[derive(BorshSerialize, BorshDeserialize)];
struct Counter {
    count:u32,
}

entrypoint!(counter_contract);

pub fn counter_contract(
    program_id:&Pubkey,
    accounts:&[AccountInfo],
    instruction_data: &[u8],
)-> ProgramResult{
    let acc = next_account_info(&mut accounts.iter())?;

    let instruction_type = InstructionType::try_from_slice(instruction_data)?;
    let mut counter_data = Counter::try_from_slice(&acc.data.borrow())?;

    match instruction_type {
        msg!("Executing increase");
        InstructionType::Increment(value)=>{
            counter_data.count += value;
        }
        msg!("Executing decrease");
        InstructionType::Decrement(value)=>{
            counter_data.count -= value;
        }
    }

    counter_data.serialize(&mut *acc.data.borrow_mut());

    msg!("Contract succeded");
    Ok(())
}
