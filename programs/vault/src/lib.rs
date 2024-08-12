use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};

declare_id!("7iBanAfVMtxVLDgPecuoDB3yqaCeTLNfq5ad23VnSmUh");

#[program]
pub mod vault {
    use super::*;
    
    pub fn deposit(ctx: Context<Vault>, lamports: u64) -> Result<()> {
        ctx.accounts.deposit(lamports)
    }

    pub fn withdraw(ctx: Context<Vault>, lamports: u64) -> Result<()> {
        ctx.accounts.withdraw(lamports, &[ctx.bumps.vault])
    }
}

#[derive(Accounts)]
pub struct Vault<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    #[account(
        mut,
        seeds = [signer.key().as_ref()],
        bump
    )]
    vault: SystemAccount<'info>,
    system_program: Program<'info, System>
}

impl<'info> Vault<'info> {
    pub fn deposit(&self, lamports: u64) -> Result<()> {
        let accounts = Transfer {
            from: self.signer.to_account_info(),
            to: self.vault.to_account_info()
        };

        let ctx = CpiContext::new(
            self.system_program.to_account_info(), 
            accounts
        );

        transfer(ctx, lamports)
    }

    pub fn withdraw(&self, lamports: u64, bump: &[u8]) -> Result<()> {
        let signer_seeds = [&[self.signer
        .key.as_ref(), bump][..]];

        let accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.signer.to_account_info()
        };

        let ctx = CpiContext::new_with_signer(
            self.system_program.to_account_info(), 
            accounts, 
            &signer_seeds
        );

        transfer(ctx, lamports)
    }
}