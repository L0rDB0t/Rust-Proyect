use anchor_lang::prelude::*;

declare_id!("AHuxcfexoU55eDhM8V5jk2U6ZJocjHVn8H5Y1RYm5qSC");

#[program]
pub mod token_timelock_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
