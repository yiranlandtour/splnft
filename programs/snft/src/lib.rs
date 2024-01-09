use anchor_lang::prelude::*;

declare_id!("Bu4AF6Zo6zBFacLdLn53pYJdXQQbdMKSs3GT6w1dP8EL");

#[program]
pub mod snft {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
