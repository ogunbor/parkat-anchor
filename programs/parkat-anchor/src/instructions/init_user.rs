use anchor_lang::prelude::*;
use crate::state::{Tenant, User};

#[derive(Accounts)]
pub struct InitUser<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"tenant", tenant_admin.key().as_ref()],
        bump = tenant.bump,
    )]
    pub tenant: Account<'info, Tenant>,

    /// CHECK: Tenant admin - must match the admin used during tenant initialization
    pub tenant_admin: UncheckedAccount<'info>,

    #[account(
        init,
        payer = user,
        space = 8 + User::INIT_SPACE,
        seeds = [b"user", tenant.key().as_ref(), user.key().as_ref()],
        bump,
    )]
    pub user_account: Account<'info, User>,

    #[account(
        seeds = [b"vault", tenant.key().as_ref(), user.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitUser<'info> {
    pub fn init_user(&mut self, bumps: &InitUserBumps, number_plate: String) -> Result<()> {
        if number_plate.is_empty() {
            return Err(error!(Error::EmptyNumberPlate));
        }

        let user_account = &mut self.user_account;
        user_account.user = self.user.key();
        user_account.tenant = self.tenant.key();
        user_account.time_stamp = Clock::get()?.unix_timestamp;
        user_account.is_parked = false;
        user_account.amount = 0;
        user_account.vault_bump = bumps.vault;
        user_account.state_bump = bumps.user_account;

        // Convert number plate string to fixed-length byte array
        let mut plate_bytes = [0u8; 16];
        let input_bytes = number_plate.as_bytes();
        let len = input_bytes.len().min(16);
        plate_bytes[..len].copy_from_slice(&input_bytes[..len]);
        user_account.number_plate = plate_bytes;

        Ok(())
    }
}

#[error_code]
pub enum Error {
    #[msg("Number plate cannot be empty")]
    EmptyNumberPlate,
}
