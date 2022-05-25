use anchor_lang::prelude::*;

mod state;
mod instructions;
mod errors;

use instructions::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod vibe_programs {
    use super::*;

    // Create an User
    pub fn init_user(
        ctx: Context<InitializeUser>,
        nick: String
    ) -> Result<()> {
        instructions::init_user::handler(ctx, nick)
    }

    // Initialize a Vibe
    pub fn init_vibe(
        ctx: Context<InitializeVibe>, 
        vibe_title: String, 
        vibe_content: String, 
        allowed_comments: bool
    ) -> Result<()> {
        instructions::init_vibe::handler(ctx, vibe_title, vibe_content, allowed_comments)
    }

    // Delete a Vibe
    pub fn remove_vibe(ctx: Context<RemoveVibe>) -> Result<()> {
        instructions::remove_vibe::handler(ctx)
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

    // Follow an User
    pub fn follow(ctx: Context<FollowUser>) -> Result<()> {
        instructions::follow_user::handler(ctx)
    }

    // Unfollow an User
    pub fn unfollow(ctx: Context<RemoveFollow>) -> Result<()> {
        instructions::remove_follow::handler(ctx)
    }
}
