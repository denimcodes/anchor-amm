#![allow(unexpected_cfgs)]
#![allow(deprecated)]
#![allow(ambiguous_glob_reexports)]

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("3FsVKDy2VVaiATW1eXXTKdofoShxN3mWa5AN19SD4eNS");

#[program]
pub mod anchor_amm {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        seed: u64,
        fee: u16,
        authority: Option<Pubkey>,
    ) -> Result<()> {
        initialize::handler(ctx, seed, fee, authority)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64, max_x: u64, max_y: u64) -> Result<()> {
        deposit::handler(ctx, amount, max_x, max_y)
    }
}
