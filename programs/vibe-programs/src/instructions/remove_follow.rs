use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct RemoveFollow<'info> {
    #[account(
        mut,
        seeds=[followed.key().as_ref(), follower.key().as_ref()],
        bump = follow.bump,
        close = follower
    )]
    pub follow: Account<'info, Follow>,

    #[account(mut)]
    pub followed: AccountInfo<'info>,

    #[account(mut)]
    pub follower: AccountInfo<'info>,

    #[account(
        mut,
        constraint = followed_account.user_key == *followed.key,
    )]
    pub followed_account: Account<'info, User>,

    #[account(
        mut,
        constraint = follower_account.user_key == *follower.key,
    )]
    pub follower_account: Account<'info, User>,
}

pub fn handler (ctx: Context<RemoveFollow>) -> Result<()> {
    let followed_account = &mut ctx.accounts.followed_account;
    let follower_account = &mut ctx.accounts.follower_account;

    followed_account.followers -= 1;
    follower_account.followings -= 1;

    Ok(())
}