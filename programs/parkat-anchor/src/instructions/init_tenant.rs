use anchor_lang::prelude::*;

use crate::state::Tenant;

#[derive(Accounts)]
pub struct InitTenant<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = 8 + Tenant::INIT_SPACE,
        seeds = [b"tenant", admin.key().as_ref()],
        bump
    )]
    pub tenant: Account<'info, Tenant>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitTenant<'info> {
    pub fn init_tenant(&mut self, bumps: &InitTenantBumps, tenant_name: String) -> Result<()> {
        // Validate tenant name
        if tenant_name.is_empty() {
            return Err(error!(InitTenantError::EmptyTenantName));
        }

        let tenant = &mut self.tenant;

        tenant.admin = self.admin.key();
        tenant.created_at = Clock::get()?.unix_timestamp;
        tenant.bump = bumps.tenant;

        // Convert tenant_name(String) to bytes with length validation
        let mut name_bytes = [0u8; 32];
        let input_bytes = tenant_name.as_bytes();
        let len = input_bytes.len().min(32);
        name_bytes[..len].copy_from_slice(&input_bytes[..len]);
        tenant.name = name_bytes;

        Ok(())
    }
}

#[error_code]
pub enum InitTenantError {
    #[msg("Tenant name cannot be empty")]
    EmptyTenantName,
}