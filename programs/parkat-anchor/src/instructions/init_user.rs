use anchor_lang::prelude::*;

use crate::state::User;

#[derive(Accounts)]
pub struct InitUser<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        space = 8 + User::INIT_SPACE,
        seeds = [b"user", user.key().as_ref()],
        bump,
    )]
    pub car: Account<'info, User>,
    #[account(
        seeds = [b"vault", car.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>
    pub system_program: Program<'info, System>,
}

impl<'info> InitUser<'info> {
    pub fn init_user(&mut self, bumps: &InitUserBumps) -> Result<()> {
        let car = &mut self.car;

        car.user = self.user.key();
        car.time_stamp = Clock::get()?.unix_timestamp;
        car.is_parked = false;
        car.amount = 0;
        car.vault_bump = bumps.vault
        car.state_bump = bumps.car;

        Ok(())
    }
}