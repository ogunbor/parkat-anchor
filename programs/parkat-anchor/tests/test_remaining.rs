#[cfg(test)]
use mollusk_svm::{ program, result::Check, Mollusk };
use solana_sdk::{
    pubkey::Pubkey,
    signature::{ Keypair, Signer },
    instruction::{ AccountMeta, Instruction },
    account::{ Account, WritableAccount },
    native_token::LAMPORTS_PER_SOL,
    rent::Rent,
    sysvar::Sysvar,
};
use anchor_lang::InstructionData;

#[test]
fn test_deposit_by_user() {
    let program_id = Pubkey::new_from_array(
        five8_const::decode_32_const("CJbYiHnNrzYe7imm54hYA9HiJS1Q8BJs5okxFJbhuUx3")
    );

    let mollusk = Mollusk::new(&program_id, "../../target/deploy/parkat_anchor");

    let user = Keypair::new();
    let tenant_admin = Keypair::new();

    // Derive tenant state PDA
    let (tenant_pda, tenant_bump) = Pubkey::find_program_address(
        &[b"tenant", tenant_admin.pubkey().as_ref()],
        &program_id
    );

    // Derive User PDA 
    let (user_pda, user_bump) = Pubkey::find_program_address(
        &[b"user", tenant_pda.as_ref(), user.pubkey().as_ref()],
        &program_id
    );

    // Derive Vault PDA
    let (vault_pda, vault_bump) = Pubkey::find_program_address(
        &[b"vault", tenant_pda.as_ref(), user.pubkey().as_ref()],
        &program_id
    );

    // System program account
    let (system_program, system_account) = program::keyed_account_for_system_program();

    // Initialize Accounts
    let user_account = Account::new(10 * LAMPORTS_PER_SOL, 0, &system_program);
    let tenant_admin_account = Account::new(1 * LAMPORTS_PER_SOL, 0, &system_program);

    // Create tenant account
    use anchor_lang::Discriminator;
    use parkat_anchor::state::{Tenant, User};
    
    let mut tenant_data = vec![0u8; 8 + 73];
    tenant_data[0..8].copy_from_slice(&Tenant::DISCRIMINATOR);
    tenant_data[8..40].copy_from_slice(tenant_admin.pubkey().as_ref());  // admin
    tenant_data[80] = tenant_bump;  // bump at position 8 + 32 + 32 + 8 = 80
    
    let tenant_account = Account {
        lamports: mollusk.sysvars.rent.minimum_balance(tenant_data.len()),
        data: tenant_data,
        owner: program_id,
        executable: false,
        rent_epoch: 0,
    };

    // Create User account
    // User space: user(32) + tenant(32) + time_stamp(8) + is_parked(1) + amount(8) + vault_bump(1) + state_bump(1) + number_plate(16) = 99 bytes
    let mut user_data = vec![0u8; 8 + 99];
    user_data[0..8].copy_from_slice(&User::DISCRIMINATOR);
    user_data[8..40].copy_from_slice(user.pubkey().as_ref());      // user pubkey
    user_data[40..72].copy_from_slice(tenant_pda.as_ref());        // tenant pubkey
    // time_stamp = 0 (already zeros at positions 72-79)
    // is_parked = false (already 0 at position 80)
    // amount = 0 (already zeros at positions 81-88)
    user_data[89] = vault_bump;   // vault_bump at position 8 + 32 + 32 + 8 + 1 + 8 = 89
    user_data[90] = user_bump;    // state_bump at position 90
    // number_plate is zeros (already zeros at positions 91-106)
    
    let user_pda_account = Account {
        lamports: mollusk.sysvars.rent.minimum_balance(user_data.len()),
        data: user_data,
        owner: program_id,
        executable: false,
        rent_epoch: 0,
    };

    // Vault account (starts with 0 lamports, will receive deposit)
    let vault_account = Account::new(0, 0, &system_program);

    // Build instruction accounts (order must match Deposit struct)
    let instruction_accounts = vec![
        AccountMeta::new(user.pubkey(), true),              
        AccountMeta::new(tenant_pda, false),                
        AccountMeta::new_readonly(tenant_admin.pubkey(), false), 
        AccountMeta::new(vault_pda, false),                 
        AccountMeta::new(user_pda, false),                  
        AccountMeta::new_readonly(system_program, false),   
    ];

    // Create instruction data
    let deposit_amount = 1 * LAMPORTS_PER_SOL;
    let data = parkat_anchor::instruction::Deposit {
        amount: deposit_amount,
    }.data();

    // Create the instruction
    let instruction = Instruction::new_with_bytes(program_id, &data, instruction_accounts);

    let tx_accounts = vec![
        (user.pubkey(), user_account),
        (tenant_pda, tenant_account),
        (tenant_admin.pubkey(), tenant_admin_account),
        (vault_pda, vault_account),
        (user_pda, user_pda_account),
        (system_program, system_account),
    ];

    // Process with user as signer
    let _result = mollusk.process_and_validate_instruction(
        &instruction,
        &tx_accounts,
        &[Check::success()],
    );
}

