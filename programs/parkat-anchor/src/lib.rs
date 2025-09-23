use anchor_lang::prelude::*;
mod instructions;
mod state;
use instructions::*;

declare_id!("CJbYiHnNrzYe7imm54hYA9HiJS1Q8BJs5okxFJbhuUx3");

#[program]
pub mod parkat_anchor {
    use super::*;

    pub fn init_user(ctx: Context<InitUser>) -> Result<()> {
        ctx.accounts.init_user(&ctx.bumps)
    }

    pub fn deposit_by_user(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)
    }

    pub fn record_parking_start(ctx: Context<RecordParkingStart>) -> Result<()> {
        ctx.accounts.record_parking_start()
    }

    pub fn process_exit(ctx: Context<ProcessExit>) -> Result<()> {
        ctx.accounts.process_exit()
    }
}
