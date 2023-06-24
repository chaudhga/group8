use anchor_lang::prelude::*;

declare_id!("25XSG7yVjFTQhcantbjHU4Auw8gmtDk1CdqPqjGwMLTm");

#[program]
pub mod group8 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
