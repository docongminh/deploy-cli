use anchor_lang::prelude::*;

declare_id!("344XyJb77NkmMw8vi4Rcqx2mfFPrJAAvEvqef39J8osj");

#[program]
pub mod signers {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
