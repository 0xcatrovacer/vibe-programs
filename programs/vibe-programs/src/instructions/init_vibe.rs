use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct InitializeVibe<'info> {
    #[account(init, payer = author, space = Vibe::LEN)]
    pub vibe: Account<'info, Vibe>,

    #[account(mut, signer)]
    pub author: AccountInfo<'info>,

    pub clock: Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
}


// Initialize an account that will store a Vibe
pub fn handler(
    ctx: Context<InitializeVibe>, 
    vibe_content: Vec<VibeContent>, 
    allowed_comments: bool
    ) -> Result<()> {

    let vibe = &mut ctx.accounts.vibe;
    let author = &mut ctx.accounts.author;
    let clock = &mut ctx.accounts.clock;

    vibe.author = *author.key;
    vibe.vibe_content = vibe_content;
    vibe.version = 0;
    vibe.likes = 0;
    vibe.comments = 0;
    vibe.allowed_comments = allowed_comments;
    vibe.timestamp = clock.unix_timestamp;

    Ok(())
}