#[test]
fn test_record_parking_start() {
    let program_id = Pubkey::new_from_array(
        five8_const::decode_32_const("CJbYiHnNrzYe7imm54hYA9HiJS1Q8BJs5okxFJbhuUx3")
    );

    let mollusk = Mollusk::new(&program_id, "../../target/deploy/parkat_anchor");

    let user = Keypair::new();
    let tenant_admin = Keypair::new();

    // Derive tenant state PDA
    let (tenant_pda, tenant_bump) = Pubkey::find_program_address(
        &[b"tenant", tenant_admin.pubkey().as_ref()],
        &program_id
    );

    // Derive User PDA
    let (user_pda, user_bump) = Pubkey::find_program_address(
        &[b"user", tenant_pda.as_ref(), user.pubkey().as_ref()],
        &program_id
    );

    // Derive Vault PDA (not used in this instruction but needed for User account data)
    let (vault_pda, vault_bump) = Pubkey::find_program_address(
        &[b"vault", tenant_pda.as_ref(), user.pubkey().as_ref()],
        &program_id
    );

    // System program account
    let (system_program, system_account) = program::keyed_account_for_system_program();

    // Initialize Accounts
    let user_account = Account::new(1 * LAMPORTS_PER_SOL, 0, &system_program);
    let tenant_admin_account = Account::new(1 * LAMPORTS_PER_SOL, 0, &system_program);

    // Create tenant account with proper discriminator
    use anchor_lang::Discriminator;
    use parkat_anchor::state::{Tenant, User};
    
    let mut tenant_data = vec![0u8; 8 + 73];
    tenant_data[0..8].copy_from_slice(&Tenant::DISCRIMINATOR);
    tenant_data[8..40].copy_from_slice(tenant_admin.pubkey().as_ref());  // admin
    tenant_data[80] = tenant_bump;  // bump
    
    let tenant_account = Account {
        lamports: mollusk.sysvars.rent.minimum_balance(tenant_data.len()),
        data: tenant_data,
        owner: program_id,
        executable: false,
        rent_epoch: 0,
    };

    // Create User account with proper discriminator
    // User is NOT parked initially (is_parked = false)
    let mut user_data = vec![0u8; 8 + 99];
    user_data[0..8].copy_from_slice(&User::DISCRIMINATOR);
    user_data[8..40].copy_from_slice(user.pubkey().as_ref());      
    user_data[40..72].copy_from_slice(tenant_pda.as_ref());        
    // time_stamp = 0 (positions 72-79)
    user_data[80] = 0;  // is_parked = false
    // amount = 0 (positions 81-88)
    user_data[89] = vault_bump;   
    user_data[90] = user_bump;    
    // number_plate (positions 91-106)
    
    let user_pda_account = Account {
        lamports: mollusk.sysvars.rent.minimum_balance(user_data.len()),
        data: user_data,
        owner: program_id,
        executable: false,
        rent_epoch: 0,
    };

    // Build instruction accounts 
    let instruction_accounts = vec![
        AccountMeta::new(user.pubkey(), true),                     
        AccountMeta::new(tenant_pda, false),                        
        AccountMeta::new_readonly(tenant_admin.pubkey(), false),    
        AccountMeta::new(user_pda, false),                          
    ];

    // Create instruction data 
    let data = parkat_anchor::instruction::RecordParkingStart {}.data();

    // Create the instruction
    let instruction = Instruction::new_with_bytes(program_id, &data, instruction_accounts);

    let tx_accounts = vec![
        (user.pubkey(), user_account),
        (tenant_pda, tenant_account),
        (tenant_admin.pubkey(), tenant_admin_account),
        (user_pda, user_pda_account),
        (system_program, system_account),
    ];

    // Process with user as signer
    let _result = mollusk.process_and_validate_instruction(
        &instruction,
        &tx_accounts,
        &[Check::success()],
    );
}

