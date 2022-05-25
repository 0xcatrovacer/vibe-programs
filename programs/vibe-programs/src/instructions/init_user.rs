use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(
        init,
        seeds = [b"vibe_user", author.key().as_ref()],
        bump,
        payer = author,
        space = User::LEN,
    )]
    pub user: Account<'info, User>,
    #[account(mut, signer)]
    pub author: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler (ctx: Context<InitializeUser>, nick: String) -> Result<()> {

    let user = &mut ctx.accounts.user;
    let author = &mut ctx.accounts.author;

    let bump = *ctx.bumps.get("user").unwrap();

    user.user_key = *author.key;
    user.nick = nick;
    user.followers = 0;
    user.followings = 0;
    user.bump = bump;

    Ok(())

}