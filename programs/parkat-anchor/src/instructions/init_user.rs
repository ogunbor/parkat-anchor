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
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitUser<'info> {
    pub fn init_user(&mut self, bumps: &InitUserBumps, number_plate: String) -> Result<()> {
        let car = &mut self.car;

        car.user = self.user.key();
        car.time_stamp = Clock::get()?.unix_timestamp;
        car.is_parked = false;
        car.amount = 0;
        car.vault_bump = bumps.vault;
        car.state_bump = bumps.car;

        // number plate of car (string) conversion to bytes
        let mut plate_bytes = [0u8; 16];
        let input_bytes = number_plate.as_bytes();
        let len = input_bytes.len().min(16);
        plate_bytes[..len].copy_from_slice(&input_bytes[..len]);
        car.number_plate = plate_bytes;

        Ok(())
    }
}
