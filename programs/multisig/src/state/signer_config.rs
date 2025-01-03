use std::cmp::max;

use anchor_lang::{prelude::*, system_program};

use crate::MultiSigError;

#[account]
pub struct SignerConfig {
    pub creator: Pubkey,
    /// master authority that can update signer config
    /// if master authority is None any update for signer config will be update by normal process by voting with signers member
    pub master_authority: Option<Pubkey>,

    /// number of signers must sign to trigger action.
    pub signers_required: u16,
    pub signers: Vec<Pubkey>,
    pub bump: u8,
}

impl SignerConfig {
    pub fn space(num_signers: usize) -> usize {
        8 + 1 + 32 * (2 + num_signers) + 1 + 2
    }

    pub fn validate_post_data(&self) -> Result<()> {
        require!(
            self.signers.len() <= usize::from(u16::MAX),
            MultiSigError::TooManySigners
        );

        let is_duplicate = self.signers.windows(2).any(|w| w[0] == w[1]);
        require!(!is_duplicate, MultiSigError::DuplicateSigner);

        require!(self.signers_required > 0, MultiSigError::InvalidSignerRequired);
        
        Ok(())
    }

    pub fn update_signers_required(&mut self, new_signer_required: u16) {
        self.signers_required = new_signer_required
    }

    pub fn find_signer(&self, signer: Pubkey) -> Option<usize> {
        self.signers.binary_search(&signer).ok()
    }

    pub fn add_signer(&mut self, new_signer: Pubkey) {
        self.signers.push(new_signer);
    }

    pub fn remove_signer(&mut self, signer: Pubkey) -> Result<()> {
        let index = match self.find_signer(signer) {
            Some(index) => index,
            None => return err!(MultiSigError::SignerIsNotExisted),
        };

        self.signers.remove(index);
        Ok(())
    }

    /// Reject Tx if nof rejected > total_signers - signer_required.
    /// derive: reject_threshold = total_signers - signer_required + 1;
    /// rejected >= reject_threshold will drop Tx.
    pub fn reject_threshold(&self) -> usize {
        self.signers
            .len()
            .checked_sub(self.signers_required as usize)
            .unwrap()
            .checked_add(1)
            .unwrap()
    }

    pub fn extend_space<'info>(
        signer_config: AccountInfo<'info>,
        num_signers: usize,
        payer: AccountInfo<'info>,
        system_program: AccountInfo<'info>,
    ) -> Result<bool> {
        let current_data_size = signer_config.data.borrow().len();
        let data_size_to_fit = SignerConfig::space(num_signers);

        if current_data_size >= data_size_to_fit {
            return Ok(false);
        }

        let new_size = max(current_data_size, data_size_to_fit);

        // Reallocate more space for account
        AccountInfo::realloc(&signer_config, new_size, false)?;

        // calculate new rent fee for new space
        let new_rent_lamports = Rent::get().unwrap().minimum_balance(new_size).max(1);
        let extend_rent_lamports =
            new_rent_lamports.saturating_sub(signer_config.to_account_info().lamports());

        // transfer lamports to pay extend space
        if extend_rent_lamports > 0 {
            system_program::transfer(
                CpiContext::new(
                    system_program,
                    system_program::Transfer {
                        from: payer,
                        to: signer_config,
                    },
                ),
                extend_rent_lamports,
            )?;
        }
        Ok(true)
    }
}
