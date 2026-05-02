use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
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
    // 1. Parse instruction data — side, price, amount, nonce (for PDA uniqueness)
    let side = data[0];
    let price = u64::from_le_bytes(data[1..9].try_into().unwrap());
    let amount = u64::from_le_bytes(data[9..17].try_into().unwrap());
    let nonce = u64::from_le_bytes(data[17..25].try_into().unwrap());
    let order_bump = data[25];
    let vault_bump = data[26];

    // 2. Verify the owner signed

    // 3. Derive and verify the order PDA matches the passed account

    // 4. Create the order account on-chain (via system program)

    // 5. Derive and verify the vault PDA (token account that holds escrowed funds)

    // 6. Create + initialize the vault token account

    // 7. Transfer tokens from owner into the vault

    // 8. Write the order data into the order account
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
