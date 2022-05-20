use crate::states::*;
use anchor_lang::prelude::*;
use std::mem::size_of;

#[derive(Accounts)]
#[instruction(fee: u32, tick_spacing: u16)]
pub struct EnableFeeAmount<'info> {
    /// Valid protocol owner
    #[account(mut, address = factory_state.load()?.owner)]
    pub owner: Signer<'info>,

    /// Factory state stores the protocol owner address
    #[account(mut)]
    pub factory_state: AccountLoader<'info, FactoryState>,

    /// Initialize an account to store new fee tier and tick spacing
    /// Fees are paid by owner
    #[account(
        init,
        seeds = [FEE_SEED.as_bytes(), &fee.to_be_bytes()],
        bump,
        payer = owner,
        space = 8 + size_of::<FeeState>()
    )]
    pub fee_state: AccountLoader<'info, FeeState>,

    /// To create a new program account
    pub system_program: Program<'info, System>,
}

pub fn enable_fee_amount(ctx: Context<EnableFeeAmount>, fee: u32, tick_spacing: u16) -> Result<()> {
    assert!(fee < 1_000_000); // 100%

    // TODO examine max value of tick_spacing
    // tick spacing is capped at 16384 to prevent the situation where tick_spacing is so large that
    // tick_bitmap#next_initialized_tick_within_one_word overflows int24 container from a valid tick
    // 16384 ticks represents a >5x price change with ticks of 1 bips
    let mut fee_state = ctx.accounts.fee_state.load_init()?;
    assert!(tick_spacing > 0 && tick_spacing < 16384);
    fee_state.bump = *ctx.bumps.get("fee_state").unwrap();
    fee_state.fee = fee;
    fee_state.tick_spacing = tick_spacing;

    emit!(FeeAmountEnabled { fee, tick_spacing });
    Ok(())
}
