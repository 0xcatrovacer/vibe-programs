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

const DISCRIMINATOR_LENGTH: usize = 8;
const VIBE_LENGTH: usize = 32;
const CONTENT_LENGTH: usize = 150 * 4;
const COMMENTOR_LENGTH: usize = 32;
const BUMP_LENGTH: usize = 1;

impl Comment {

    // Length of comment
    pub const LEN: usize = DISCRIMINATOR_LENGTH +
                VIBE_LENGTH +
                CONTENT_LENGTH +
                COMMENTOR_LENGTH +
                BUMP_LENGTH;

    // Add a Comment to a Vibe
    pub fn add_comment(
        &mut self, 
        vibe: Pubkey, 
        content: String, 
        commentor: Pubkey, 
        bump: u8
    ) {
        self.vibe = vibe;
        self.content = content;
        self.commentor = commentor;
        self.bump = bump;
    }
}