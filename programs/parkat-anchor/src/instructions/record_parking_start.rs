use anchor_lang::prelude::*;

use crate::state::{Tenant, User};

#[derive(Accounts)]
pub struct RecordParkingStart<'info> {
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
        mut,
        seeds = [b"user", tenant.key().as_ref(), user.key().as_ref()],
        bump = user_account.state_bump
    )]
    pub user_account: Account<'info, User>,
}

impl<'info> RecordParkingStart<'info> {
    pub fn record_parking_start(&mut self) -> Result<()> {
        let user_account = &mut self.user_account;

        // Check if user is already parked
        if user_account.is_parked {
            return Err(error!(Error::AlreadyParked));
        }

        // Update parking start time and status
        user_account.time_stamp = Clock::get()?.unix_timestamp;
        user_account.is_parked = true;

        Ok(())
    }
}

#[error_code]
pub enum Error {
    #[msg("User is already parked")]
    AlreadyParked,
}