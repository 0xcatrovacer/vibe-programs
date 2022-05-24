use anchor_lang::prelude::*;

#[account]
pub struct Like {
    // The Vibe that will be liked
    pub vibe: Pubkey,

    // The user who will like the Vibe
    pub liker: Pubkey,

    // Bump
    pub bump: u8,
}

const DISCRIMINATOR_LENGTH: usize = 8;
const VIBE_LENGTH: usize = 32;
const LIKER_LENGTH: usize = 32;
const BUMP_LENGTH: usize = 1;


impl Like {
    // Length of a Like
    pub const LEN: usize = DISCRIMINATOR_LENGTH +
                VIBE_LENGTH +
                LIKER_LENGTH +
                BUMP_LENGTH;

    pub fn add_like(
        &mut self, 
        vibe: Pubkey,
        liker: Pubkey,
        bump: u8,
    ) {
        self.vibe = vibe;
        self. liker = liker;
        self.bump = bump;
    }
}