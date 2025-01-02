use anchor_lang::prelude::*;

#[error_code]
#[derive(PartialEq)]
pub enum MultiSigError {
    #[msg("unauthorize for this action")]
    Unauthorize,
    #[msg("Signers can not duplicate")]
    DuplicateSigner,
    #[msg("Master authority is not support")]
    MasterAuthorityIsNotSupport,
    #[msg("Invalid master authority")]
    InvalidMasterAuthority,
    #[msg("Signer is not existed")]
    SignerIsNotExisted,
    #[msg("Too many signers")]
    TooManySigners,
    #[msg("InvalidSignerRequired")]
    InvalidSignerRequired,
}
