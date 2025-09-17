use anchor_lang::prelude::*;

declare_id!("CJbYiHnNrzYe7imm54hYA9HiJS1Q8BJs5okxFJbhuUx3");

#[program]
pub mod parkat_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
