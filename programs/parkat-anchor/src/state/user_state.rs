use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct User {
    pub user: Pubkey,
    pub latest_update: i64,
    pub is_parked: bool,
    pub bump: u8,
}