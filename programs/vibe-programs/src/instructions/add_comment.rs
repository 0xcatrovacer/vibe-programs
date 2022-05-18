use anchor_lang::prelude::*;

use crate::state::*;

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

