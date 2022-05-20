use crate::states::*;
use anchor_lang::prelude::*;
use std::mem::size_of;

#[derive(Accounts)]
#[instruction(tick: i32)]
pub struct InitTickAccount<'info> {
    /// Pays to create tick account
    #[account(mut)]
    pub signer: Signer<'info>,

    /// Create a tick account for this pool
    pub pool_state: AccountLoader<'info, PoolState>,

    /// The tick account to be initialized
    #[account(
        init,
        seeds = [
            TICK_SEED.as_bytes(),
            pool_state.load()?.token_0.as_ref(),
            pool_state.load()?.token_1.as_ref(),
            &pool_state.load()?.fee.to_be_bytes(),
            &tick.to_be_bytes()
        ],
        bump,
        payer = signer,
        space = 8 + size_of::<TickState>()
    )]
    pub tick_state: AccountLoader<'info, TickState>,

    /// Program to initialize the tick account
    pub system_program: Program<'info, System>,
}
