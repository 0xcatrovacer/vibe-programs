use anchor_lang::prelude::*;

mod state;
mod instructions;
mod errors;

use state::*;
use instructions::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod vibe_programs {
    use super::*;

    // Initialize a Vibe
    pub fn init_vibe(
        ctx: Context<InitializeVibe>, 
        vibe_title: String, 
        vibe_content: String, 
        allowed_comments: bool
    ) -> Result<()> {
        instructions::init_vibe::handler(ctx, vibe_title, vibe_content, allowed_comments)
    }

    // Add a Comment
    pub fn add_comment(
        ctx: Context<AddComment>,
        content: String,
    ) -> Result<()> {
        instructions::add_comment::handler(ctx, content)
    }

    // Remove a Comment
    pub fn remove_comment(ctx: Context<RemoveComment>) -> Result<()> {
        instructions::remove_comment::handler(ctx)
    }

    // Add a Like
    pub fn add_like(ctx: Context<AddLike>) -> Result<()> {
        instructions::add_like::handler(ctx)
    }

    // Remove a Like
    pub fn remove_like(ctx: Context<RemoveLike>) -> Result<()> {
        instructions::remove_like::handler(ctx)
    }
}
