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
    
    #[account(
        mut,
        seeds=[b"vibe_user", commentor.key().as_ref()],
        bump = user.bump,
        constraint = user.user_key == *commentor.key
    )]
    pub user: Account<'info, User>,

}

pub fn handler(ctx: Context<RemoveComment>) -> Result<()> {
    let vibe = &mut ctx.accounts.vibe;
    let user = &mut ctx.accounts.user;

    vibe.comments -= 1;
    user.comments -= 1;

    Ok(())
}