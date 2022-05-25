use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct RemoveLike<'info> {
    #[account(
        mut,
        seeds=[liker.key().as_ref(), vibe.key().as_ref()],
        bump = like.bump,
        close = liker
    )]
    pub like: Account<'info, Like>,
    #[account(mut)]
    pub vibe: Account<'info, Vibe>,
    #[account(mut, signer)]
    pub liker: AccountInfo<'info>,
}

pub fn handler(ctx: Context<RemoveLike>) -> Result<()> {
    let vibe = &mut ctx.accounts.vibe;

    vibe.likes -= 1;

    Ok(())
}