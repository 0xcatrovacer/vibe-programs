use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct AddLike<'info> {
    #[account(
        init,
        seeds=[liker.key().as_ref(), vibe.key().as_ref()],
        bump,
        payer = liker, 
        space = Like::LEN
    )]
    pub like: Account<'info, Like>,
    #[account(mut)]
    pub vibe: Account<'info, Vibe>,
    #[account(mut, signer)]
    pub liker: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler (ctx: Context<AddLike>) -> Result<()> {

    let vibe = &mut ctx.accounts.vibe;
    let like = &mut ctx.accounts.like;
    let liker = &mut ctx.accounts.liker;

    let bump = *ctx.bumps.get("like").unwrap();
    vibe.likes += 1;

    like.add_like(vibe.key(), *liker.key, bump);

    Ok(())
}