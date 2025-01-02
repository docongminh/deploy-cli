use std::cmp::max;

use anchor_lang::{prelude::*, system_program};

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

    pub fn find_signer(&self, signer: Pubkey) -> Option<usize> {
        self.signers.binary_search(&signer).ok()
    }

    pub fn add_signer(&mut self, new_signer: Pubkey) {
        self.signers.push(new_signer);
    }

    pub fn remove_signer(&mut self, signer: Pubkey) -> Result<()> {
        let index = match self.find_signer(signer) {
            Some(index) => index,
            None => todo!(),
        };

        self.signers.remove(index);
        Ok(())
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
