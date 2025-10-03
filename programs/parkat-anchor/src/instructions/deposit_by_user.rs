use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::state::{Tenant, User};

#[derive(Accounts)]
pub struct Deposit<'info> {
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
        seeds = [b"vault", tenant.key().as_ref(), user.key().as_ref()],
        bump = car.vault_bump,
    )]
    pub vault: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [b"user", tenant.key().as_ref(), user.key().as_ref()],
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

        // Perform transfer first
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.user.to_account_info(),
            to: self.vault.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        transfer(cpi_ctx, amount)?;

        // Update tracked amount to reflect current vault balance
        self.car.amount = self.vault.lamports();

        Ok(())
    }
}

#[error_code]
pub enum Error {
    #[msg("Deposit amount must be greater than zero")]
    InvalidDepositAmount,
}