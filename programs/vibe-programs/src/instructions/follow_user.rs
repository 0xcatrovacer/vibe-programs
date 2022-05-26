use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct FollowUser<'info> {
    #[account(
        init,
        seeds=[followed.key().as_ref(), follower.key().as_ref()],
        bump,
        payer = follower,
        space = Follow::LEN,
    )]
    pub follow: Account<'info, Follow>,

    #[account(mut)]
    pub followed: AccountInfo<'info>,

    #[account(mut, signer)]
    pub follower: AccountInfo<'info>,

    #[account(
        mut,
        constraint = followed_account.user_key == *followed.key
    )]
    pub followed_account: Account<'info, User>,

    #[account(
        mut,
        constraint = follower_account.user_key == *follower.key
    )]
    pub follower_account: Account<'info, User>,
    
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<FollowUser>) -> Result<()> {

    let follow = &mut ctx.accounts.follow;
    let follower = &mut ctx.accounts.follower;
    let followed = &mut ctx.accounts.followed;

    let followed_account = &mut ctx.accounts.followed_account;
    let follower_account = &mut ctx.accounts.follower_account;

    let bump = *ctx.bumps.get("follow").unwrap();

    follow.followed = *followed.key;
    follow.follower = *follower.key;
    follow.bump = bump;

    followed_account.followers += 1;
    follower_account.followings += 1;

    Ok(())

}