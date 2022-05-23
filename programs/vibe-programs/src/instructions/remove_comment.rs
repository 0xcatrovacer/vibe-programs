use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct RemoveComment<'info> {
    #[account(
        mut,
        seeds=[commentor.key().as_ref(), vibe.key().as_ref()],
        bump = comment.bump,
        close = commentor
    )]
    pub comment: Account<'info, Comment>,

    #[account(mut)]
    pub vibe: Account<'info, Vibe>,

    #[account(signer)]
    pub commentor: AccountInfo<'info>,
}

pub fn handler(ctx: Context<RemoveComment>) -> Result<()> {
    let vibe = &mut ctx.accounts.vibe;
    vibe.comments -= 1;

    Ok(())
}