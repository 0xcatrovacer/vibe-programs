use anchor_lang::prelude::*;

#[account]
pub struct Vibe {

    pub version: u32,

    // Content of the Vibe
    pub vibe_content: Vec<VibeContent>,

    // Author of the Vibe
    pub author: Pubkey,
    
    // Number of Likes
    pub likes: u32,
    
    // If comments are allowed or not
    pub allowed_comments: bool,

    // Number of Comments
    pub comments: u32,

    // Timestamp when vibe was created
    pub timestamp: i64,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct VibeContent {
    // Vibe Title
    pub vibe_t: String,

    //Vibe Paragraph
    pub vibe_p: String,
}

impl Vibe {
    pub const LEN: usize = DISCRIMINATOR_LENGTH
                + VERSION
                + CONTENT_TITLE
                + CONTENT_PARA
                + AUTHOR
                + LIKES
                + ALLCOMMENTS
                + COMMENTS
                + TIMESTAMPS;
}

const DISCRIMINATOR_LENGTH: usize = 8;
const VERSION: usize = 4;
const CONTENT_TITLE: usize = 50 * 4;
const CONTENT_PARA: usize = 300 * 4;
const AUTHOR: usize = 32;
const LIKES: usize = 4;
const ALLCOMMENTS: usize = 1;
const COMMENTS: usize = 4;
const TIMESTAMPS: usize = 8;


