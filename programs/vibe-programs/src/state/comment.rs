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

impl Comment {
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