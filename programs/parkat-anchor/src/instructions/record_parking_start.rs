use anchor_lang::prelude::*;

use crate::state::User;

#[derive(Accounts)]
pub struct RecordParkingStart<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"user", user.key().as_ref()],
        bump = car.state_bump
    )]
    pub car: Account<'info, User>,
}

impl<'info> RecordParkingStart<'info> {
    pub fn record_parking_start(&mut self) -> Result<()> {
        let car = &mut self.car;

        // Update fields
        car.time_stamp = Clock::get()?.unix_timestamp;
        car.is_parked = true;

        Ok(())
    }
}
