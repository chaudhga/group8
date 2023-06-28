use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;
// use mpl_token_metadata::state::Metadata;
// use mpl_token_metadata::instruction::Transfer;
use anchor_spl::token::{TokenAccount, Mint, Token};
use anchor_spl::token::spl_token::{instruction::transfer};
use anchor_spl::associated_token;
// use anchor_lang::system_program::ID;
// use solana_program::account_info::{Account, next_account_info};

declare_id!("25XSG7yVjFTQhcantbjHU4Auw8gmtDk1CdqPqjGwMLTm");

#[derive(Accounts)]
pub struct LockNFT<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,
    #[account(mut)]
    pub sender_token_account: Account<'info, TokenAccount>,
    
    // #[account(mut)]
    // pub to: Account<'info, Signer>,
    // TODO add checks if NFT been ever distributed already with primary sell
    // TODO add metaplex stuff so royalties, authorities and other stuff would be taked into account too
    // #[account(mut)]
    // pub metadata: Account<'info, Metadata>,

    #[account(
        init, 
        // just a reasonable top estimate
        space = 100,
        payer = sender, 
        seeds = [
            b"lock_nft".as_ref(), 
            &sender_token_account.mint.to_bytes()[..]
        ], 
        bump
    )]
    /// CHECK: TODO why no checks through types are necessary
    pub nft: UncheckedAccount<'info>,
    #[account(mut)]
    pub locking_token_account: Signer<'info>,
    // #[account(mut)]
    pub program: Signer<'info>,
    // pub program_authority: Account<'info, Signer>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub mint: Account<'info, Mint>,
}

#[program]
pub mod group8 {
    // use core::slice::SlicePattern;
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        // Initialize the SPL token lock account
        Ok(())
    }

    pub fn lock_nft(ctx: Context<LockNFT>) -> /* Program */Result<()> {
        let cpi_program = ctx.accounts.token_program.to_account_info().clone();
        let preseeds = [
            b"lock_nft",
            &ctx.accounts.sender_token_account.mint.to_bytes()[..],
        ];
        let seeds = [
            preseeds.as_slice(),
        ];
        let cpi_ctx: CpiContext<'_, '_, '_, '_, associated_token::Create> = 
            CpiContext::new_with_signer(
                cpi_program, 
                // TOSO see relevant comments below in `transfer` invokation
                associated_token::Create {
                    payer: ctx.accounts.sender.to_account_info(), 
                    associated_token: ctx.accounts.locking_token_account.to_account_info(), 
                    authority: ctx.accounts.sender.to_account_info(), 
                    mint: ctx.accounts.mint.to_account_info(), 
                    system_program: ctx.accounts.system_program.to_account_info(), 
                    token_program: ctx.accounts.token_program.to_account_info()
                },
                seeds.as_slice()
            );
            // ).with_signer(&[&[&ctx.accounts.program.to_account_info().key.to_bytes()]]);


    
        // let token_lock_data = &mut ctx.accounts.token_lock.try_borrow_mut_data()?;
        // let token_lock = spl_token_lock::TokenLock {
        //     bump,
        //     owner: *ctx.accounts.program.to_account_info().key,
        //     mint: *ctx.accounts.token_account.mint.key,
        //     token_account: *ctx.accounts.token_account.key,
        // };
        // spl_token_lock::TokenLock::pack(token_lock, token_lock_data)?;

        let associated_token_address = associated_token::get_associated_token_address(
            &ctx.accounts.program.key,
            // &cpi_ctx.accounts.token_program.key,
            &ctx.accounts.nft.key,
        );
        if &associated_token_address != ctx.accounts.locking_token_account.to_account_info().key {
            // msg!("client asks for wrong token receiver!");
            // ~~TODO make a resonable error (`struct`?)~~
            return err!(MyError::AssocAccDonotMatch);
        }
        // TODO close the account on NFT redeem.
        // TODO set the owner the program, and think if there's possible scheming when it's not
        let _create_associated_token_account = associated_token::create(
            // &ctx.accounts.sender.key(),
            // &associated_token_address,
            // &ctx.accounts.nft.key,
            // &spl_token::id(),
            cpi_ctx
        );
        
        // solana_program::program::invoke(
        //     &create_associated_token_account,
        //     &[
        //         ctx.accounts.sender.to_account_info(),
        //         ctx.accounts.program.to_account_info(),
        //         ctx.accounts.locking_token_account.to_account_info(),
        //         ctx.accounts.token_program.clone(),
        //     ],
        // )?;        

        // Transfer the SPL token to the program-controlled account
        //      we **assume** here that NFT is approved to transfer
        //      TODO add approve instruction/function OR maybe better add in this one
        // let cpi_accounts = Transfer {
        //     from: ctx.accounts.sender_token_account.to_account_info().clone(),
        //     to: ctx.accounts.locking_token_account.to_account_info().clone(),
        //     authority: ctx.accounts.from.to_account_info().clone(),
        // };
        // In transfer we used the account we just created. *If client derived it wrong, tx will fail*
        let _ = transfer(
            &ctx.accounts.token_program.to_account_info().clone().key(), 
            &associated_token_address, 
            &ctx.accounts.nft.to_account_info().clone().key(), 
            // TODO generalize the whole thing to the case when authority isn't `sender`
            &ctx.accounts.sender.to_account_info().clone().key(), 
            &[&ctx.accounts.sender.to_account_info().clone().key()], 
            1
        );

        Ok(())
    }

    #[error_code]
    pub enum MyError {
        #[msg("client asks for wrong token receiver!")]
        AssocAccDonotMatch
    }
    #[derive(Accounts)]
    pub struct Initialize {
        // Your account definitions here, if necessary
    }
}
