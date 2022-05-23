use anchor_lang::prelude::*;

mod state;
mod instructions;

use state::*;
use instructions::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod vibe_programs {
    use super::*;

    // Initialize a Vibe
    pub fn init_vibe(
        ctx: Context<InitializeVibe>, 
        vibe_content: Vec<VibeContent>, 
        allowed_comments: bool
    ) -> Result<()> {
        instructions::init_vibe::handler(ctx, vibe_content, allowed_comments)
    }

    // Add a Comment
    pub fn add_comment(
        ctx: Context<AddComment>,
        content: String,
        bump: u8,
    ) -> Result<()> {
        instructions::add_comment::handler(ctx, content, bump)
    }

    // Remove a Comment
    pub fn remove_comment(ctx: Context<RemoveComment>) -> Result<()> {
        instructions::remove_comment::handler(ctx)
    }
}
