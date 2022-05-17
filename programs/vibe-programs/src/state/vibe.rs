use anchor_lang::prelude::*;

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
