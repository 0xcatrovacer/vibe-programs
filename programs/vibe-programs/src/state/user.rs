use anchor_lang::prelude::*;

#[account]
pub struct User {

    // Pubkey of user
    pub user_key: Pubkey,

    // Nick of user
    pub nick: String,

    // Followers
    pub followers: u32,

    // Followed
    pub followings: u32,

    // Bump
    pub bump: u8,
}

const DISCRIMINATOR_LENGTH: usize = 8;
const NICK_LENGTH: usize = 20 * 4;
const FOLLOWERS_LENGTH: usize = 4;
const FOLLOWINGS_LENGTH: usize = 4;
const BUMP_LENGTH: usize = 1;

impl User {
    pub const LEN: usize = DISCRIMINATOR_LENGTH
            + NICK_LENGTH
            + FOLLOWERS_LENGTH
            + FOLLOWINGS_LENGTH
            + BUMP_LENGTH;
}