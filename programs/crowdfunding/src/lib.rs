use anchor_lang::prelude::*;

declare_id!("33uCBHj4K8icQcx37Q9QzWa77LvXYNWcoGDKx5T3JsZS");

#[program]
pub mod crowdfunding {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
