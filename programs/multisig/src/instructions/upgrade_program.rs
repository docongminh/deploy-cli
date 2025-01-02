use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UpgradeProgram {}

impl UpgradeProgram {
    pub fn handler(ctx: Context<Self>) -> Result<()> {
        Ok(())
    }
}
