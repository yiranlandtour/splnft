use anchor_lang::prelude::*;
// use spl_token::state::Mint;

use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        create_master_edition_v3, create_metadata_accounts_v3, CreateMasterEditionV3,
        CreateMetadataAccountsV3, Metadata,
    },
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};



declare_id!("Bu4AF6Zo6zBFacLdLn53pYJdXQQbdMKSs3GT6w1dP8EL");

#[program]
pub mod snft {
    
    use anchor_spl::{token::{initialize_mint2, InitializeMint, mint_to, MintTo, transfer, Transfer, burn, Burn, freeze_account, FreezeAccount, close_account, CloseAccount, thaw_account, ThawAccount, set_authority, SetAuthority, spl_token::instruction::AuthorityType}, associated_token, metadata::{create_metadata_accounts_v3, create_master_edition_v3}};

    use super::*;
    pub fn initialize(ctx: Context<Initialize>,authority: Pubkey) -> Result<()> {
        let inita = &mut ctx.accounts.initaccount;
        inita.authority = authority;
        Ok(())
    }

    // pub fn create_token<'_, '_, '_,'b, 'b>(ctx: CpiContext<'_, '_, '_, 'b,InitializeMint2<'b>>, a: u8, authority: Pubkey) -> Result<()> {
    //     initialize_mint2(
    //         ctx,
    //         a,
    //         &authority,
    //         &authority,
    //     )?;
    //     Ok(())
    // }

    // pub fn burn(ctx: Context<Burn>,nft: Pubkey) -> Result<()> {

    //     Ok(())
    // }

    // pub fn sendtoken(ctx: Context<Sendtoken>,nft: Pubkey) -> Result<()> {

    //     Ok(())
    // }

    // pub fn mintnft(ctx: Context<Sendtoken>,nft: Pubkey) -> Result<()> {
    //     Ok(())
    // }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 40)]
    pub initaccount: Account<'info, MyAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// #[derive(Accounts)]
// pub struct Burn<'info> {
//     pub nftAccount: Pubkey,
//     #[account(mut)]
//     pub user: Signer<'info>,
// }

#[account]
pub struct MyAccount {
    pub data: u64,
    pub authority: Pubkey,
}

// #[derive(Accounts)]
// pub struct CreateToken<'info> {
//     #[account(init, payer = user, space = Mint::LEN)]
//     pub token_mint: Account<'info, Mint>,
//     #[account(mut)]
//     pub user: Signer<'info>,
//     pub system_program: Program<'info, System>,
// }

// #[derive(Accounts)]
// pub struct InitializeMint2<'info> {
//     pub mint: AccountInfo<'info>,
// }