use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint, Transfer};

declare_id!("AHuxcfexoU55eDhM8V5jk2U6ZJocjHVn8H5Y1RYm5qSC");

#[program]
pub mod token_timelock_vault {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        release_time: i64,
        amount: u64,
    ) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.authority = ctx.accounts.authority.key();
        vault.beneficiary = ctx.accounts.beneficiary.key();
        vault.mint = ctx.accounts.mint.key();
        vault.release_time = release_time;
        vault.amount = amount;
        vault.released = false;

        // Transfiere tokens al vault
        token::transfer(
            ctx.accounts.into_transfer_to_vault_context(),
            amount,
        )?;
        Ok(())
    }

    pub fn release(ctx: Context<Release>) -> Result<()> {
        let clock = Clock::get()?;
        let vault = &mut ctx.accounts.vault;
        require!(
            clock.unix_timestamp >= vault.release_time,
            TimelockError::ReleaseTimeNotReached
        );
        require!(!vault.released, TimelockError::AlreadyReleased);

        vault.released = true;

        // Transfiere tokens del vault al beneficiario
        token::transfer(
            ctx.accounts.into_transfer_to_beneficiary_context(),
            vault.amount,
        )?;
        Ok(())
    }
}

#[account]
pub struct Vault {
    pub authority: Pubkey,
    pub beneficiary: Pubkey,
    pub mint: Pubkey,
    pub release_time: i64,
    pub amount: u64,
    pub released: bool,
}

#[derive(Accounts)]
#[instruction(release_time: i64, amount: u64)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    /// CHECK: Solo se usa como referencia
    pub beneficiary: UncheckedAccount<'info>,
    pub mint: Account<'info, Mint>,
    #[account(
        init,
        payer = authority,
        space = 8 + 32*3 + 8 + 8 + 1 // Vault struct size
    )]
    pub vault: Account<'info, Vault>,
    #[account(
        mut,
        constraint = from.owner == authority.key()
    )]
    pub from: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = authority,
        token::mint = mint,
        token::authority = vault
    )]
    pub vault_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> Initialize<'info> {
    fn into_transfer_to_vault_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.from.to_account_info().clone(),
            to: self.vault_token_account.to_account_info().clone(),
            authority: self.authority.to_account_info().clone(),
        };
        CpiContext::new(self.token_program.to_account_info().clone(), cpi_accounts)
    }
}

#[derive(Accounts)]
pub struct Release<'info> {
    #[account(mut, has_one = beneficiary, has_one = mint)]
    pub vault: Account<'info, Vault>,
    pub beneficiary: Signer<'info>,
    pub mint: Account<'info, Mint>,
    #[account(
        mut,
        constraint = vault_token_account.owner == vault.key(),
        constraint = vault_token_account.mint == mint.key()
    )]
    pub vault_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub beneficiary_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

impl<'info> Release<'info> {
    fn into_transfer_to_beneficiary_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.vault_token_account.to_account_info().clone(),
            to: self.beneficiary_token_account.to_account_info().clone(),
            authority: self.vault.to_account_info().clone(),
        };
        CpiContext::new(self.token_program.to_account_info().clone(), cpi_accounts)
    }
}

#[error_code]
pub enum TimelockError {
    #[msg("Aún no se cumple el tiempo de liberación.")]
    ReleaseTimeNotReached,
    #[msg("Tokens ya fueron liberados.")]
    AlreadyReleased,
}
