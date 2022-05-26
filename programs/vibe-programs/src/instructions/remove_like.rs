use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct RemoveLike<'info> {
    #[account(
        mut,
        seeds=[liker.key().as_ref(), vibe.key().as_ref()],
        bump = like.bump,
        has_one = liker,
        close = liker
    )]
    pub like: Account<'info, Like>,

    #[account(mut)]
    pub vibe: Account<'info, Vibe>,

    #[account(mut, signer)]
    pub liker: AccountInfo<'info>,
        
    #[account(
        mut,
        seeds=[b"vibe_user", liker.key().as_ref()],
        bump = user.bump,
        constraint = user.user_key == *liker.key
    )]
    pub user: Account<'info, User>,

}

pub fn handler(ctx: Context<RemoveLike>) -> Result<()> {
    let vibe = &mut ctx.accounts.vibe;

    vibe.likes -= 1;

    Ok(())
}