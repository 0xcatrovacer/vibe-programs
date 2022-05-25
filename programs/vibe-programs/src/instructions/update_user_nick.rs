use anchor_lang::prelude::*;

use crate::state::*;
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct UpdateNick<'info> {
    #[account(
        mut,
        seeds = [b"vibe_user", author.key().as_ref()],
        bump = user.bump,
        constraint = user.user_key == *author.key
    )]
    pub user: Account<'info, User>,

    #[account(mut, signer)]
    pub author: AccountInfo<'info>
}

pub fn handler (ctx: Context<UpdateNick>, nick: String) -> Result<()> {

    if nick.chars().count() > 20 {
        return Err(ErrorCode::NickTooLong.into())
    }

    let user = &mut ctx.accounts.user;

    user.nick = nick;

    Ok(())
}