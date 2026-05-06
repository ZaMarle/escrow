use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    example_mocks::solana_sdk::system_instruction,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    match data[0] {
        0 => place_order(program_id, accounts, &data[1..]),
        // 1 => cancel_order(program_id, accounts, &data[1..]),
        // 2 => fill_order(program_id, accounts, &data[1..]),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

pub fn place_order(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    // 1. Parse accounts from
    let mut account_info_iter = accounts.iter();
    let owner_account = account_info_iter.next().unwrap();
    let order_account = account_info_iter.next().unwrap();
    let system_program_account = account_info_iter.next().unwrap();
    let vault_account = account_info_iter.next().unwrap();
    let mint_account = account_info_iter.next().unwrap();
    let token_program_account = account_info_iter.next().unwrap();
    let owner_token_account = account_info_iter.next().unwrap();

    // 1. Parse instruction data — side, price, amount, nonce (for PDA uniqueness)
    let side = data[0];
    let price = u64::from_le_bytes(data[1..9].try_into().unwrap());
    let amount = u64::from_le_bytes(data[9..17].try_into().unwrap());
    let nonce = u64::from_le_bytes(data[17..25].try_into().unwrap());
    let order_bump = data[25];
    let vault_bump = data[26];

    // 2. Verify the owner signed
    if !owner_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // 3. Derive and verify the order PDA matches the passed account
    let order_seeds = &[
        b"order",
        owner_account.key.as_ref(),
        &nonce.to_le_bytes(),
        &[order_bump],
    ];
    let expected_order_key = Pubkey::create_program_address(order_seeds, program_id)
        .map_err(|_| ProgramError::InvalidArgument)?;

    if order_account.key != &expected_order_key {
        return Err(ProgramError::InvalidArgument);
    }

    // 4. Create the order account on-chain (via system program)
    let order_size = 57;
    let rent = Rent::get().unwrap();
    let lamports = rent.minimum_balance(order_size);

    let create_ix = system_instruction::create_account(
        owner_account.key, // funding account (pays rent)
        order_account.key, // new pda account to create
        lamports,
        order_size as u64,
        program_id,
    );

    invoke_signed(
        &create_ix,
        &[
            owner_account.clone(),
            order_account.clone(),
            system_program_account.clone(),
        ],
        &[&[
            b"order",
            owner_account.key.as_ref(),
            &nonce.to_le_bytes(),
            &[order_bump],
        ]],
    )
    .unwrap();

    // 5. Derive and verify the vault PDA (token account that holds escrowed funds)
    let vault_seeds = &[b"vault", order_account.key.as_ref(), &[vault_bump]];
    let expected_vault_key = Pubkey::create_program_address(vault_seeds, program_id).unwrap();
    if vault_account.key != &expected_vault_key {
        return Err(ProgramError::InvalidArgument);
    }

    // 6. Create + initialize the vault token account
    // a. allocate account, owned by the token program
    let vault_size = spl_token::state::Account::LEN;
    let vault_lamports = rent.minimum_balance(vault_size);

    let create_vault_ix = system_instruction::create_account(
        owner_account.key,
        vault_account.key,
        vault_lamports,
        vault_size as u64,
        token_program_account.key,
    );

    invoke_signed(
        &create_vault_ix,
        &[
            owner_account.clone(),
            vault_account.clone(),
            system_program_account.clone(),
        ],
        &[&[b"vault", order_account.key.as_ref(), &[vault_bump]]],
    );

    // b. Initialize it as a token account; order PDA is the authority so only the program can move funds
    let init_vault_ix = spl_token::instruction::initialize_account3(
        token_program_account.key,
        vault_account.key,
        mint_account.key,
        order_account.key,
    )
    .unwrap();

    invoke(
        &init_vault_ix,
        &[
            vault_account.clone(),
            mint_account.clone(),
            token_program_account.clone(),
        ],
    );

    // 7. Transfer tokens from owner into the vault
    let transfer_ix = spl_token::instruction::transfer(
        token_program_account.key,
        owner_token_account.key,
        vault_account.key,
        owner_account.key,
        &[],
        amount,
    )
    .unwrap();

    invoke(
        &transfer_ix,
        &[
            owner_token_account.clone(),
            vault_account.clone(),
            owner_account.clone(),
            token_program_account.clone(),
        ],
    );

    // 8. Write the order data into the order account
    let mut data = order_account.data.borrow_mut();
    data[0] = side;
    data[1..9].copy_from_slice(&price.to_le_bytes());
    data[9..17].copy_from_slice(&amount.to_le_bytes());
    data[17..25].copy_from_slice(&nonce.to_le_bytes());
    data[25..57].copy_from_slice(owner_account.key.as_ref());

    Ok(())
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
