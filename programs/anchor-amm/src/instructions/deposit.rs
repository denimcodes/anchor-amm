use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, transfer, Mint, MintTo, Token, TokenAccount, Transfer},
};
use constant_product_curve::ConstantProduct;

use crate::{error::AmmError, Config};

#[derive(Accounts)]
pub struct Deposit<'info> {
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
           mut,
           associated_token::mint = mint_x,
           associated_token::authority = user,
       )]
    pub user_x: Account<'info, TokenAccount>,
    #[account(
           mut,
           associated_token::mint = mint_y,
           associated_token::authority = user,
       )]
    pub user_y: Account<'info, TokenAccount>,
    #[account(
           init_if_needed,
           payer = user,
           associated_token::mint = mint_y,
           associated_token::authority = user,
       )]
    pub user_lp: Account<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> Deposit<'info> {
    fn deposit_token(&mut self, is_x: bool, amount: u64) -> Result<()> {
        let (from, to) = match is_x {
            true => (
                self.user_x.to_account_info(),
                self.vault_x.to_account_info(),
            ),
            false => (
                self.user_y.to_account_info(),
                self.vault_y.to_account_info(),
            ),
        };

        let cpi_accounts = Transfer {
            from,
            to,
            authority: self.user.to_account_info(),
        };
        let cpi_context = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);
        transfer(cpi_context, amount)
    }

    fn mint_lp_tokens(&mut self, amount: u64) -> Result<()> {
        let signer_seeds: [&[&[u8]]; 1] = [&[
            b"config",
            &self.config.seed.to_le_bytes()[..],
            &[self.config.lp_bump],
        ]];

        let mint_accounts = MintTo {
            mint: self.mint_lp.to_account_info(),
            to: self.user_lp.to_account_info(),
            authority: self.config.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            mint_accounts,
            &signer_seeds,
        );

        mint_to(cpi_ctx, amount)
    }

    fn deposit(&mut self, amount: u64, max_x: u64, max_y: u64) -> Result<()> {
        require!(self.config.locked != false, AmmError::PoolLocked);
        require!(amount != 0, AmmError::InvalidAmount);

        let (x, y) = match self.mint_lp.supply == 0
            && self.vault_x.amount == 0
            && self.vault_y.amount == 0
        {
            true => (max_x, max_y),
            false => {
                let amounts = ConstantProduct::xy_deposit_amounts_from_l(
                    self.vault_x.amount,
                    self.vault_y.amount,
                    self.mint_lp.supply,
                    amount,
                    6,
                )
                .unwrap();
                (amounts.x, amounts.y)
            }
        };

        require!(x <= max_x && y <= max_y, AmmError::SlippageExceeded);

        self.deposit_token(true, x)?;
        self.deposit_token(false, y)?;

        self.mint_lp_tokens(amount)?;

        Ok(())
    }
}

pub fn handler(ctx: Context<Deposit>, amount: u64, max_x: u64, max_y: u64) -> Result<()> {
    ctx.accounts.deposit(amount, max_x, max_y)
}
