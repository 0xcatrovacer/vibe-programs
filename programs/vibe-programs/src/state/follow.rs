use anchor_lang::prelude::*;

#[account]
pub struct Follow {

    // The user being followed
    pub followed: Pubkey,

    // The user who is the follower. Also, the Signer of the Instruction
    pub follower: Pubkey,

    // Bump
    pub bump: u8,
}

const DISCRIMINATOR_LENGTH: usize = 8;
const FOLLOWER_USER_PUBLIC_KEY_LENGTH: usize = 32;
const FOLLOWED_USER_PUBLIC_KEY_LENGTH: usize = 32;
const FOLLOW_BUMP: usize = 1;

impl Follow {

    //Length of Follow
    pub const LEN: usize = DISCRIMINATOR_LENGTH
        + FOLLOWED_USER_PUBLIC_KEY_LENGTH
        + FOLLOWER_USER_PUBLIC_KEY_LENGTH
        + FOLLOW_BUMP;
}