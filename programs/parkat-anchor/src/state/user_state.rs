use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct User {
    pub user: Pubkey,
    pub tenant: Pubkey,
    pub time_stamp: i64,
    pub is_parked: bool,
    pub amount: u64,
    pub vault_bump: u8,
    pub state_bump: u8,
    pub number_plate: [u8; 16],
}
