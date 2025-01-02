use anchor_lang::prelude::*;

use crate::{MultiSigError, SignerConfig, CONFIG_SEED};

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct AddSignerParams {
    pub new_signer: Pubkey,
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct RemoveSignerParams {
    pub signer: Pubkey,
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct UpdateSignersRequiredParams {
    pub new_signer_required: u16,
}

#[derive(Accounts)]
pub struct UpdateSignerConfig<'info> {
    #[account(mut, seeds = [CONFIG_SEED, signer_config.creator.as_ref()], bump = signer_config.bump)]
    pub signer_config: Account<'info, SignerConfig>,

    pub master_authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

impl UpdateSignerConfig<'_> {
    pub fn require_master_authority(&self) -> Result<()> {
        require!(
            self.signer_config.master_authority.is_some(),
            MultiSigError::MasterAuthorityIsNotSupport
        );

        require_keys_eq!(
            self.signer_config.master_authority.unwrap().key(),
            self.master_authority.key(),
            MultiSigError::InvalidMasterAuthority
        );

        Ok(())
    }

    #[access_control(ctx.accounts.require_master_authority())]
    pub fn add_signer_handler(ctx: Context<Self>, params: AddSignerParams) -> Result<()> {
        let signer_config = &mut ctx.accounts.signer_config;
        signer_config.add_signer(params.new_signer);

        SignerConfig::extend_space(
            signer_config.to_account_info(),
            signer_config.signers.len(),
            ctx.accounts.master_authority.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        )?;

        signer_config.validate_post_data()?;

        Ok(())
    }

    #[access_control(ctx.accounts.require_master_authority())]
    pub fn remove_signer_handler(ctx: Context<Self>, params: RemoveSignerParams) -> Result<()> {
        let signer_config = &mut ctx.accounts.signer_config;
        signer_config.remove_signer(params.signer)?;

        Ok(())
    }

    #[access_control(ctx.accounts.require_master_authority())]
    pub fn update_signers_required_handler(
        ctx: Context<Self>,
        params: UpdateSignersRequiredParams,
    ) -> Result<()> {
        let signer_config = &mut ctx.accounts.signer_config;
        signer_config.update_signers_required(params.new_signer_required);

        signer_config.validate_post_data()?;

        Ok(())
    }
}
