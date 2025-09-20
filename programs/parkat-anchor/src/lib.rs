use anchor_lang::prelude::*;
mod state;
mod instructions;
use instructions::*;

declare_id!("CJbYiHnNrzYe7imm54hYA9HiJS1Q8BJs5okxFJbhuUx3");

#[program]
pub mod parkat_anchor {
    use super::*;

    pub fn init_user(ctx: Context<InitUser>) -> Result<()> {
        ctx.accounts.init_user(&ctx.bumps)
    }
}
