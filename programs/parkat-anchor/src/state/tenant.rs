use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Tenant {
    pub admin: Pubkey,
    pub name: [u8; 32],
    pub created_at: i64,
    pub bump: u8,
}