#[test]
fn test_process_exit() {
    let program_id = Pubkey::new_from_array(
        five8_const::decode_32_const("CJbYiHnNrzYe7imm54hYA9HiJS1Q8BJs5okxFJbhuUx3")
    );

    let mollusk = Mollusk::new(&program_id, "../../target/deploy/parkat_anchor");

    let user = Keypair::new();
    let tenant_admin = Keypair::new();
    let admin_wallet = Keypair::new();

    let (tenant_pda, tenant_bump) = Pubkey::find_program_address(
        &[b"tenant", tenant_admin.pubkey().as_ref()],
        &program_id
    );

    let (user_pda, user_bump) = Pubkey::find_program_address(
        &[b"user", tenant_pda.as_ref(), user.pubkey().as_ref()],
        &program_id
    );

    let (vault_pda, vault_bump) = Pubkey::find_program_address(
        &[b"vault", tenant_pda.as_ref(), user.pubkey().as_ref()],
        &program_id
    );

    let (system_program, system_account) = program::keyed_account_for_system_program();

    // Initialize Accounts
    let user_account = Account::new(1 * LAMPORTS_PER_SOL, 0, &system_program);
    let tenant_admin_account = Account::new(1 * LAMPORTS_PER_SOL, 0, &system_program);
    let admin_wallet_account = Account::new(0, 0, &system_program);

    // Create tenant account
    use anchor_lang::Discriminator;
    use parkat_anchor::state::{Tenant, User};
    
    let mut tenant_data = vec![0u8; 8 + 73];
    tenant_data[0..8].copy_from_slice(&Tenant::DISCRIMINATOR);
    tenant_data[8..40].copy_from_slice(tenant_admin.pubkey().as_ref());
    tenant_data[80] = tenant_bump;
    
    let tenant_account = Account {
        lamports: mollusk.sysvars.rent.minimum_balance(tenant_data.len()),
        data: tenant_data,
        owner: program_id,
        executable: false,
        rent_epoch: 0,
    };

    // Create User account - user is parked with a timestamp
    let parking_start_time: i64 = 0; // Some past timestamp
    let mut user_data = vec![0u8; 8 + 99];
    user_data[0..8].copy_from_slice(&User::DISCRIMINATOR);
    user_data[8..40].copy_from_slice(user.pubkey().as_ref());      
    user_data[40..72].copy_from_slice(tenant_pda.as_ref());        
    // time_stamp (positions 72-79) - set parking start time
    user_data[72..80].copy_from_slice(&parking_start_time.to_le_bytes());
    user_data[80] = 1;  // is_parked = true
    // amount (positions 81-88) - set to vault balance
    let vault_balance: u64 = 2 * LAMPORTS_PER_SOL;
    user_data[81..89].copy_from_slice(&vault_balance.to_le_bytes());
    user_data[89] = vault_bump;   
    user_data[90] = user_bump;    
    
    let user_pda_account = Account {
        lamports: mollusk.sysvars.rent.minimum_balance(user_data.len()),
        data: user_data,
        owner: program_id,
        executable: false,
        rent_epoch: 0,
    };

    // Vault account with balance (user has deposited funds)
    let vault_account = Account::new(vault_balance, 0, &system_program);

    // Build instruction accounts 
    let instruction_accounts = vec![
        AccountMeta::new(user.pubkey(), true),                      
        AccountMeta::new(vault_pda, false),                         
        AccountMeta::new(user_pda, false),                          
        AccountMeta::new(tenant_pda, false),                        
        AccountMeta::new_readonly(tenant_admin.pubkey(), false),    
        AccountMeta::new(admin_wallet.pubkey(), false),             
        AccountMeta::new_readonly(system_program, false),           
    ];

    // Create instruction data 
    let data = parkat_anchor::instruction::ProcessExit {}.data();

    // Create the instruction
    let instruction = Instruction::new_with_bytes(program_id, &data, instruction_accounts);

    let tx_accounts = vec![
        (user.pubkey(), user_account),
        (vault_pda, vault_account),
        (user_pda, user_pda_account),
        (tenant_pda, tenant_account),
        (tenant_admin.pubkey(), tenant_admin_account),
        (admin_wallet.pubkey(), admin_wallet_account),
        (system_program, system_account),
    ];

    let _result = mollusk.process_and_validate_instruction(
        &instruction,
        &tx_accounts,
        &[Check::success()],
    );
}