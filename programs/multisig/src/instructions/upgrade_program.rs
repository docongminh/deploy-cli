use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UpgradeProgram {}

impl UpgradeProgram {
    pub fn handler(_ctx: Context<Self>) -> Result<()> {
        Ok(())
    }
}
