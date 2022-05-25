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

    #[account(mut, signer)]
    pub author: AccountInfo<'info>,
}

pub fn handler (_ctx: Context<RemoveVibe>) -> Result<()> {
    Ok(())
}