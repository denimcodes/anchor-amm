use anchor_lang::prelude::*;

#[error_code]
pub enum AmmError {
    #[msg("Amount is not valid")]
    InvalidAmount,
    #[msg("Pool is locked")]
    PoolLocked,
    #[msg("Slippage has exceeded")]
    SlippageExceeded,
}
