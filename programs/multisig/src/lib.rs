use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

pub use instructions::*;
pub use state::*;
declare_id!("344XyJb77NkmMw8vi4Rcqx2mfFPrJAAvEvqef39J8osj");

#[constant]
pub const CONFIG_SEED: &[u8] = b"config_seed";

#[program]
pub mod signers {
    use super::*;

    pub fn create_signers(ctx: Context<CreateSigners>, params: CreateSignersParams) -> Result<()> {
        CreateSigners::handler(ctx, params)
    }

    pub fn upgrade_program(ctx: Context<UpgradeProgram>) -> Result<()> {
        UpgradeProgram::handler(ctx)
    }
}
