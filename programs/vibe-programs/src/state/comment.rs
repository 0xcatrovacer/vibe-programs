use anchor_lang::prelude::*;

#[account]
pub struct Comment {
    // Vibe where the comment is being made
    pub vibe: Pubkey,

    // Content of the Comment
    pub content: String,

    // Commentor
    pub commentor: Pubkey,

    // bump
    pub bump: u8
}

