use anchor_lang::prelude::*;


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
    use super::*;

    pub fn initialize(ctx: Context<Initialize>,authority: Pubkey) -> Result<()> {
        let inita = &mut ctx.accounts.initaccount;
        inita.authority = authority;
        Ok(())
    }

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

