use anchor_lang::prelude::*;

pub mod error;
pub mod instructions;
pub mod state;

pub use error::*;
pub use instructions::*;
pub use state::*;

declare_id!("344XyJb77NkmMw8vi4Rcqx2mfFPrJAAvEvqef39J8osj");

#[constant]
pub const CONFIG_SEED: &[u8] = b"config_seed";

#[program]
pub mod signers {
    use super::*;

    pub fn create_signers(ctx: Context<CreateSigners>, params: CreateSignersParams) -> Result<()> {
        CreateSigners::create_signers_config_handler(ctx, params)
    }

    pub fn add_signer(ctx: Context<UpdateSignerConfig>, params: AddSignerParams) -> Result<()> {
        UpdateSignerConfig::add_signer_handler(ctx, params)
    }

    pub fn remove_signer(
        ctx: Context<UpdateSignerConfig>,
        params: RemoveSignerParams,
    ) -> Result<()> {
        UpdateSignerConfig::remove_signer_handler(ctx, params)
    }

    pub fn update_signers_required(
        ctx: Context<UpdateSignerConfig>,
        params: UpdateSignersRequiredParams,
    ) -> Result<()> {
        UpdateSignerConfig::update_signers_required_handler(ctx, params)
    }

    pub fn upgrade_program(ctx: Context<UpgradeProgram>) -> Result<()> {
        UpgradeProgram::handler(ctx)
    }
}
