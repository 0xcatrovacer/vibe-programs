use anchor_lang::prelude::*;

#[account]
pub struct Follow {

    // The user being followed
    pub followed: Pubkey,

    // The user who is following. Also, the Signer of the Instruction
    pub following: Pubkey,

    // Bump
    pub bump: u8,
}

const DISCRIMINATOR_LENGTH: usize = 8;
const FOLLOWER_USER_PUBLIC_KEY_LENGTH: usize = 32;
const FOLLOWED_USER_PUBLIC_KEY_LENGTH: usize = 32;
const FOLLOW_BUMP: usize = 2;

impl Follow {

    //Length of Follow
    pub const LEN: usize = DISCRIMINATOR_LENGTH
        + FOLLOWED_USER_PUBLIC_KEY_LENGTH
        + FOLLOWER_USER_PUBLIC_KEY_LENGTH
        + FOLLOW_BUMP;
}