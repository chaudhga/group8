use anchor_lang::prelude::*;
use spl_token_nft::state::Nft;
use spl_token_nft::instruction::transfer;
use spl_token::state::Account as TokenAccount;
use spl_token::instruction::{initialize_account, initialize_mint, mint_to};
use std::str::FromStr;

#[program]
pub mod fractional_nft {
    use super::*;

    #[state]
    pub struct FractionalNFT {
        pub nft: Box<Account<Nft>>,
        pub nft_owner: Box<Account<Signer>>,
        pub fractional_mint: Box<Account<Mint>>,
        pub fractional_token_account: Box<Account<TokenAccount>>,
        pub multisig: Box<Account<Signer>>,
        pub token_program: Account<'info, TokenProgram>,
    }

    impl<'info> FractionalNFT<'info> {
        pub fn lock_nft(&mut self, ctx: Context<LockNFT>) -> ProgramResult {
            // Transfer the NFT to the multisig
            let cpi_program = ctx.accounts.nft_program.clone();
            let cpi_accounts = Transfer {
                from: ctx.accounts.from.clone(),
                to: ctx.accounts.multisig.to_account_info().clone(),
                authority: ctx.accounts.from.clone(),
            };
            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
            nft_program::transfer(cpi_ctx, ctx.accounts.nft_id)?;

            // Initialize the fractional mint
            let fractional_mint_account = self.fractional_mint.to_account_info();
            let mint_authority = ctx.accounts.from.to_account_info();
            let cpi_accounts = mint::InitializeMint {
                mint: fractional_mint_account.clone(),
                mint_authority: mint_authority.clone(),
                rent: Rent::from_account_info(&ctx.accounts.sysvar_rent)?,
            };
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
            mint::initialize_mint(cpi_ctx)?;

            // Create the token account for the fractional NFT
            let fractional_token_account_info = self.fractional_token_account.to_account_info();
            let cpi_accounts = initialize_account(
                &ctx.accounts.token_program.to_account_info(),
                fractional_token_account_info.clone(),
                &fractional_mint_account.key(),
                &ctx.accounts.from.key(),
            )?;
            let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
            initialize_account(cpi_ctx)?;

            // Mint all of the fractional tokens to the multisig
            let cpi_accounts = mint::MintTo {
                mint: self.fractional_mint.to_account_info().clone(),
                to: self.multisig.to_account_info().clone(),
                authority: mint_authority.clone(),
                amount: ctx.accounts.nft_amount,
            };
            let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
            mint::mint_to(cpi_ctx)?;

            Ok(())
        }
    }
}
