use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::state::{User, Tenant};

#[derive(Accounts)]
pub struct ProcessExit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault", tenant.key().as_ref(), user.key().as_ref()],
        bump = user_account.vault_bump,
    )]
    pub vault: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [b"user", tenant.key().as_ref(), user.key().as_ref()],
        bump = user_account.state_bump,
    )]
    pub user_account: Account<'info, User>,

    #[account(
        mut,
        seeds = [b"tenant", tenant_admin.key().as_ref()],
        bump = tenant.bump,
    )]
    pub tenant: Account<'info, Tenant>,

    /// CHECK: Tenant admin - must match the admin used during tenant initialization
    pub tenant_admin: UncheckedAccount<'info>,

    /// CHECK: Admin wallet only used for transfer
    pub admin_wallet: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> ProcessExit<'info> {
    pub fn process_exit(&mut self) -> Result<()> {
        let user_account = &mut self.user_account;

        // Ensure the user is parked
        if !user_account.is_parked {
            return Err(error!(ProcessExitError::NotCurrentlyParked));
        }

        // Get current blockchain time
        let current_time = Clock::get()?.unix_timestamp;

        // Calculate duration parked
        let duration = current_time
            .checked_sub(user_account.time_stamp)
            .ok_or_else(|| error!(ProcessExitError::InvalidParkingDuration))?;

        let duration_u64 = u64::try_from(duration)
            .map_err(|_| error!(ProcessExitError::InvalidParkingDuration))?;

        // Calculate amount to transfer (parking fee)
        // Fee: 100 lamports per minute
        let rate_per_minute: u64 = 100;
        let duration_minutes = duration_u64 / 60;
        let amount = duration_minutes
            .checked_mul(rate_per_minute)
            .ok_or_else(|| error!(ProcessExitError::AmountCalculationError))?;

        // Only transfer if amount > 0
        if amount > 0 {
            // Ensure vault has enough balance
            let vault_balance = self.vault.to_account_info().lamports();
            if amount > vault_balance {
                return Err(error!(ProcessExitError::InsufficientVaultBalance));
            }

            // Prepare signer seeds for vault PDA
            let tenant_key = self.tenant.key();
            let user_key = self.user.key();

            let seeds_slice: &[&[u8]] = &[
                b"vault",
                tenant_key.as_ref(),
                user_key.as_ref(),
                &[user_account.vault_bump],
            ];
            let signer_seeds: &[&[&[u8]]] = &[seeds_slice];

            // Perform CPI transfer (parking fee to admin)
            let cpi_program = self.system_program.to_account_info();
            let cpi_accounts = Transfer {
                from: self.vault.to_account_info(),
                to: self.admin_wallet.to_account_info(),
            };
            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

            transfer(cpi_ctx, amount)?;

            // Update user account amount to reflect current vault balance after transfer
            user_account.amount = self.vault.lamports();
        }

        // Update parking state
        user_account.time_stamp = Clock::get()?.unix_timestamp;
        user_account.is_parked = false;

        Ok(())
    }
}

#[error_code]
pub enum ProcessExitError {
    #[msg("Time calculation failed")]
    TimeCalculationError,

    #[msg("Invalid parking duration - current time is before parking start time")]
    InvalidParkingDuration,

    #[msg("User is not currently parked")]
    NotCurrentlyParked,

    #[msg("Amount calculation failed - insufficient balance for parking fee")]
    AmountCalculationError,

    #[msg("Vault does not have enough balance")]
    InsufficientVaultBalance,
}