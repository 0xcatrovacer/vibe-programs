use anchor_lang::prelude::*;

use crate::state::*;
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct AddComment<'info> {
    #[account(
        init, 
        seeds=[commentor.key().as_ref(), vibe.key().as_ref()], 
        bump,
        payer = commentor, 
        space = Comment::LEN
    )]
    pub comment: Account<'info, Comment>,

    #[account(mut)]
    pub vibe: Account<'info, Vibe>,

    #[account(mut, signer)]
    pub commentor: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<AddComment>,
    content: String,
) -> Result<()> {

    if content.chars().count() > 150 {
        return Err(ErrorCode::CommentTooLong.into())
    }

    let vibe = &mut ctx.accounts.vibe;
    let comment_account = &mut ctx.accounts.comment;

    let commentor = &mut ctx.accounts.commentor;

    let bump = *ctx.bumps.get("comment").unwrap();

    comment_account.add_comment(vibe.key(), content, *commentor.key, bump);

    vibe.comments += 1;

    Ok(())
}