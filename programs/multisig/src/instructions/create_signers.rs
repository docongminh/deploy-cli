use anchor_lang::prelude::*;

use crate::{SignerConfig, CONFIG_SEED};

#[derive(Accounts)]
#[instruction(signers: Vec<Pubkey>)]
pub struct CreateSigners<'info> {
    #[account(init, payer = creator,space = SignerConfig::space(signers.len()), seeds = [CONFIG_SEED, creator.key().as_ref()], bump)]
    pub signer_config: Account<'info, SignerConfig>,

    #[account(mut)]
    creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl CreateSigners<'_> {
    pub fn handler(ctx: Context<Self>, params: CreateSignersParams) -> Result<()> {
        let signer_config = &mut ctx.accounts.signer_config;
        signer_config.creator = ctx.accounts.creator.key();
        signer_config.master_authority = params.master_authority;
        signer_config.signers_required = params.signers_required;
        signer_config.signers = params.signers;
        signer_config.bump = ctx.bumps.signer_config;

        Ok(())
    }
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct CreateSignersParams {
    pub master_authority: Option<Pubkey>,
    pub signers_required: u16,
    pub signers: Vec<Pubkey>,
}
