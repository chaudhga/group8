//#![feature(trivial_bounds)]
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program_error::ProgramError;

// use mpl_token_metadata::state::Mint;
// use spl_token_nft::instruction::transfer;
// use mpl_token_metadata::state::Account;
// use spl_token::instruction::{initialize_account, initialize_mint, mint_to};
// use std::str::FromStr;
// use anchor_lang::system_program::ID;

declare_id!("25XSG7yVjFTQhcantbjHU4Auw8gmtDk1CdqPqjGwMLTm");

#[derive(Accounts)]
pub struct LockNFT<'info> {
    #[account(mut)]
    sender: Account<'info, Signer>,
    // #[account(mut)]
    // program_account: Account<'info>,
    #[account(mut, constraint = nft_acc.owner == *sender.key, close = nft_acc == sender)]
    nft_acc: Account<'info, NFT>,
    #[account(seeds = [b"lock_nft".as_ref(), nft_acc.to_account_info().key.as_ref()], bump)]
    authority: Signer<'info>,
    rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct NFT<'info> {
    #[account(constraint = nft_acc.owner == *sender.key, close = nft_acc == sender)]
    nft_acc: Account<'info, NFT>,
    sender: Account<'info, Signer>
}

#[account]
pub struct NFT {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub token: Pubkey
}

#[derive(Accounts)]
pub struct SaveNftAddress<'info> {
    #[account(mut)]
    nft: Account<'info, NFT>,
    authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct ApproveNftTransfer<'info> {
    #[account(mut, constraint = nft_acc.owner == *sender.key, close = nft_acc == sender)]
    nft_acc: Account<'info, NFT>,
    #[account(seeds = [b"lock_nft".as_ref(), nft_acc.to_account_info().key.as_ref()], bump)]
    authority: Signer<'info>,
}

#[program]
pub mod fractional_nft {
    use super::*;

    pub fn lock_nft(&mut self, ctx: Context<NFT_to_lock>) -> ProgramResult {
        // descriptive errors are welcome to substitute the check if the NFT is approved
        // TODO add metaplex calls
        // TODO develop logic so address from which NFT is transferred could differ from multisig
        // check that owner is program_id

/* 
Give me an Anchor `fn` which transfer approved metaplex NFT into the program, descriptive errors are welcome to substitute the check if the NFT is approved. Through purpose of the `fn` is to save the address from which the NFT was transferred. Last time your code got cut off.
 */

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

#[state]
pub struct FractionalNFT<'info> {
    pub nft: Box<Account<Nft>>,
    pub nft_owner: Box<Account<Signer>>,
    pub fractional_mint: Box<Account<Mint>>,
    pub fractional_token_account: Box<Account<TokenAccount>>,
    pub multisig: Box<Account<Signer>>,
    pub token_program: Account<'info, TokenProgram>,
}