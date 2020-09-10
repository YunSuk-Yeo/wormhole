//! Error types

use num_derive::FromPrimitive;
use solana_sdk::{decode_error::DecodeError, program_error::ProgramError};
use thiserror::Error;

/// Errors that may be returned by the TokenSwap program.
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum VAAError {
    /// The given action is unknown or invalid
    #[error("InvalidAction")]
    InvalidAction,

    /// An io error occurred
    #[error("IOError")]
    IOError,
}

/// Errors that may be returned by the TokenSwap program.
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum Error {
    /// The deserialization of the Token state returned something besides State::Token.
    #[error("ExpectedToken")]
    ExpectedToken,
    /// The deserialization of the Bridge returned something besides State::Bridge.
    #[error("ExpectedBridge")]
    ExpectedBridge,
    /// The deserialization of the Token state returned something besides State::Account.
    #[error("ExpectedAccount")]
    ExpectedAccount,
    /// The deserialization of the GuardianSet state returned something besides State::GuardianSet.
    #[error("ExpectedGuardianSet")]
    ExpectedGuardianSet,
    /// The deserialization of the TransferOutProposal state returned something besides State::TransferOutProposal.
    #[error("ExpectedTransferOutProposal")]
    ExpectedTransferOutProposal,
    /// The deserialization of the GuardianSet state returned something besides State::WrappedAssetMeta.
    #[error("ExpectedWrappedAssetMeta")]
    ExpectedWrappedAssetMeta,
    /// State is uninitialized.
    #[error("State is unititialized")]
    UninitializedState,
    /// The program address provided doesn't match the value generated by the program.
    #[error("InvalidProgramAddress")]
    InvalidProgramAddress,
    /// The submitted VAA is invalid
    #[error("InvalidVAAFormat")]
    InvalidVAAFormat,
    /// The submitted VAA is invalid form
    #[error("InvalidVAAAction")]
    InvalidVAAAction,
    /// The submitted VAA has an invalid signature
    #[error("InvalidVAASignature")]
    InvalidVAASignature,
    /// The account is already initialized
    #[error("AlreadyExists")]
    AlreadyExists,
    /// An account was not derived correctly
    #[error("InvalidDerivedAccount")]
    InvalidDerivedAccount,
    /// A given token account does not belong to the given mint
    #[error("TokenMintMismatch")]
    TokenMintMismatch,
    /// A given mint account does not belong to the program
    #[error("WrongMintOwner")]
    WrongMintOwner,
    /// A given bridge account does not belong to the program
    #[error("WrongBridgeOwner")]
    WrongBridgeOwner,
    /// A given token account does not belong to the program
    #[error("WrongTokenAccountOwner")]
    WrongTokenAccountOwner,
    /// A parsing operation failed
    #[error("ParseFailed")]
    ParseFailed,
    /// The guardian set that signed this VAA has expired
    #[error("GuardianSetExpired")]
    GuardianSetExpired,
    /// The given VAA has already been claimed
    #[error("VAAClaimed")]
    VAAClaimed,
    /// The given VAA was not signed by the latest guardian set
    #[error("OldGuardianSet")]
    OldGuardianSet,
    /// The guardian set index must increase in steps of 1 on update
    #[error("GuardianIndexNotIncreasing")]
    GuardianIndexNotIncreasing,
    /// The given VAA does not match the proposal
    #[error("VAAProposalMismatch")]
    VAAProposalMismatch,
    /// Invalid transfer with src=dst
    #[error("SameChainTransfer")]
    SameChainTransfer,
    /// VAA is longer than the maximum size
    #[error("VAATooLong")]
    VAATooLong,
    /// Cannot wrap a solana native asset
    #[error("CannotWrapNative")]
    CannotWrapNative,
    /// VAA for this transfer has already been submitted
    #[error("VAAAlreadySubmitted")]
    VAAAlreadySubmitted,
    /// Mismatching guardian set
    #[error("GuardianSetMismatch")]
    GuardianSetMismatch,
}

impl From<Error> for ProgramError {
    fn from(e: Error) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Self {
        Error::ParseFailed
    }
}

impl<T> DecodeError<T> for Error {
    fn type_of() -> &'static str {
        "Swap Error"
    }
}
