use anchor_lang::prelude::*;

use crate::state::*;
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct AddComment<'info> {
    #[account(
        init,
        payer = commentor, 
        space = Comment::LEN
    )]
    pub comment: Account<'info, Comment>,

    #[account(mut)]
    pub vibe: Account<'info, Vibe>,

    #[account(mut, signer)]
    pub commentor: AccountInfo<'info>,

    #[account(
        mut,
        seeds=[b"vibe_user", commentor.key().as_ref()],
        bump = user.bump,
        constraint = user.user_key == *commentor.key
    )]
    pub user: Account<'info, User>,

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
    
    if vibe.allowed_comments != true {
        return Err(ErrorCode::CommentsNotAllowed.into())
    }

    let comment_account = &mut ctx.accounts.comment;
    let user = &mut ctx.accounts.user;

    let commentor = &mut ctx.accounts.commentor;

    comment_account.add_comment(vibe.key(), content, *commentor.key);

    vibe.comments += 1;
    user.comments += 1;

    Ok(())
}