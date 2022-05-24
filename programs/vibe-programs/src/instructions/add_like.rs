use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct AddLike<'info> {
    #[account(init, payer = liker, space = Like::LEN)]
    pub like: Account<'info, Like>,
    #[account(mut)]
    pub vibe: Account<'info, Vibe>,
    #[account(mut, signer)]
    pub liker: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}