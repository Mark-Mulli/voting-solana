use anchor_lang::prelude::*;

declare_id!("GYDa6fTek7nz1T5atPzgpHpRHMFhZaWryHfUvoRi1bne");

#[program]
pub mod voting {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
