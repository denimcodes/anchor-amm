use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{burn, transfer, Burn, Mint, Token, TokenAccount, Transfer},
};
use constant_product_curve::ConstantProduct;

use crate::{error::AmmError, Config};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub mint_x: Account<'info, Mint>,
    pub mint_y: Account<'info, Mint>,
    #[account(
        mut,
        seeds = [b"lp", config.key().as_ref()],
        bump = config.lp_bump
    )]
    pub mint_lp: Account<'info, Mint>,
    #[account(
        mut,
        seeds = [b"config", config.seed.to_le_bytes().as_ref()],
        bump = config.config_bump,
        has_one = mint_x,
        has_one = mint_y
    )]
    pub config: Account<'info, Config>,
    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = config,
    )]
    pub vault_x: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = config,
    )]
    pub vault_y: Account<'info, TokenAccount>,
    #[account(
           init_if_needed,
           payer = user,
           associated_token::mint = mint_x,
           associated_token::authority = user,
       )]
    pub user_x: Account<'info, TokenAccount>,
    #[account(
           init_if_needed,
           payer = user,
           associated_token::mint = mint_y,
           associated_token::authority = user,
       )]
    pub user_y: Account<'info, TokenAccount>,
    #[account(
           init_if_needed,
           payer = user,
           associated_token::mint = mint_lp,
           associated_token::authority = user,
       )]
    pub user_lp: Account<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> Withdraw<'info> {
    fn withdraw_token(&mut self, is_x: bool, amount: u64) -> Result<()> {
        let signer_seeds: [&[&[u8]]; 1] = [&[
            b"config",
            &self.config.seed.to_le_bytes()[..],
            &[self.config.lp_bump],
        ]];

        let (from, to) = match is_x {
            true => (
                self.vault_x.to_account_info(),
                self.user_x.to_account_info(),
            ),
            false => (
                self.vault_y.to_account_info(),
                self.user_y.to_account_info(),
            ),
        };

        let cpi_accounts = Transfer {
            from,
            to,
            authority: self.user.to_account_info(),
        };
        let cpi_context = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            cpi_accounts,
            &signer_seeds,
        );
        transfer(cpi_context, amount)
    }

    fn withdraw(&mut self, amount: u64, min_x: u64, min_y: u64) -> Result<()> {
        require!(self.config.locked != false, AmmError::PoolLocked);
        require!(amount != 0, AmmError::InvalidAmount);
        require!(min_x != 0 && min_y != 0, AmmError::InvalidAmount);

        let amounts = ConstantProduct::xy_withdraw_amounts_from_l(
            self.vault_x.amount,
            self.vault_y.amount,
            self.mint_lp.supply,
            amount,
            6,
        )
        .unwrap();

        require!(min_x <= amounts.x, AmmError::SlippageExceeded);
        require!(min_y <= amounts.y, AmmError::SlippageExceeded);

        self.withdraw_token(true, amounts.x)?;
        self.withdraw_token(false, amounts.y)?;
        self.burn_lp_tokens(amount)?;

        Ok(())
    }

    fn burn_lp_tokens(&mut self, amount: u64) -> Result<()> {
        let accounts = Burn {
            mint: self.mint_lp.to_account_info(),
            from: self.user_x.to_account_info(),
            authority: self.user.to_account_info(),
        };
        let program = self.token_program.to_account_info();
        let ctx = CpiContext::new(program, accounts);
        burn(ctx, amount)?;

        Ok(())
    }
}

pub fn handler(ctx: Context<Withdraw>, amount: u64, min_x: u64, min_y: u64) -> Result<()> {
    ctx.accounts.withdraw(amount, min_x, min_y)
}
