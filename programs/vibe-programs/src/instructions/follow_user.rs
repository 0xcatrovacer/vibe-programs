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
    pub system_program: Program<'info, System>,
}

// pub fn handler(ctx: Context<FollowUser>) -> Result<()> {

//     let follow = &mut ctx.accounts.follow;
//     let follower = &mut ctx.accounts.follower;
//     let followed = &mut ctx.accounts.followed;

//     Ok(())

// }