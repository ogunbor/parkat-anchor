use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

use crate::state::User;

#[derive(Accounts)]
pub struct ProcessExit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault", car.key().as_ref()],
        bump = car.vault_bump,
    )]
    pub vault: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [b"user", user.key().as_ref()],
        bump = car.state_bump,
    )]
    pub car: Account<'info, User>,

    /// CHECK: Admin's wallet is used only for transfer purposes and does not require validation.
    pub admin_wallet: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> ProcessExit<'info> {
    pub fn process_exit(&mut self) -> Result<()> {
        let car = &mut self.car;

        // Check if user is currently parked
        if !car.is_parked {
            return Err(error!(Error::NotCurrentlyParked));
        }

        // Get current blockchain time
        let current_time = Clock::get()?.unix_timestamp;

        // Calculate duration parked
        let duration = current_time
            .checked_sub(car.time_stamp)
            .ok_or_else(|| error!(Error::InvalidParkingDuration))?;

        // Convert to u64
        let duration_u64 = u64::try_from(duration)
            .map_err(|_| error!(Error::InvalidParkingDuration))?;

        let amount = duration_u64 / 1000;

        // Check if we need to transfer (only if amount > 0)
        if amount > 0 {
            // Check vault balance
            let vault_balance = self.vault.to_account_info().lamports();
            if amount > vault_balance {
                return Err(error!(Error::InsufficientVaultBalance));
            }

            // Perform CPI transfer
            let cpi_program = self.system_program.to_account_info();
            let cpi_accounts = Transfer {
                from: self.vault.to_account_info(),
                to: self.admin_wallet.to_account_info(),
            };
            let seeds = &[
                b"vault",
                car.to_account_info().key.as_ref(),
                &[car.vault_bump],
            ];
            let signer_seeds = &[&seeds[..]];
            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

            transfer(cpi_ctx, amount)?;

            // Update car amount
            car.amount = car
                .amount
                .checked_sub(amount)
                .ok_or_else(|| error!(Error::AmountCalculationError))?;
        }

        // Update parking state
        car.time_stamp = current_time;
        car.is_parked = false;

        Ok(())
    }
}

#[error_code]
pub enum Error {
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