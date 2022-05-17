use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct InitializeVibe<'info> {
    #[account(init, payer = author, space = Vibe::LEN)]
    pub vibe: Account<'info, Vibe>,

    #[account(mut, signer)]
    pub author: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}