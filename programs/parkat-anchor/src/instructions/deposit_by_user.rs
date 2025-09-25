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
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.user.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, amount)?;

        // update balance field
        self.car.amount = self.car.amount.checked_add(amount).unwrap();

        // assert balance
        assert_eq!(
            self.vault.lamports(),
            self.car.amount,
            "Vault lamports do not match recorded deposits"
        );

        Ok(())
    }
}