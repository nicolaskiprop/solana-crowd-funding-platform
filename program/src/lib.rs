use borsh::{BorshDeserialize, BorshSerialize};

// #[cfg(test)]

// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }


use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};



//entrypoint
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],

) -> ProgramResult {
    Ok(())
}

//call entrypoint
entrypoint!(process_instruction);


fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if instruction_data.len() == 0 {
        return Err(ProgramError::InvalidInstructionData);
    }

    if instruction_data[0] == 0 {
        return create_campaign(
            program_id,
            accounts,
            &instruction_data[1..instruction_data.len()],
        );
    } else if instruction_data[0] == 1 {
        return withdraw(
            program_id,
            accounts,
            &instruction_data[1..instruction_data.len()],
        );
    } else if instruction_data[0] == 2 {
        return donate(
            program_id,
            accounts,
            &instruction_data[1..instruction_data.len()],
        );
    }
    msg!("Didn't find the entrypoint required");
    Err(ProgramError::InvalidInstructionData)
}

entrypoint!(process_instruction);

#[derive(BorchSerialize, BorchDeserialize, Debug)]
struct CampaignDetails {
    pub admin: Pubkey,
    pub name: String,
    pub description: String,
    pub image_link: String,
    pub amount_donated: u64,
}

fn create_campaign(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    //iterator on accounts
    let accounts_iter = &mut accounts.iter();
    let writting_account = next_account_info(accounts_iter)?;
    let creator_account = next_account_info(accounts_iter)?;

    //now to allow transactoions we want the creator to sign the transaction
    if !creator_account.is_signer {
        msg!("creator_account should be signer");
        return Err(ProgramError::IncorrectProgramId);

    }

    if writting_account.owner != program_id {
        msg!("Writting_account isnt owned by program");
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut input_data = CampaignDetails::try_from_slice(&instruction_data)
    .expect("Instruction data serialization didn't work");

    if input_data.admin != *creator_account.key {
        msg!("invalid instruction data");
        return Err(ProgramError::InvalidInstructionData);
    }

    let rent_exemption = Rent::get()?.minimum_balance(writting_account.data_len());

    if **writting_account.lamports.borrow() < rent_exemption {
        msg!("The balance of writing_account should be more than rent_exemption");
        return Err(ProgramError::InsufficientFunds);
    }
    input_data.amount_donated=0;
    input_data.serialize(&mut &mut writing_account.data.borrow_mut()[..])?;
    Ok(())
}

fn withdraw(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    Ok(())
}

fn donate(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8]
) -> ProgramResult {
    Ok(())
}


#[derive(Debug)]
struct Human {
    pub eyes_color : String,
    pub name : String,
    pub height : i32,
}

#[derive(BorchSerialize, BorchDeserialize, Debug)]
struct CampaignDetails {
    pub admin: Pubkey,
    pub name: String,
    pub description: String, 
    pub image_link: String,
    pub amount_donated : u64,
}
