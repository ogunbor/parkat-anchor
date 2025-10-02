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