use anchor_lang::prelude::*;

declare_id!("Bqvu7yb2NgXhThMrYapQiCgRAgtRVUd9VfQ2uq1JebLE");

#[program]
pub mod rockcoin {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
