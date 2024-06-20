mod errors;
use anchor_lang::prelude::*;
use crate::{errors::*};
use solana_program::{program::{invoke,invoke_signed}, system_instruction};

declare_id!("61gPL8pSqVhXNk9ovzLQ8KMLmtL1J3WMHmfHwXCo7b45");

use std::mem::size_of;

#[program]
pub mod n_raffle {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let accts = ctx.accounts;
    
        accts.global_state.owner = accts.owner.key();
        accts.global_state.vault = accts.vault.key();
    
        Ok(())
    }

    pub fn deposit_sol(ctx: Context<DepositPool>, amount: u64) -> Result<()> {
        let accts = ctx.accounts;

        //  deposit sol from owner to valut account for reward pool
        invoke(
            &system_instruction::transfer(
                &accts.user.key(),
                &accts.vault.key(),
                amount
            ),
            &[
                accts.user.to_account_info().clone(),
                accts.vault.to_account_info().clone(),
                accts.system_program.to_account_info().clone(),
            ],
        )?;
        
        Ok(())
    }
    
    pub fn withdraw_pool(ctx: Context<WithdrawPool>, amount: u64) -> Result<()> {
        let accts = ctx.accounts;
        
        if accts.global_state.owner != accts.owner.key() {
            return Err(RaffleError::NotAllowedOwner.into());
        }
        
        //  withdraw sol from vault to owner account to refund the reward pool
        let (_, bump) = Pubkey::find_program_address(&[VAULT_SEED], &crate::ID);
    
        invoke_signed(
            &system_instruction::transfer(&accts.vault.key(), &accts.to.key(), amount),
            &[
                accts.vault.to_account_info().clone(),
                accts.to.to_account_info().clone(),
                accts.system_program.to_account_info().clone(),
            ],
            &[&[VAULT_SEED, &[bump]]],
        )?;
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    
    #[account(
        init,
        payer = owner,
        seeds = [GLOBAL_STATE_SEED],
        bump,
        space = 8 + size_of::<GlobalState>()
    )]
    pub global_state: Account<'info, GlobalState>,

    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(
        mut,
        seeds = [VAULT_SEED],
        bump
    )]
    pub vault: AccountInfo<'info>, // to receive SOL

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DepositPool<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [GLOBAL_STATE_SEED],
        bump
    )]
    pub global_state: Account<'info, GlobalState>,

    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(
        mut,
        address = global_state.vault
    )]
    pub vault: AccountInfo<'info>, // to receive SOL

    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct WithdrawPool<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    /// CHECK:` doc comment explaining why no checks through types are necessary
    #[account(mut)]
    pub to: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [GLOBAL_STATE_SEED],
        bump
    )]
    pub global_state: Account<'info, GlobalState>,

    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(
        mut,
        address = global_state.vault
    )]
    pub vault: AccountInfo<'info>, // to receive SOL

    pub system_program: Program<'info, System>
}

#[account]
#[derive(Default)]
pub struct GlobalState {
    pub owner: Pubkey, // the pubkey of owner
    pub vault: Pubkey,
}

pub const GLOBAL_STATE_SEED: &[u8] = b"GLOBAL-STATE-SEED";
pub const VAULT_SEED: &[u8] = b"VAULT-SEED";
