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
fn test_init_tenant() {
    let program_id = Pubkey::new_from_array(
        five8_const::decode_32_const("CJbYiHnNrzYe7imm54hYA9HiJS1Q8BJs5okxFJbhuUx3")
    );

    // Initialize Mollusk
    let mollusk = Mollusk::new(&program_id, "../../target/deploy/parkat_anchor");

    // Keypair for admin
    let admin = Keypair::new();

    // Derive tenant state PDA
    let (tenant_pda, _tenant_bump) = Pubkey::find_program_address(
        &[b"tenant", admin.pubkey().as_ref()],
        &program_id
    );

    // System program account
    let (system_program, system_account) = program::keyed_account_for_system_program();

    // Build the accounts
    let admin_account = Account::new(1 * LAMPORTS_PER_SOL, 0, &system_program);
    
    let tenant_account = Account::new(0, 0, &system_program);

    // Get the accounts meta
    let instruction_accounts = vec![
        AccountMeta::new(admin.pubkey(), true),
        AccountMeta::new(tenant_pda, false),
        AccountMeta::new_readonly(system_program, false)
    ];
   
    // Get the anchor discriminator
    let data = parkat_anchor::instruction::InitTenant { 
        tenant_name: String::from("1") 
    }.data();

    // Create the instruction
    let instruction = Instruction::new_with_bytes(program_id, &data, instruction_accounts);

    let tx_accounts = vec![
        (admin.pubkey(), admin_account),
        (tenant_pda, tenant_account),
        (system_program, system_account)
    ];

    // Process with admin as signer
    let _init_result = mollusk.process_and_validate_instruction(
        &instruction,
        &tx_accounts,
        &[Check::success()],
    );
}

#[test]
fn test_init_user() {
    let program_id = Pubkey::new_from_array(
        five8_const::decode_32_const("CJbYiHnNrzYe7imm54hYA9HiJS1Q8BJs5okxFJbhuUx3")
    );

    let mollusk = Mollusk::new(&program_id, "../../target/deploy/parkat_anchor");

    let user = Keypair::new();
    let tenant_admin = Keypair::new();

    let (tenant_pda, tenant_bump) = Pubkey::find_program_address(
        &[b"tenant", tenant_admin.pubkey().as_ref()],
        &program_id
    );
    
    let (user_account_pda, _user_account_bump) = Pubkey::find_program_address(
        &[b"user", tenant_pda.as_ref(), user.pubkey().as_ref()],
        &program_id
    );
    
    let (vault_pda, _vault_bump) = Pubkey::find_program_address(
        &[b"vault", tenant_pda.as_ref(), user.pubkey().as_ref()],
        &program_id
    );

    let (system_program, system_account) = program::keyed_account_for_system_program();

    // Build the accounts
    let user_account = Account::new(1 * LAMPORTS_PER_SOL, 0, &system_program);
    let tenant_admin_account = Account::new(1 * LAMPORTS_PER_SOL, 0, &system_program);
    
    // Create tenant account with proper discriminator
    use anchor_lang::Discriminator;
    use parkat_anchor::state::Tenant;
    
    let mut tenant_data = vec![0u8; 8 + 73];
    
    // Set the discriminator (first 8 bytes)
    tenant_data[0..8].copy_from_slice(&Tenant::DISCRIMINATOR);
    
    // Serialize the tenant fields manually
    // Format: admin (32) + name (32) + created_at (8) + bump (1)
    tenant_data[8..40].copy_from_slice(tenant_admin.pubkey().as_ref());
    // name is [0u8; 32] - already zeros
    // created_at is 0 - already zeros
    tenant_data[80] = tenant_bump; // bump is at position 8 + 32 + 32 + 8 = 80
    
    let tenant_account = Account {
        lamports: mollusk.sysvars.rent.minimum_balance(tenant_data.len()),
        data: tenant_data,
        owner: program_id,
        executable: false,
        rent_epoch: 0,
    };
    
    let user_account_data = Account::new(0, 0, &system_program);
    let vault_account = Account::new(0, 0, &system_program);

    let instruction_accounts = vec![
        AccountMeta::new(user.pubkey(), true),
        AccountMeta::new(tenant_pda, false),
        AccountMeta::new_readonly(tenant_admin.pubkey(), false),
        AccountMeta::new(user_account_pda, false),
        AccountMeta::new(vault_pda, false),
        AccountMeta::new_readonly(system_program, false),
    ];

    let data = parkat_anchor::instruction::InitUser {
        number_plate: String::from("ABC123"),
    }.data();

    let instruction = Instruction::new_with_bytes(program_id, &data, instruction_accounts);

    let tx_accounts = vec![
        (user.pubkey(), user_account),
        (tenant_pda, tenant_account),
        (tenant_admin.pubkey(), tenant_admin_account),
        (user_account_pda, user_account_data),
        (vault_pda, vault_account),
        (system_program, system_account),
    ];

    let _init_result = mollusk.process_and_validate_instruction(
        &instruction,
        &tx_accounts,
        &[Check::success()],
    );
}