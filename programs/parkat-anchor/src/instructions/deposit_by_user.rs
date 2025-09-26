use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::state::User;

#[derive(Accounts)]
pub struct Deposit<'info> {
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

    pub system_program: Program<'info, System>,
}

impl<'info> Deposit<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
       
        if amount == 0 {
            return Err(error!(Error::InvalidDepositAmount));
        }

        // Check for overflow before performing operations
        let new_amount = self.car.amount
            .checked_add(amount)
            .ok_or(error!(Error::ArithmeticOverflow))?;

        // Perform the transfer
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.user.to_account_info(),
            to: self.vault.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, amount)?;

        // Update balance field after successful transfer
        self.car.amount = new_amount;

        // Verify balance consistency with proper error handling
        let vault_balance = self.vault.lamports();
        if vault_balance != self.car.amount {
            return Err(error!(Error::BalanceMismatch));
        }

        Ok(())
    }
}

#[error_code]
pub enum Error {
    #[msg("Deposit amount must be greater than zero")]
    InvalidDepositAmount,

    #[msg("Deposit amount is too large")]
    DepositTooLarge,

    #[msg("Arithmetic overflow occurred")]
    ArithmeticOverflow,

    #[msg("Vault balance does not match recorded amount")]
    BalanceMismatch,
}