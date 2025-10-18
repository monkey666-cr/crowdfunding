use anchor_lang::prelude::*;

declare_id!("BrpDKtbu9Z6dHZteU1sQc8644QDZCgjNKU1KUNiGhrQQ");

#[program]
pub mod crowdfunding2 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
