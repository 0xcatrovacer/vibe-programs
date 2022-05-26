use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct RemoveVibe<'info> {
    #[account(
        mut,
        has_one = author,
        close = author,
    )]
    pub vibe: Account<'info, Vibe>,

    #[account(
        mut,
        seeds=[b"vibe_user", author.key().as_ref()],
        bump = user.bump,
        constraint = user.user_key == *author.key
    )]
    pub user: Account<'info, User>,

    #[account(mut, signer)]
    pub author: AccountInfo<'info>,
}

pub fn handler (ctx: Context<RemoveVibe>) -> Result<()> {

    let user = &mut ctx.accounts.user;

    user.vibes -= 1;

    Ok(())
}