use anchor_lang::prelude::*;

#[error_code]
#[derive(PartialEq)]
pub enum MultiSigError {
    #[msg("unauthorize for this action")]
    Unauthorize,
    #[msg("Two pubkey can not duplicate")]
    DuplicatePubkey,
    #[msg("Master authority is not support")]
    MasterAuthorityIsNotSupport,
    #[msg("Invalid master authority")]
    InvalidMasterAuthority,
    #[msg("Signer is not existed")]
    SignerIsNotExisted,
}
